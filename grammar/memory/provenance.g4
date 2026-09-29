/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/provenance.g4
 *
 * Grammar:
 *     MemoryProvenance
 *
 * Status:
 *     PRODUCTION MEMORY-DOMAIN LEAF GRAMMAR
 *
 * Purpose:
 *     Define source-level provenance and lineage intent for MEMORY SEMANTICS.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * Primary architectural objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     POCO-REAF
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * Memory provenance answers questions such as:
 *
 *     - Where did this logical memory state originate?
 *     - From which logical memory object was it derived?
 *     - Which logical memory operation transformed it?
 *     - Which ownership/lifetime transition produced it?
 *     - Which logical region or memory space was involved?
 *     - Which checkpoint, persistence event, or migration is related to it?
 *     - Which declared memory capability or policy participated?
 *     - Which provenance metadata is associated with the memory state?
 *
 * This is SOURCE-LEVEL MEMORY SEMANTIC INTENT.
 *
 * It is NOT a physical-memory tracing implementation.
 *
 * ============================================================================
 * 2. SINGLE-OWNER RULE
 * ============================================================================
 *
 * This file owns MEMORY-SPECIFIC provenance.
 *
 * It does NOT become the owner of general provenance.
 *
 * General data provenance:
 *
 *     grammar/data/provenance.g4
 *
 * Security provenance:
 *
 *     grammar/security/provenance.g4
 *
 * Compilation provenance:
 *
 *     grammar/compile/provenance.g4
 *
 * Memory persistence:
 *
 *     grammar/memory/persistence.g4
 *
 * Memory ownership:
 *
 *     grammar/memory/ownership.g4
 *
 * Memory regions:
 *
 *     grammar/memory/regions.g4
 *
 * Memory constraints:
 *
 *     grammar/memory/memory-constraints.g4
 *
 * Memory capabilities:
 *
 *     grammar/memory/memory-capabilities.g4
 *
 * This file may REFER to concepts owned by those files, but MUST NOT
 * reimplement their grammars.
 *
 * ============================================================================
 * 3. WHAT THIS FILE OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     memory provenance declarations
 *     memory provenance attachments
 *     memory provenance relationships
 *     memory derivation relationships
 *     memory origin relationships
 *     memory production relationships
 *     memory consumption relationships
 *     memory transformation relationships
 *     memory allocation lineage
 *     memory deallocation lineage
 *     memory ownership-transition lineage
 *     memory borrow lineage
 *     memory lifetime lineage
 *     memory region lineage
 *     memory-space lineage
 *     memory persistence lineage
 *     memory checkpoint lineage
 *     memory migration lineage
 *     memory recovery lineage
 *     memory provenance identity
 *     memory provenance version
 *     memory provenance scope
 *     memory provenance schema reference
 *     memory provenance properties
 *     memory provenance requirements
 *     memory provenance constraints
 *     memory provenance preferences
 *     memory provenance hints
 *
 * ============================================================================
 * 4. WHAT THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical definitions
 *     identifiers
 *     qualified-name syntax
 *     expression syntax
 *     type syntax
 *     ownership checking
 *     borrow checking
 *     lifetime inference
 *     allocation
 *     deallocation implementation
 *     persistence implementation
 *     serialization
 *     data provenance
 *     security provenance
 *     compilation provenance
 *     resource discovery
 *     capability discovery
 *     hardware discovery
 *     physical memory addresses
 *     physical memory banks
 *     NUMA topology
 *     cache topology
 *     device selection
 *     accelerator selection
 *     quantum hardware
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime tracing
 *     provenance storage
 *     provenance databases
 *     audit logs
 *     cryptographic verification
 *     cryptographic signing
 *     canonical IR definitions
 *
 * ============================================================================
 * 5. ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * canonical lexer
 *   |
 *   v
 * MemoryProvenance
 *   |
 *   v
 * domain-neutral frontend AST
 *   |
 *   v
 * semantic memory analysis
 *   |
 *   +----------------------+-----------------------+
 *   |                      |                       |
 *   v                      v                       v
 * ownership/lifetime   resource/capability    persistence/security
 * analysis             analysis               analysis
 *   |                      |                       |
 *   +----------------------+-----------------------+
 *                          |
 *                          v
 *                  canonical semantic model
 *                          |
 *                          v
 *                         IR
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *          classical    quantum::ir   HDL/hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *               optimization / lowering
 *                          |
 *                    target realization
 *
 * This grammar does not introduce another IR.
 *
 * ============================================================================
 * 6. MEMORY PROVENANCE VS DATA PROVENANCE
 * ============================================================================
 *
 * DATA provenance answers:
 *
 *     "Where did this logical data value come from?"
 *
 * MEMORY provenance answers:
 *
 *     "What is the semantic history of this memory state/object/region/
 *      ownership/lifetime/persistence relationship?"
 *
 * Example:
 *
 *     data::provenance
 *
 * may describe that a tensor was derived from another tensor.
 *
 *     memory::provenance
 *
 * may describe that the memory object holding that tensor:
 *
 *     was allocated;
 *     entered a region;
 *     was borrowed;
 *     changed ownership;
 *     was checkpointed;
 *     was migrated;
 *     was restored.
 *
 * These semantic domains may be correlated downstream.
 *
 * They MUST NOT become duplicate parser grammars for the same concept.
 *
 * ============================================================================
 * 7. MEMORY PROVENANCE VS SECURITY PROVENANCE
 * ============================================================================
 *
 * Memory provenance may record:
 *
 *     security-related provenance references
 *
 * but does not authenticate them.
 *
 * Security provenance remains owned by:
 *
 *     grammar/security/provenance.g4
 *
 * This file therefore permits symbolic references such as:
 *
 *     security::provenance::integrity
 *     security::attestation
 *
 * as expressions/names.
 *
 * It does not implement:
 *
 *     signatures
 *     hashes
 *     attestations
 *     certificates
 *     trust verification
 *
 * ============================================================================
 * 8. MEMORY PROVENANCE VS COMPILATION PROVENANCE
 * ============================================================================
 *
 * Compilation provenance records:
 *
 *     source -> compilation -> artifact
 *
 * Memory provenance records:
 *
 *     memory state -> memory semantic transition -> memory state
 *
 * They may be connected by semantic analysis.
 *
 * This grammar MUST NOT duplicate:
 *
 *     grammar/compile/provenance.g4
 *
 * ============================================================================
 * 9. POCO-REAF
 * ============================================================================
 *
 * Memory provenance must remain portable across:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPU/quantum-classical systems
 *     simulators
 *     HPC systems
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * The source describes semantic memory provenance.
 *
 * It does not describe the physical implementation.
 *
 * ============================================================================
 * 10. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT impose:
 *
 *     MAX_MEMORY
 *     MAX_MEMORY_OBJECTS
 *     MAX_REGIONS
 *     MAX_ALLOCATIONS
 *     MAX_REFERENCES
 *     MAX_LIFETIMES
 *     MAX_PROVENANCE_RECORDS
 *     MAX_PROVENANCE_EDGES
 *     MAX_CHECKPOINTS
 *     MAX_SNAPSHOTS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_ADDRESS_BITS
 *
 * It MUST NOT encode:
 *
 *     32-bit addresses
 *     64-bit addresses
 *     fixed memory capacities
 *     fixed region counts
 *     fixed allocation counts
 *     fixed object counts
 *     fixed checkpoint counts
 *
 * Numeric values remain valid program semantics when explicitly supplied by
 * the program.
 *
 * ============================================================================
 * 11. OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Provenance relationship names and properties are open-world.
 *
 * The grammar provides common structural relationships, but future memory
 * technologies must be representable without modifying this file merely
 * because a new semantic technology exists.
 *
 * Examples of possible semantic names include:
 *
 *     memory::volatile
 *     memory::persistent
 *     memory::shared
 *     memory::distributed
 *     memory::accelerator
 *     memory::remote
 *     memory::managed
 *     memory::unified
 *     memory::transactional
 *     memory::checkpoint
 *     memory::migration
 *     memory::future::technology
 *     vendor::memory::extension
 *
 * The parser does not decide whether such names are semantically valid.
 *
 * ============================================================================
 * 12. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These categories MUST remain distinct.
 *
 * Requirement:
 *
 *     requires expression;
 *
 * Constraint:
 *
 *     constraint expression;
 *
 * Preference:
 *
 *     prefer expression;
 *
 * Hint:
 *
 *     hint expression;
 *
 * A requirement is mandatory semantic intent.
 *
 * A constraint restricts valid realization.
 *
 * A preference is non-mandatory guidance.
 *
 * A hint is implementation guidance without mandatory semantics.
 *
 * None of them selects a physical device.
 *
 * ============================================================================
 * 13. DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only upon:
 *
 *     source token stream
 *     grammar version
 *     canonical parser composition
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend upon:
 *
 *     available memory
 *     current machine
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     runtime state
 *     wall-clock time
 *     randomness
 *     environment variables
 *     hardware discovery
 *
 * ============================================================================
 * 14. SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no secret access
 *     no command execution
 *     no runtime execution
 *
 * A parsed provenance declaration does not grant:
 *
 *     memory access
 *     filesystem access
 *     device access
 *     security authority
 *     provenance signing authority
 *
 * ============================================================================
 * 15. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Downstream AST/semantic structures must preserve source spans for:
 *
 *     declaration
 *     subject
 *     relationship
 *     related expression
 *     property
 *     identity
 *     version
 *     schema
 *     scope
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * This is required for:
 *
 *     diagnostics
 *     IDE tooling
 *     provenance explanation
 *     source mapping
 *     refactoring
 *     compatibility tooling
 *
 * ============================================================================
 * 16. AST CONTRACT
 * ============================================================================
 *
 * The grammar should map to domain-neutral structures conceptually equivalent
 * to:
 *
 *     MemoryProvenanceDeclaration
 *         name
 *         subject
 *         members
 *         source_span
 *
 *     MemoryProvenanceAttachment
 *         subject
 *         members
 *         source_span
 *
 *     MemoryProvenanceRelation
 *         relation_kind
 *         subjects[]
 *         source_span
 *
 *     MemoryProvenanceProperty
 *         qualified_name
 *         value
 *         source_span
 *
 *     MemoryProvenanceContract
 *         requirements[]
 *         constraints[]
 *         preferences[]
 *         hints[]
 *         source_span
 *
 * Exact Rust AST type names remain owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar must not depend on those Rust types.
 *
 * ============================================================================
 * 17. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether a memory subject exists;
 *     whether a relationship is meaningful;
 *     whether a memory object is valid;
 *     whether an allocation relationship is valid;
 *     whether an ownership relationship is valid;
 *     whether a borrow relationship is valid;
 *     whether a lifetime relationship is valid;
 *     whether a region relationship is valid;
 *     whether a persistence relationship is valid;
 *     whether a capability requirement is satisfiable;
 *     whether a constraint is satisfiable;
 *     whether a preference is applicable;
 *     whether a hint is meaningful;
 *     whether a provenance identity is valid;
 *     whether a provenance schema is compatible.
 *
 * None of these decisions occur in the grammar.
 *
 * ============================================================================
 * 18. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO:
 *
 *     MemoryProvenanceIR
 *     MemoryLineageIR
 *     MemoryHistoryIR
 *     QuantumMemoryProvenanceIR
 *
 * Provenance lowers through the repository's canonical semantic/IR pipeline.
 *
 * Memory semantics may accompany:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representations
 *     distributed semantic representations
 *     execution metadata
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * 19. GENERAL MEMORY INTEGRATION
 * ============================================================================
 *
 * The grammar consumes universal:
 *
 *     qualifiedName
 *     expression
 *
 * syntax.
 *
 * It deliberately does not redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *
 * Memory-specific semantic interpretation is downstream.
 *
 * ============================================================================
 * 20. MEMORY OBJECT SUBJECTS
 * ============================================================================
 *
 * A provenance subject is a general expression.
 *
 * This permits provenance for:
 *
 *     memory objects
 *     buffers
 *     arrays
 *     tensors
 *     regions
 *     references
 *     shared state
 *     distributed state
 *     persistent state
 *     accelerator-visible memory
 *     quantum-classical state
 *     temporary state
 *     user-defined memory abstractions
 *
 * No physical address is required.
 *
 * ============================================================================
 * 21. PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * `memoryProvenanceUnit`
 *     Standalone grammar-conformance entry point.
 *
 * `memoryProvenanceConstruct`
 *     Generic reusable memory-provenance construct.
 *
 * `memoryProvenanceDeclaration`
 *     Named memory-provenance declaration.
 *
 * `memoryProvenanceAttachStatement`
 *     Attach provenance to an existing logical memory subject.
 *
 * `memoryProvenanceExpression`
 *     Expression-position integration point.
 *
 * ============================================================================
 */

parser grammar MemoryProvenance;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Expressions;


/*
 * ============================================================================
 * 22. STANDALONE UNIT
 * ============================================================================
 */

memoryProvenanceUnit
    : memoryProvenanceConstruct* EOF
    ;


/*
 * ============================================================================
 * 23. GENERIC CONSTRUCT
 * ============================================================================
 */

memoryProvenanceConstruct
    : memoryProvenanceDeclaration
    | memoryProvenanceAttachStatement
    | memoryProvenanceExpression
    | memoryProvenanceContract
    ;


/*
 * ============================================================================
 * 24. NAMED MEMORY-PROVENANCE DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     provenance memory_state for value {
 *         origin: allocation;
 *         derived_from: source;
 *     }
 *
 * The semantic layer determines whether the declaration name and subject are
 * valid memory entities.
 *
 * `PROVENANCE` remains the canonical provenance introducer already used by
 * the repository's provenance grammars.
 *
 * The explicit `MEMORY` marker makes the ownership domain unambiguous and
 * prevents this grammar from competing with general data provenance.
 */

memoryProvenanceDeclaration
    : PROVENANCE MEMORY qualifiedName
      FOR expression
      memoryProvenanceBody
    ;


/*
 * ============================================================================
 * 25. ATTACHMENT FORM
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     provenance memory value with {
 *         derived_from: source;
 *     };
 *
 * The `MEMORY` marker establishes that this is memory provenance rather than
 * general data provenance.
 */

memoryProvenanceAttachStatement
    : PROVENANCE MEMORY expression
      WITH
      memoryProvenanceBody
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. EXPRESSION FORM
 * ============================================================================
 *
 * This form permits memory provenance to participate in larger memory
 * expressions without creating a second expression language.
 *
 * The semantic layer determines whether the resulting provenance expression
 * is usable in a particular context.
 */

memoryProvenanceExpression
    : PROVENANCE MEMORY
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 27. BODY
 * ============================================================================
 */

memoryProvenanceBody
    : LBRACE
      memoryProvenanceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 28. MEMBERS
 * ============================================================================
 */

memoryProvenanceMember
    : memoryProvenanceOrigin
    | memoryProvenanceDerivedFrom
    | memoryProvenanceProducedBy
    | memoryProvenanceConsumedBy
    | memoryProvenanceTransformedBy
    | memoryProvenanceAllocatedBy
    | memoryProvenanceReleasedBy
    | memoryProvenanceOwnedBy
    | memoryProvenanceBorrowedFrom
    | memoryProvenanceLifetime
    | memoryProvenanceRegion
    | memoryProvenanceSpace
    | memoryProvenancePersistedBy
    | memoryProvenanceCheckpointedBy
    | memoryProvenanceMigratedBy
    | memoryProvenanceRecoveredBy
    | memoryProvenanceIdentity
    | memoryProvenanceVersion
    | memoryProvenanceSchema
    | memoryProvenanceScope
    | memoryProvenanceContractClause
    | memoryProvenanceProperty
    ;


/*
 * ============================================================================
 * 29. COMMON RELATIONSHIP SHAPE
 * ============================================================================
 *
 * All relationships use expressions.
 *
 * This is intentional.
 *
 * A relationship may therefore refer to:
 *
 *     a variable
 *     a memory object
 *     a region
 *     a function
 *     a symbolic operation
 *     a resource
 *     a capability
 *     a persistence operation
 *     a future semantic entity
 *
 * The parser does not resolve any of them.
 */


/*
 * ============================================================================
 * 30. ORIGIN
 * ============================================================================
 */

memoryProvenanceOrigin
    : ORIGIN COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 31. DERIVATION
 * ============================================================================
 */

memoryProvenanceDerivedFrom
    : DERIVED_FROM COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 32. PRODUCER
 * ============================================================================
 */

memoryProvenanceProducedBy
    : PRODUCED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 33. CONSUMER
 * ============================================================================
 */

memoryProvenanceConsumedBy
    : CONSUMED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 34. TRANSFORMATION
 * ============================================================================
 */

memoryProvenanceTransformedBy
    : TRANSFORMED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 35. ALLOCATION LINEAGE
 * ============================================================================
 *
 * This records the logical operation that produced the memory allocation.
 *
 * It does not identify:
 *
 *     allocator implementation
 *     heap
 *     stack
 *     page
 *     physical address
 *     memory bank
 *     NUMA node
 *     device
 */

memoryProvenanceAllocatedBy
    : ALLOCATED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 36. RELEASE / DEALLOCATION LINEAGE
 * ============================================================================
 */

memoryProvenanceReleasedBy
    : RELEASED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 37. OWNERSHIP LINEAGE
 * ============================================================================
 *
 * This records the logical owner relationship.
 *
 * Ownership validity remains owned by ownership semantics.
 */

memoryProvenanceOwnedBy
    : OWNED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 38. BORROW LINEAGE
 * ============================================================================
 */

memoryProvenanceBorrowedFrom
    : BORROWED_FROM COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 39. LIFETIME LINEAGE
 * ============================================================================
 *
 * The value is an expression so lifetimes may be symbolic and computed.
 *
 * The grammar does not interpret the value as physical time.
 */

memoryProvenanceLifetime
    : LIFETIME COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 40. REGION LINEAGE
 * ============================================================================
 *
 * The region remains an abstract semantic region.
 *
 * Physical placement is downstream.
 */

memoryProvenanceRegion
    : REGION COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 41. MEMORY-SPACE LINEAGE
 * ============================================================================
 *
 * A memory space may be:
 *
 *     local
 *     shared
 *     remote
 *     persistent
 *     distributed
 *     accelerator
 *     unified
 *     managed
 *     custom
 *
 * The grammar does not enumerate those values.
 */

memoryProvenanceSpace
    : SPACE COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 42. PERSISTENCE LINEAGE
 * ============================================================================
 *
 * Persistence implementation remains owned by persistence.g4 and downstream
 * runtime/storage systems.
 */

memoryProvenancePersistedBy
    : PERSISTED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 43. CHECKPOINT LINEAGE
 * ============================================================================
 */

memoryProvenanceCheckpointedBy
    : CHECKPOINTED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 44. MIGRATION LINEAGE
 * ============================================================================
 *
 * Migration is semantic state migration.
 *
 * This grammar does not specify:
 *
 *     source node
 *     destination node
 *     network path
 *     memory controller
 *     device
 *     storage technology
 */

memoryProvenanceMigratedBy
    : MIGRATED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 45. RECOVERY LINEAGE
 * ============================================================================
 */

memoryProvenanceRecoveredBy
    : RECOVERED_BY COLON
      memoryProvenanceExpressionList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 46. IDENTITY
 * ============================================================================
 *
 * Identity is an arbitrary semantic expression.
 *
 * It may represent:
 *
 *     symbolic identity
 *     content identity
 *     external identity
 *     versioned identity
 *     application-defined identity
 *
 * Cryptographic identity remains a security concern.
 */

memoryProvenanceIdentity
    : IDENTITY COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 47. VERSION
 * ============================================================================
 */

memoryProvenanceVersion
    : VERSION COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 48. SCHEMA
 * ============================================================================
 *
 * Schema ownership remains outside this grammar.
 */

memoryProvenanceSchema
    : SCHEMA COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 49. SCOPE
 * ============================================================================
 *
 * Scope is semantic provenance scope.
 *
 * It is not a filesystem scope or hardware topology.
 */

memoryProvenanceScope
    : SCOPE COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 50. CONTRACT
 * ============================================================================
 */

memoryProvenanceContractClause
    : memoryProvenanceRequirement
    | memoryProvenanceConstraint
    | memoryProvenancePreference
    | memoryProvenanceHint
    ;


/*
 * ============================================================================
 * 51. REQUIREMENT
 * ============================================================================
 */

memoryProvenanceRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 52. CONSTRAINT
 * ============================================================================
 */

memoryProvenanceConstraint
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 53. PREFERENCE
 * ============================================================================
 */

memoryProvenancePreference
    : PREFER
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 54. HINT
 * ============================================================================
 */

memoryProvenanceHint
    : HINT
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 55. EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * Property names are qualified names rather than a closed enumeration.
 *
 * This allows future memory provenance semantics without requiring a lexer
 * change for every new property.
 */

memoryProvenanceProperty
    : qualifiedName
      COLON
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 56. EXPRESSION LIST
 * ============================================================================
 */

memoryProvenanceExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 57. AST NORMALIZATION CONTRACT
 * ============================================================================
 *
 * The parser preserves source structure.
 *
 * Semantic analysis may normalize:
 *
 *     allocated_by
 *     released_by
 *     owned_by
 *     borrowed_from
 *     region
 *     space
 *     persisted_by
 *     checkpointed_by
 *     migrated_by
 *     recovered_by
 *
 * into canonical memory provenance relationships.
 *
 * The parser MUST NOT perform that normalization.
 */


/*
 * ============================================================================
 * 58. OWNERSHIP/LIFETIME INTEGRATION
 * ============================================================================
 *
 * Ownership semantics remain owned by:
 *
 *     grammar/memory/ownership.g4
 *
 * Borrowing remains owned by:
 *
 *     grammar/memory/borrowing.g4
 *
 * Lifetimes remain owned by:
 *
 *     grammar/memory/lifetimes.g4
 *
 * This grammar records provenance relationships involving those constructs.
 *
 * It does not validate them.
 */


/*
 * ============================================================================
 * 59. REGION INTEGRATION
 * ============================================================================
 *
 * Region semantics remain owned by:
 *
 *     grammar/memory/regions.g4
 *
 * A provenance region relationship records semantic association with a region.
 *
 * It does not select:
 *
 *     NUMA nodes
 *     memory banks
 *     pages
 *     physical addresses
 *     cache levels
 */


/*
 * ============================================================================
 * 60. ADDRESS-SPACE INTEGRATION
 * ============================================================================
 *
 * Address-space semantics remain owned by:
 *
 *     grammar/memory/address-spaces.g4
 *
 * Provenance may record a logical space expression.
 *
 * It does not encode address width or physical address layout.
 */


/*
 * ============================================================================
 * 61. PERSISTENCE INTEGRATION
 * ============================================================================
 *
 * Persistence semantics remain owned by:
 *
 *     grammar/memory/persistence.g4
 *
 * This grammar only records logical relationships to persistence events.
 *
 * For example:
 *
 *     persisted_by: checkpoint;
 *
 * does not choose:
 *
 *     disk
 *     NVRAM
 *     object storage
 *     database
 *     device
 *     node
 *
 * Those decisions remain downstream.
 */


/*
 * ============================================================================
 * 62. DISTRIBUTED-MEMORY INTEGRATION
 * ============================================================================
 *
 * Distributed memory remains owned by:
 *
 *     grammar/memory/distributed-memory.g4
 *     grammar/distributed/
 *
 * Provenance may describe migration, replication, partitioning, or recovery
 * through expressions.
 *
 * No node topology is encoded here.
 */


/*
 * ============================================================================
 * 63. ACCELERATOR-MEMORY INTEGRATION
 * ============================================================================
 *
 * Accelerator memory remains owned by:
 *
 *     grammar/memory/accelerator-memory.g4
 *
 * Provenance may refer to an accelerator memory semantic object.
 *
 * It does not select:
 *
 *     GPU 0
 *     accelerator 0
 *     device 0
 *
 * or any physical memory bank.
 */


/*
 * ============================================================================
 * 64. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Memory provenance may accompany quantum-classical computation.
 *
 * Examples include:
 *
 *     provenance memory quantum_state for state {
 *         origin: initialization;
 *         region: quantum::logical_state;
 *         property: quantum::measurement_context;
 *     }
 *
 * The grammar does not create a quantum memory IR.
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN
 *          |
 *          v
 *     HAL
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QPU topology
 *     calibration
 *     QEC codes
 *
 */


/*
 * ============================================================================
 * 65. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical memory provenance may accompany:
 *
 *     arrays
 *     matrices
 *     tensors
 *     buffers
 *     variables
 *     execution state
 *     scientific data
 *     accelerator staging
 *
 * The grammar remains independent of the final classical representation.
 */


/*
 * ============================================================================
 * 66. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware/co-design memory provenance may refer to:
 *
 *     logical memory resources
 *     generated memories
 *     HDL memory intent
 *     verification state
 *     synthesis artifacts
 *
 * It does not define:
 *
 *     register width
 *     memory-bank count
 *     physical SRAM size
 *     physical DRAM size
 *     FPGA block count
 *     ASIC topology
 *
 */


/*
 * ============================================================================
 * 67. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Requirements may use the existing resource/capability expression model.
 *
 * Examples:
 *
 *     requires capability("memory.provenance");
 *
 *     requires capability("memory.persistence");
 *
 *     requires capability("memory.lineage");
 *
 * These are expressions.
 *
 * The grammar does not decide whether a target provides them.
 */


/*
 * ============================================================================
 * 68. DATA PROVENANCE INTEGRATION
 * ============================================================================
 *
 * If a memory object contains logical data, data provenance may be attached
 * downstream.
 *
 * This grammar does not redefine:
 *
 *     data::provenance
 *
 * A semantic relationship may connect:
 *
 *     MemoryProvenance
 *          |
 *          +--> DataProvenance
 *
 * without making the grammars identical.
 */


/*
 * ============================================================================
 * 69. SECURITY PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Security provenance remains owned by:
 *
 *     grammar/security/provenance.g4
 *
 * Memory provenance may reference security requirements using expressions.
 *
 * Example:
 *
 *     requires capability("provenance.integrity");
 *
 * The grammar performs no authentication or authorization.
 */


/*
 * ============================================================================
 * 70. INTEROPERABILITY
 * ============================================================================
 *
 * Memory provenance may eventually be mapped to external provenance formats.
 *
 * Such mapping belongs to:
 *
 *     grammar/interoperability/
 *
 * and the compiler/interoperability implementation.
 *
 * This grammar does not encode:
 *
 *     W3C PROV
 *     OpenTelemetry
 *     vendor trace formats
 *     database schemas
 *
 * as the canonical Zamani syntax.
 *
 */


/*
 * ============================================================================
 * 71. DIALECT EXTENSION
 * ============================================================================
 *
 * Dialects may extend memory provenance using qualified properties.
 *
 * Example:
 *
 *     vendor::memory::property: expression;
 *
 * A dialect must declare:
 *
 *     namespace
 *     version
 *     semantic meaning
 *     AST mapping
 *     compatibility
 *
 * Dialects MUST NOT silently redefine standard relationships.
 */


/*
 * ============================================================================
 * 72. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover malformed structure.
 *
 * Examples:
 *
 *     provenance
 *     provenance memory
 *     provenance memory name
 *     provenance memory name for
 *     provenance memory name for value {
 *         origin:
 *     }
 *
 * Semantic diagnostics cover:
 *
 *     unresolved memory subject
 *     invalid memory relationship
 *     invalid ownership relationship
 *     invalid lifetime relationship
 *     invalid region reference
 *     invalid memory-space reference
 *     unsatisfied capability
 *     unsatisfied persistence requirement
 *     invalid provenance schema
 *     incompatible provenance version
 *
 * Resource diagnostics cover:
 *
 *     unavailable capability
 *     insufficient resources
 *     unsupported target realization
 *
 * Runtime diagnostics cover:
 *
 *     unavailable storage
 *     unavailable device
 *     runtime provenance failure
 *
 * The parser must not manufacture downstream diagnostics.
 */


/*
 * ============================================================================
 * 73. PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     expression recursion
 *
 * rather than finite cardinality alternatives.
 *
 * There is no language-level maximum for:
 *
 *     provenance declarations
 *     relationships
 *     properties
 *     expressions
 *     memory objects
 *     regions
 *     memory spaces
 *     checkpoints
 *     migrations
 *     provenance chain length
 *
 * Actual parser memory/stack/time exhaustion is an implementation-resource
 * condition and MUST NOT be converted into a language semantic maximum.
 */


/*
 * ============================================================================
 * 74. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden physical identities:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     device0
 *     memory_bank0
 *     address0
 *
 * are not grammar constructs.
 *
 * A programmer may still use an arbitrary identifier containing such text as
 * ordinary program data where the canonical lexical grammar permits it.
 *
 * The prohibition is against assigning universal physical semantics to those
 * names.
 */


/*
 * ============================================================================
 * 75. TEST CONTRACT — POSITIVE
 * ============================================================================
 *
 * Required structural examples:
 *
 *     provenance memory state for buffer {
 *         origin: allocation;
 *     }
 *
 *     provenance memory state for buffer {
 *         derived_from: source;
 *         allocated_by: allocator;
 *         region: region::working;
 *         space: memory::local;
 *     }
 *
 *     provenance memory state for buffer {
 *         owned_by: owner;
 *         borrowed_from: source;
 *         lifetime: 'a;
 *     }
 *
 *     provenance memory state for buffer {
 *         persisted_by: checkpoint;
 *         checkpointed_by: snapshot;
 *         recovered_by: restore;
 *     }
 *
 *     provenance memory state for buffer {
 *         migrated_by: migration;
 *         requires capability("memory.lineage");
 *         constraint provenance::complete;
 *         prefer provenance::portable;
 *         hint provenance::cacheable;
 *     }
 *
 * Attachment:
 *
 *     provenance memory buffer with {
 *         origin: allocation;
 *         region: region::working;
 *     };
 *
 * Expression:
 *
 *     provenance memory(buffer)
 *
 * Open-world properties:
 *
 *     provenance memory state for buffer {
 *         vendor::memory::lineage: metadata;
 *         future::memory::property: value;
 *     }
 */


/*
 * ============================================================================
 * 76. TEST CONTRACT — NEGATIVE
 * ============================================================================
 *
 * Must reject malformed syntax:
 *
 *     provenance
 *
 *     provenance memory
 *
 *     provenance memory name
 *
 *     provenance memory name for
 *
 *     provenance memory name for value
 *
 *     provenance memory name for value {
 *         origin:
 *     }
 *
 *     provenance memory name for value {
 *         : value;
 *     }
 *
 *     provenance memory value with
 *
 *     provenance memory value with {
 *         origin:
 *     };
 *
 *     provenance memory(value
 *
 *     provenance memory()
 *
 *     provenance memory state for buffer {
 *         requires;
 *     }
 */


/*
 * ============================================================================
 * 77. TEST CONTRACT — BOUNDARY
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     one relationship
 *     many relationships
 *     one property
 *     many properties
 *     deeply qualified names
 *     large expression lists
 *     deeply nested expressions
 *     empty provenance bodies
 *     long provenance chains
 *     symbolic quantities
 *     computed quantities
 *     large source files
 *     tiny source files
 *
 * Empty bodies are structurally legal:
 *
 *     provenance memory state for buffer {}
 *
 * Semantic validation decides whether an empty declaration is meaningful.
 */


/*
 * ============================================================================
 * 78. TEST CONTRACT — SCALABILITY
 * ============================================================================
 *
 * Generated tests MUST demonstrate that the grammar imposes no semantic limit
 * on:
 *
 *     memory-object count
 *     region count
 *     memory-space count
 *     relationship count
 *     provenance chain length
 *     checkpoint count
 *     migration count
 *     property count
 *     expression count
 *     source size
 *
 * Test sizes are implementation-resource tests, not language limits.
 */


/*
 * ============================================================================
 * 79. TEST CONTRACT — DETERMINISM
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     grammar version
 *     lexer vocabulary
 *     parser configuration
 *
 * must produce equivalent parse structures.
 *
 * Hardware availability MUST NOT alter parsing.
 */


/*
 * ============================================================================
 * 80. TEST CONTRACT — CROSS-DOMAIN
 * ============================================================================
 *
 * Required semantic integration coverage:
 *
 *     classical memory + provenance
 *     quantum-classical memory + provenance
 *     hybrid computation + provenance
 *     HDL memory + provenance
 *     hardware intent + provenance
 *     distributed memory + provenance
 *     accelerator memory + provenance
 *     persistence + provenance
 *     security provenance + memory provenance
 *     data provenance + memory provenance
 *     resource capability + memory provenance
 *
 * At least one end-to-end test must combine:
 *
 *     classical state
 *     quantum operation
 *     memory region
 *     persistence intent
 *     resource capability
 *     provenance
 *
 * without requiring a physical machine identifier.
 */


/*
 * ============================================================================
 * 81. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where AST formatting exists:
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
 *     formatter
 *       |
 *       v
 *     parser
 *
 * must preserve:
 *
 *     provenance domain
 *     declaration name
 *     subject
 *     relationship ordering where semantically relevant
 *     expressions
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     source intent
 */


/*
 * ============================================================================
 * 82. COMPATIBILITY
 * ============================================================================
 *
 * The stable public entry points are:
 *
 *     memoryProvenanceUnit
 *     memoryProvenanceConstruct
 *     memoryProvenanceDeclaration
 *     memoryProvenanceAttachStatement
 *     memoryProvenanceExpression
 *
 * Future extensions should first use:
 *
 *     qualified names
 *     expressions
 *     properties
 *
 * rather than introducing new reserved keywords.
 *
 * Existing data/security/compile provenance grammars remain semantically
 * distinct.
 *
 * ============================================================================
 * 83. RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated parser integration MUST:
 *
 *     compile on Rust 1.97;
 *     compile on Rust 1.97.1;
 *     use Rust 2021;
 *     use safe Rust;
 *     require no unsafe;
 *     preserve source spans;
 *     preserve deterministic parsing.
 *
 * ============================================================================
 * 84. COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] One canonical memory-provenance owner.
 * [x] Existing filenames preserved.
 * [x] General data provenance is not duplicated.
 * [x] Security provenance is not duplicated.
 * [x] Compilation provenance is not duplicated.
 * [x] Persistence is not reimplemented.
 * [x] Ownership is not reimplemented.
 * [x] Borrow checking is not reimplemented.
 * [x] Lifetime inference is not reimplemented.
 * [x] Region semantics are not reimplemented.
 * [x] Address-space semantics are not reimplemented.
 * [x] Resource discovery is not performed.
 * [x] Hardware discovery is not performed.
 * [x] Physical memory is not selected.
 * [x] No machine-size limits are encoded.
 * [x] No fixed memory capacity is encoded.
 * [x] No fixed address width is encoded.
 * [x] No fixed provenance cardinality is encoded.
 * [x] Open-world properties are supported.
 * [x] Requirements differ from constraints.
 * [x] Constraints differ from preferences.
 * [x] Preferences differ from hints.
 * [x] Expressions provide scalable values.
 * [x] Source spans are specified.
 * [x] AST ownership is specified.
 * [x] Semantic ownership is specified.
 * [x] IR ownership is specified.
 * [x] Quantum::ir remains canonical.
 * [x] Deterministic parsing is specified.
 * [x] Safe-Rust integration is specified.
 * [x] Positive tests are specified.
 * [x] Negative tests are specified.
 * [x] Boundary tests are specified.
 * [x] Scalability tests are specified.
 * [x] Cross-domain tests are specified.
 * [x] Round-trip tests are specified.
 * [x] Compatibility is specified.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What is the semantic provenance/history of this logical memory state?"
 *
 * It does NOT answer:
 *
 *     "Which physical memory contains it?"
 *
 *     "Which CPU owns it?"
 *
 *     "Which GPU owns it?"
 *
 *     "Which QPU owns it?"
 *
 *     "Which memory bank contains it?"
 *
 *     "Which NUMA node contains it?"
 *
 *     "Which physical address contains it?"
 *
 *     "Which allocator implements it?"
 *
 *     "Which storage device persists it?"
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 * END
 * ============================================================================
 */