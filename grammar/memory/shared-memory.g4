/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/shared-memory.g4
 *
 * Grammar:
 *     SharedMemory
 *
 * Status:
 *     Production shared-memory parser component.
 *
 * Language:
 *     Zamani Universal Computing Language
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL SHARED-MEMORY INTENT.
 *
 * Shared memory is a semantic relationship in which multiple execution
 * contexts may access the same logical memory object/place according to
 * explicitly represented access, visibility, consistency, synchronization,
 * resource, and policy requirements.
 *
 * This grammar does NOT define a physical shared-memory implementation.
 *
 * The same source construct may therefore be lowered to:
 *
 *     - ordinary host memory;
 *     - multicore shared memory;
 *     - NUMA memory;
 *     - unified memory;
 *     - accelerator-visible memory;
 *     - distributed shared memory;
 *     - transactional memory;
 *     - persistent shared memory;
 *     - heterogeneous memory;
 *     - future memory architectures.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------+----------------+----------------+
 *          |                |                |                |
 *          v                v                v                v
 *      ownership       concurrency       resources        effects
 *          |                |                |                |
 *          +----------------+----------------+----------------+
 *                           |
 *                           v
 *                 canonical semantic model
 *                           |
 *                           v
 *                    canonical IR layer
 *                           |
 *          +----------------+----------------+
 *          |                |                |
 *          v                v                v
 *      classical       quantum::ir       HDL/hardware
 *          |                |                |
 *          +----------------+----------------+
 *                           |
 *                           v
 *                optimization / lowering
 *                           |
 *                  routing / scheduling
 *                           |
 *                     target HAL
 *                           |
 *                    target realization
 *                           |
 *                         runtime
 *
 * This grammar participates only before semantic lowering.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - shared-memory semantic construct composition;
 *     - shared relationship intent;
 *     - unsharing intent;
 *     - shared access intent;
 *     - shared-memory visibility intent;
 *     - shared-memory consistency intent;
 *     - synchronization requirements attached to shared-memory intent;
 *     - shared-memory resource requirements;
 *     - shared-memory constraints;
 *     - shared-memory preferences;
 *     - shared-memory hints;
 *     - shared-memory metadata;
 *     - shared-memory annotations;
 *     - open-world shared-memory extension operations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - lexical vocabulary;
 *     - expressions;
 *     - types;
 *     - memory places;
 *     - generic memory operations;
 *     - allocation;
 *     - deallocation;
 *     - ownership checking;
 *     - borrowing;
 *     - lifetime checking;
 *     - memory regions;
 *     - address spaces;
 *     - distributed-memory semantics;
 *     - accelerator-memory semantics;
 *     - quantum-memory semantics;
 *     - concurrency primitives;
 *     - locks;
 *     - barriers;
 *     - channels;
 *     - scheduling;
 *     - resource discovery;
 *     - hardware discovery;
 *     - physical placement;
 *     - physical addresses;
 *     - topology;
 *     - compiler optimization;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - any IR.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical memory foundation:
 *
 *     grammar/memory/memory.g4
 *
 * This file reuses the canonical memory rules supplied by Memory, including:
 *
 *     memoryQualifiedName
 *     memoryPlace
 *     memoryArgumentList
 *     memoryArgument
 *     memoryNamedArgument
 *     memorySpecification
 *     memoryRequirement
 *     memoryConstraint
 *     memoryPreference
 *     memoryHint
 *     memoryPolicy
 *     memoryAnnotation
 *
 * This file MUST NOT redefine those concepts.
 *
 * Canonical lexical boundary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     ZamaniLexer
 *
 * not ZamaniTokens directly.
 *
 * ============================================================================
 * LEXICAL POLICY
 * ============================================================================
 *
 * NO NEW SHARED-MEMORY LEXER TOKENS ARE REQUIRED BY THIS FILE.
 *
 * In particular, this file deliberately does not introduce universal tokens
 * for:
 *
 *     shared
 *     share
 *     unshare
 *     coherent
 *     coherence
 *     visibility
 *     consistency
 *     synchronization
 *     read
 *     write
 *
 * Those names remain ordinary source-level names unless the canonical lexical
 * specification independently reserves them.
 *
 * This is intentional.
 *
 * Shared-memory technology evolves continuously. Reserving every technology,
 * protocol, memory model, vendor feature, or future concept as a lexer token
 * would make the language unnecessarily closed-world.
 *
 * Structural shared-memory constructs therefore use the canonical memory
 * qualified-name and operation architecture:
 *
 *     memory::shared(...)
 *     memory::share(...)
 *     memory::unshare(...)
 *     memory::access(...)
 *     memory::shared::domain::operation(...)
 *
 * Semantic analysis determines whether a qualified operation is a recognized
 * shared-memory operation.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal limit on:
 *
 *     shared objects
 *     shared places
 *     access operations
 *     participants
 *     execution contexts
 *     processes
 *     tasks
 *     threads
 *     cores
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     nodes
 *     memory capacity
 *     memory address width
 *     regions
 *     synchronization relationships
 *     consistency domains
 *     visibility domains
 *     topology
 *     devices
 *
 * The grammar MUST NOT contain language-level constants such as:
 *
 *     MAX_SHARED_MEMORY
 *     MAX_SHARED_OBJECTS
 *     MAX_SHARERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_REGIONS
 *
 * Any practical implementation limit belongs to the parser/compiler/runtime
 * implementation or target resource model, not the Zamani source language.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * "shared" does not automatically mean:
 *
 *     coherent
 *     atomic
 *     mutually exclusive
 *     sequentially consistent
 *     immediately visible
 *     physically co-located
 *     CPU memory
 *     RAM
 *     one machine
 *     one process
 *     one thread
 *     one device
 *
 * These are independent semantic properties.
 *
 * For example:
 *
 *     memory::shared(
 *         value,
 *         access = read,
 *         visibility = eventual,
 *         consistency = relaxed
 *     );
 *
 * expresses a semantic contract.
 *
 * It does not select:
 *
 *     cache protocol;
 *     NUMA node;
 *     memory bank;
 *     physical address;
 *     device;
 *     CPU;
 *     GPU;
 *     accelerator;
 *     network route.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts MUST remain distinct.
 *
 * REQUIREMENT:
 *
 *     mandatory semantic condition.
 *
 * CONSTRAINT:
 *
 *     restriction on the legal implementation space.
 *
 * PREFERENCE:
 *
 *     desired implementation characteristic that may be unavailable.
 *
 * HINT:
 *
 *     non-binding information useful to downstream implementation.
 *
 * This grammar preserves those distinctions structurally.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every shared-memory construct must preserve enough source information for
 * the domain-neutral frontend AST to represent:
 *
 *     - source span;
 *     - operation path;
 *     - target memory place;
 *     - operation arguments;
 *     - named arguments;
 *     - access intent;
 *     - visibility intent;
 *     - consistency intent;
 *     - synchronization intent;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - metadata;
 *     - annotation information.
 *
 * This grammar does NOT prescribe a Rust AST type name.
 *
 * Existing frontend AST architecture remains authoritative.
 *
 * In particular, shared-memory syntax should be representable through the
 * domain-neutral operation model rather than requiring a hardware-specific
 * SharedMemory IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the target is a valid memory place;
 *     - whether sharing is legal;
 *     - whether the object may be shared;
 *     - whether ownership permits sharing;
 *     - whether borrowing permits the requested access;
 *     - whether lifetimes remain valid;
 *     - whether access modes conflict;
 *     - whether visibility requirements are satisfiable;
 *     - whether consistency requirements are satisfiable;
 *     - whether synchronization requirements are satisfiable;
 *     - whether resource requirements can be satisfied;
 *     - whether constraints can be satisfied;
 *     - whether preferences can be honored;
 *     - whether hints are applicable;
 *     - whether a shared-memory operation is supported by the selected
 *       semantic/runtime model.
 *
 * None of these decisions occur in the parser.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Shared-memory syntax must lower through the existing domain-neutral AST and
 * semantic representation.
 *
 * The downstream representation may participate in:
 *
 *     classical IR;
 *     hybrid semantic representation;
 *     hardware/HDL representation;
 *     distributed execution representation;
 *     accelerator execution representation;
 *     quantum-associated classical state.
 *
 * Quantum computation remains governed by:
 *
 *     quantum::ir
 *
 * This file MUST NOT create:
 *
 *     SharedMemoryIR
 *     QuantumSharedMemoryIR
 *     PhysicalSharedMemoryIR
 *
 * merely because a shared-memory construct interacts with another domain.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar may preserve symbolic resource intent such as:
 *
 *     memory::shared(
 *         value,
 *         requires = capability("shared.memory")
 *     );
 *
 * or:
 *
 *     memory::shared(
 *         value,
 *         requires = memory_capacity
 *     );
 *
 * The parser does not determine whether the requirement is satisfiable.
 *
 * Resource/capability analysis remains downstream.
 *
 * ============================================================================
 * CONCURRENCY CONTRACT
 * ============================================================================
 *
 * Shared memory and concurrency are related but are not the same abstraction.
 *
 * This file may reference synchronization intent:
 *
 *     synchronize = fence
 *     synchronize = release_acquire
 *
 * but MUST NOT redefine:
 *
 *     mutexes;
 *     semaphores;
 *     channels;
 *     actors;
 *     tasks;
 *     barriers;
 *     schedulers;
 *     executors.
 *
 * Those belong to concurrency/effect grammars.
 *
 * ============================================================================
 * OWNERSHIP / BORROWING CONTRACT
 * ============================================================================
 *
 * Sharing does not override ownership.
 *
 * For example:
 *
 *     memory::shared(value);
 *
 * does not automatically make `value` copyable, movable, borrowable, or
 * globally mutable.
 *
 * Ownership and borrowing analysis remains authoritative.
 *
 * Shared-memory syntax merely contributes semantic intent.
 *
 * ============================================================================
 * DISTRIBUTED-MEMORY CONTRACT
 * ============================================================================
 *
 * A shared-memory construct may eventually be realized by distributed shared
 * memory.
 *
 * This file MUST NOT define:
 *
 *     node placement;
 *     replica count;
 *     shard count;
 *     partition topology;
 *     network topology;
 *     process placement.
 *
 * Those belong to distributed-memory, distributed execution, resource, and
 * deployment semantics.
 *
 * ============================================================================
 * ACCELERATOR CONTRACT
 * ============================================================================
 *
 * Shared memory may be lowered to:
 *
 *     CPU-visible memory;
 *     accelerator-visible memory;
 *     unified memory;
 *     mapped memory;
 *     future heterogeneous memory.
 *
 * This file MUST NOT select:
 *
 *     GPU;
 *     accelerator;
 *     device ID;
 *     memory bank;
 *     DMA engine;
 *     cache;
 *     interconnect.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum programs may contain shared classical state associated with quantum
 * computation.
 *
 * The architecture remains:
 *
 *     Zamani source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *
 * Shared-memory grammar MUST NOT define:
 *
 *     physical qubit locations;
 *     quantum memory addresses;
 *     QPU topology;
 *     QEC;
 *     noise models;
 *     pulse schedules.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Shared memory can appear in software/hardware co-design.
 *
 * The grammar may preserve semantic intent such as:
 *
 *     visibility;
 *     access;
 *     consistency;
 *     synchronization;
 *     latency requirements;
 *     capability requirements.
 *
 * It MUST NOT define universal physical widths or topology.
 *
 * There must be no grammar-level equivalent of:
 *
 *     shared_wire[31:0]
 *     memory_bank_0
 *     cpu_0
 *     gpu_0
 *     node_0
 *
 * as universal implementation constructs.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The stable public integration rule is:
 *
 *     sharedMemoryConstruct
 *
 * The rule is intentionally small enough to be consumed by the Memory
 * composition grammar without creating a second memory root.
 *
 * ============================================================================
 */

parser grammar SharedMemory;

options {
    tokenVocab = ZamaniLexer;
}

import Memory, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Shared-memory syntax consists of:
 *
 *     - semantic operations;
 *     - shared-memory annotations.
 *
 * Bare contextual words such as:
 *
 *     shared x;
 *
 * are deliberately NOT made parser-level syntax here because `shared` is
 * currently not a reserved lexical token. Treating arbitrary IDENTIFIER
 * values as keywords would make the grammar ambiguous.
 *
 * The canonical target-independent form is therefore operation-oriented:
 *
 *     memory::shared(...)
 *     memory::share(...)
 *     memory::unshare(...)
 *     memory::access(...)
 *
 * This also provides a stable extension mechanism for future forms.
 */
sharedMemoryConstruct
    : sharedMemoryStatement
    | sharedMemoryExpression
    | sharedMemoryAnnotation
    ;


/*
 * ============================================================================
 * 2. STATEMENT FORM
 * ============================================================================
 *
 * A shared-memory operation can appear as a statement.
 */
sharedMemoryStatement
    : sharedMemoryOperation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. EXPRESSION FORM
 * ============================================================================
 *
 * An operation may also be expression-valued.
 *
 * Whether a specific operation is permitted in expression position is a
 * semantic/type-system decision.
 */
sharedMemoryExpression
    : sharedMemoryOperation
    ;


/*
 * ============================================================================
 * 4. OPERATION
 * ============================================================================
 *
 * The operation name is intentionally open-world.
 *
 * Examples:
 *
 *     memory::shared(...)
 *     memory::share(...)
 *     memory::unshare(...)
 *     memory::access(...)
 *     memory::shared::transaction(...)
 *     memory::shared::vendor::operation(...)
 *
 * The parser does not enumerate technologies or protocols.
 */
sharedMemoryOperation
    : sharedMemoryOperationName
      LPAREN
      sharedMemoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 5. OPERATION NAME
 * ============================================================================
 *
 * Reuse Memory's canonical qualified-name representation.
 *
 * The semantic layer determines whether the name belongs to the shared-memory
 * namespace.
 */
sharedMemoryOperationName
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * 6. ARGUMENT LIST
 * ============================================================================
 *
 * Shared-memory arguments are deliberately more structured than a completely
 * generic memory operation so that requirements, constraints, preferences,
 * hints, and metadata remain distinguishable.
 */
sharedMemoryArgumentList
    : sharedMemoryArgument
      (
          COMMA
          sharedMemoryArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. ARGUMENT
 * ============================================================================
 */
sharedMemoryArgument
    : expression
    | sharedMemoryNamedArgument
    | sharedMemoryRequirementArgument
    | sharedMemoryConstraintArgument
    | sharedMemoryPreferenceArgument
    | sharedMemoryHintArgument
    ;


/*
 * ============================================================================
 * 8. GENERAL NAMED ARGUMENT
 * ============================================================================
 *
 * Examples:
 *
 *     access = read
 *     visibility = eventual
 *     consistency = relaxed
 *     synchronize = fence
 */
sharedMemoryNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 9. REQUIREMENT
 * ============================================================================
 *
 * REQUIREMENT is lexically reserved by the canonical lexer and therefore
 * provides an unambiguous structural boundary.
 *
 * Example:
 *
 *     memory::shared(
 *         value,
 *         requires capability("shared.memory")
 *     )
 */
sharedMemoryRequirementArgument
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * 10. CONSTRAINT
 * ============================================================================
 *
 * Example:
 *
 *     memory::shared(
 *         value,
 *         constraint consistency != unsupported
 *     )
 */
sharedMemoryConstraintArgument
    : CONSTRAINT
      expression
    ;


/*
 * ============================================================================
 * 11. PREFERENCE
 * ============================================================================
 *
 * Example:
 *
 *     memory::shared(
 *         value,
 *         prefer coherence
 *     )
 */
sharedMemoryPreferenceArgument
    : PREFER
      expression
    ;


/*
 * ============================================================================
 * 12. HINT
 * ============================================================================
 *
 * Example:
 *
 *     memory::shared(
 *         value,
 *         hint locality
 *     )
 */
sharedMemoryHintArgument
    : HINT
      expression
    ;


/*
 * ============================================================================
 * 13. SHARED-MEMORY ANNOTATION
 * ============================================================================
 *
 * Existing annotation infrastructure is reused.
 *
 * Examples:
 *
 *     @memory::shared(value)
 *     @memory::shared(access = read)
 *     @memory::shared(visibility = eventual)
 *
 * This avoids introducing another annotation/token system.
 */
sharedMemoryAnnotation
    : AT
      sharedMemoryOperationName
      LPAREN
      sharedMemoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 14. COMMON SEMANTIC ACCESS ARGUMENTS
 * ============================================================================
 *
 * These are NOT reserved words.
 *
 * They are ordinary identifiers represented through the generic named
 * argument rule:
 *
 *     access = read
 *     access = write
 *     access = read_write
 *
 * The grammar intentionally does not enumerate the access vocabulary.
 *
 * Future examples may include:
 *
 *     atomic_read
 *     transactional
 *     streaming
 *     snapshot
 *     speculative
 *     capability_scoped
 *
 * without modifying this parser grammar.
 */


/*
 * ============================================================================
 * 15. VISIBILITY ARGUMENT
 * ============================================================================
 *
 * Represented through:
 *
 *     visibility = <expression>
 *
 * No physical propagation mechanism is implied.
 */


/*
 * ============================================================================
 * 16. CONSISTENCY ARGUMENT
 * ============================================================================
 *
 * Represented through:
 *
 *     consistency = <expression>
 *
 * Examples may include:
 *
 *     relaxed
 *     acquire
 *     release
 *     acquire_release
 *     sequential
 *     transactional
 *
 * These remain semantic values.
 */


/*
 * ============================================================================
 * 17. SYNCHRONIZATION ARGUMENT
 * ============================================================================
 *
 * Represented through:
 *
 *     synchronize = <expression>
 *
 * Synchronization implementation belongs to concurrency/effect semantics.
 */


/*
 * ============================================================================
 * 18. PARTICIPANT / EXECUTION-CONTEXT ARGUMENT
 * ============================================================================
 *
 * A shared-memory operation may express a symbolic sharing domain:
 *
 *     domain = workers
 *     domain = processes
 *     domain = tasks
 *     domain = devices
 *
 * The parser does not interpret the domain as a fixed machine topology.
 *
 * No participant count is encoded.
 */


/*
 * ============================================================================
 * 19. TARGET
 * ============================================================================
 *
 * The target is normally the first ordinary operation argument.
 *
 * The canonical memory place grammar remains authoritative.
 *
 * Semantic analysis determines whether an argument is a valid memory target.
 *
 * This grammar deliberately does not duplicate memoryPlace.
 */


/*
 * ============================================================================
 * 20. OPTIONAL SHARED-MEMORY TARGET FORM
 * ============================================================================
 *
 * This reusable rule is provided for downstream grammar components that need
 * to identify a shared-memory target structurally.
 *
 * It delegates to the canonical Memory grammar.
 */
sharedMemoryTarget
    : memoryPlace
    ;


/*
 * ============================================================================
 * 21. SHARED-MEMORY SPECIFICATION
 * ============================================================================
 *
 * A specification is represented by the same operation-oriented structure.
 *
 * This rule exists as a stable semantic composition point for future memory
 * declarations, region declarations, data declarations, and domain-specific
 * constructs.
 */
sharedMemorySpecification
    : sharedMemoryOperationName
      LPAREN
      sharedMemoryArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 22. OPEN-WORLD EXTENSION
 * ============================================================================
 *
 * Future shared-memory systems can use qualified operation names without
 * changing the universal grammar.
 *
 * Examples:
 *
 *     memory::shared::transactional(...)
 *     memory::shared::persistent(...)
 *     memory::shared::distributed(...)
 *     memory::shared::accelerator(...)
 *     memory::shared::vendor::operation(...)
 *     memory::shared::future::technology(...)
 *
 * Semantic registration, capability checking, and implementation selection
 * remain downstream.
 */
sharedMemoryExtension
    : sharedMemoryOperation
    ;


/*
 * ============================================================================
 * 23. DOMAIN BOUNDARY
 * ============================================================================
 *
 * Shared-memory syntax may coexist with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     concurrency
 *     effects
 *     resources
 *
 * No domain gets a second shared-memory language.
 *
 * All domains consume the same shared-memory semantic contract.
 */


/*
 * ============================================================================
 * 24. RESOURCE / CAPABILITY EXAMPLES
 * ============================================================================
 *
 * The following are conceptual source forms:
 *
 *     memory::shared(
 *         buffer,
 *         requires capability("shared.memory")
 *     );
 *
 *     memory::shared(
 *         buffer,
 *         requires capability("shared.memory.atomic")
 *     );
 *
 *     memory::shared(
 *         buffer,
 *         constraint memory_class != unavailable
 *     );
 *
 *     memory::shared(
 *         buffer,
 *         prefer locality
 *     );
 *
 *     memory::shared(
 *         buffer,
 *         hint frequently_accessed
 *     );
 *
 * The grammar preserves these values.
 *
 * The resource/capability system determines their meaning and satisfiability.
 */


/*
 * ============================================================================
 * 25. OWNERSHIP EXAMPLE
 * ============================================================================
 *
 * The following is syntactically representable:
 *
 *     memory::shared(
 *         value,
 *         access = read
 *     );
 *
 * Whether `value` can actually be shared is determined by:
 *
 *     ownership analysis;
 *     borrowing analysis;
 *     type analysis;
 *     lifetime analysis.
 *
 * The parser MUST NOT bypass those systems.
 */


/*
 * ============================================================================
 * 26. CONSISTENCY / VISIBILITY EXAMPLES
 * ============================================================================
 *
 * Examples:
 *
 *     memory::shared(
 *         state,
 *         visibility = eventual,
 *         consistency = relaxed
 *     );
 *
 *     memory::shared(
 *         state,
 *         visibility = immediate,
 *         consistency = sequential
 *     );
 *
 * These do not mandate a particular cache-coherence protocol or hardware
 * memory model.
 */


/*
 * ============================================================================
 * 27. DISTRIBUTED SHARED MEMORY
 * ============================================================================
 *
 * A distributed implementation may use:
 *
 *     memory::shared(
 *         object,
 *         domain = distributed,
 *         consistency = eventual
 *     );
 *
 * This grammar does not introduce:
 *
 *     node count;
 *     replica count;
 *     shard count;
 *     network topology;
 *     node IDs.
 *
 * Distributed-memory grammar remains responsible for distributed realization
 * intent.
 */


/*
 * ============================================================================
 * 28. ACCELERATOR SHARED MEMORY
 * ============================================================================
 *
 * A heterogeneous implementation may use:
 *
 *     memory::shared(
 *         buffer,
 *         domain = accelerator,
 *         visibility = device_visible
 *     );
 *
 * The parser does not select a GPU, accelerator, memory bank, or device ID.
 */


/*
 * ============================================================================
 * 29. QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * Shared classical state may coexist with quantum computation:
 *
 *     memory::shared(
 *         measurement_state,
 *         access = read_write
 *     );
 *
 * The quantum operation itself continues through the canonical quantum
 * semantic path:
 *
 *     generic AST Operation
 *          |
 *          v
 *     semantic quantum operation
 *          |
 *          v
 *     quantum::ir
 *
 * This file contributes memory semantics only.
 */


/*
 * ============================================================================
 * 30. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Shared-memory intent may coexist with hardware/co-design constructs.
 *
 * The grammar does not establish:
 *
 *     wire width;
 *     memory bank number;
 *     physical port number;
 *     cache level;
 *     device ID;
 *     address width.
 *
 * Those properties belong to target-specific semantic realization.
 */


/*
 * ============================================================================
 * 31. AST -> SEMANTICS -> IR INTEGRATION
 * ============================================================================
 *
 * Required flow:
 *
 *     sharedMemoryConstruct
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic shared-memory model
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     ownership/type         resources/effects
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *             canonical semantic IR
 *
 * The grammar does not select the final target representation.
 */


/*
 * ============================================================================
 * 32. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler must consume the semantic representation produced from this
 * grammar.
 *
 * Compiler responsibilities include:
 *
 *     - ownership checking;
 *     - type checking;
 *     - alias analysis;
 *     - effect analysis;
 *     - resource analysis;
 *     - capability satisfaction;
 *     - legality checking;
 *     - optimization;
 *     - lowering;
 *     - target selection.
 *
 * This grammar performs none of those operations.
 */


/*
 * ============================================================================
 * 33. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior may include:
 *
 *     - shared-state realization;
 *     - synchronization;
 *     - visibility enforcement;
 *     - consistency enforcement;
 *     - recovery;
 *     - migration;
 *     - monitoring.
 *
 * None of those behaviors are implemented by the parser.
 */


/*
 * ============================================================================
 * 34. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     - source text;
 *     - grammar version;
 *     - canonical lexer;
 *     - parser configuration;
 *     - explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend upon:
 *
 *     - CPU count;
 *     - GPU availability;
 *     - QPU availability;
 *     - memory capacity;
 *     - operating-system state;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime state.
 */


/*
 * ============================================================================
 * 35. DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level diagnostics include:
 *
 *     - missing operation name;
 *     - missing opening parenthesis;
 *     - missing closing parenthesis;
 *     - malformed argument list;
 *     - malformed named argument;
 *     - malformed requirement;
 *     - malformed constraint;
 *     - malformed preference;
 *     - malformed hint;
 *     - malformed annotation.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     - object cannot be shared;
 *     - ownership violation;
 *     - invalid borrow;
 *     - invalid lifetime;
 *     - conflicting access modes;
 *     - unsatisfied consistency requirement;
 *     - unsatisfied visibility requirement;
 *     - unsatisfied synchronization requirement;
 *     - unsatisfied capability;
 *     - violated resource constraint.
 *
 * The parser MUST NOT manufacture semantic diagnostics.
 */


/*
 * ============================================================================
 * 36. SECURITY
 * ============================================================================
 *
 * Parsing this grammar performs no:
 *
 *     - memory allocation on behalf of the program;
 *     - hardware access;
 *     - filesystem access;
 *     - network access;
 *     - secret access;
 *     - capability authorization;
 *     - runtime execution.
 *
 * Security policy remains downstream.
 */


/*
 * ============================================================================
 * 37. PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * Repetition is represented structurally:
 *
 *     *
 *     +
 *
 * rather than through finite alternatives.
 *
 * There is no universal bound on:
 *
 *     argument count;
 *     qualification depth;
 *     operation count;
 *     shared objects;
 *     metadata entries;
 *     source size.
 *
 * Actual parser-resource exhaustion is an implementation concern, not a
 * language semantic limit.
 */


/*
 * ============================================================================
 * 38. COMPATIBILITY
 * ============================================================================
 *
 * The stable public entry point is:
 *
 *     sharedMemoryConstruct
 *
 * Existing consumers should integrate through this rule.
 *
 * Future extensions should prefer:
 *
 *     qualified operation names;
 *     named arguments;
 *     new semantic values;
 *     new optional metadata;
 *
 * rather than introducing incompatible lexical keywords.
 *
 * Existing valid forms remain structurally valid when future semantic
 * capabilities are added.
 */


/*
 * ============================================================================
 * 39. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms should be accepted structurally:
 *
 *     memory::shared(value);
 *
 *     memory::share(value);
 *
 *     memory::unshare(value);
 *
 *     memory::access(
 *         value,
 *         access = read
 *     );
 *
 *     memory::shared(
 *         value,
 *         access = read_write,
 *         visibility = eventual,
 *         consistency = relaxed
 *     );
 *
 *     memory::shared(
 *         value,
 *         requires capability("shared.memory")
 *     );
 *
 *     memory::shared(
 *         value,
 *         constraint consistency != unavailable
 *     );
 *
 *     memory::shared(
 *         value,
 *         prefer locality
 *     );
 *
 *     memory::shared(
 *         value,
 *         hint frequently_accessed
 *     );
 *
 *     @memory::shared(value)
 *
 *     memory::shared::transactional(value);
 *
 *     memory::shared::distributed(value);
 *
 *     memory::shared::accelerator(value);
 *
 *     memory::shared::vendor::future_operation(value);
 *
 * These examples establish syntax only.
 *
 * Their semantic legality is downstream.
 */


/*
 * ============================================================================
 * 40. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The grammar must reject malformed syntax such as:
 *
 *     memory::shared(
 *
 *     memory::shared);
 *
 *     memory::shared(, value);
 *
 *     memory::shared(value,);
 *
 *     memory::shared(value,,access);
 *
 *     memory::shared(value access = read);
 *
 *     memory::shared(value requires);
 *
 *     memory::shared(value constraint);
 *
 *     memory::shared(value prefer);
 *
 *     memory::shared(value hint);
 *
 *     @memory::shared(
 *
 *     @memory::shared(value
 *
 * Semantic-invalid constructs should not be confused with syntactically
 * malformed constructs.
 */


/*
 * ============================================================================
 * 41. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests must cover:
 *
 *     - one shared object;
 *     - deeply qualified operation names;
 *     - many named arguments;
 *     - many requirements;
 *     - many constraints;
 *     - many preferences;
 *     - many hints;
 *     - nested expressions;
 *     - symbolic resource quantities;
 *     - dynamically computed values;
 *     - large source files.
 *
 * Tests MUST NOT establish a source-language maximum.
 */


/*
 * ============================================================================
 * 42. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Generate increasingly large valid programs and verify that the grammar does
 * not fail because of artificial limits on:
 *
 *     shared-object count;
 *     participant count;
 *     node count;
 *     device count;
 *     memory capacity;
 *     address width;
 *     topology size;
 *     synchronization relationship count.
 *
 * Any actual parser/compiler resource exhaustion must be reported as an
 * implementation-resource condition rather than encoded as language syntax.
 */


/*
 * ============================================================================
 * 43. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required integration coverage:
 *
 *     classical + shared memory
 *     quantum + shared classical state
 *     hybrid + shared memory
 *     HDL + shared memory
 *     hardware + shared memory
 *     distributed + shared memory
 *     AI + shared memory
 *     data + shared memory
 *     networking + shared memory
 *     concurrency + shared memory
 *     resources + shared memory
 *     effects + shared memory
 *     security + shared memory
 *
 * At least one end-to-end fixture must combine:
 *
 *     classical computation
 *     quantum operation
 *     shared state
 *     distributed intent
 *     hardware capability requirements
 *
 * without embedding a machine-specific topology.
 */


/*
 * ============================================================================
 * 44. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source;
 *     grammar version;
 *     lexer vocabulary;
 *     parser configuration;
 *
 * the resulting parse structure must be equivalent.
 *
 * No hardware discovery or runtime state may affect parsing.
 */


/*
 * ============================================================================
 * 45. ROUND-TRIP TEST CONTRACT
 * ============================================================================
 *
 * Where an AST printer/formatter exists:
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
 *     operation identity;
 *     target expression;
 *     named arguments;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     annotation identity;
 *     semantic intent.
 */


/*
 * ============================================================================
 * 46. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal implementation limits include:
 *
 *     MAX_SHARED_MEMORY
 *     MAX_SHARED_OBJECTS
 *     MAX_SHARED_REGIONS
 *     MAX_SHARED_PARTICIPANTS
 *     MAX_SHARERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * Forbidden universal physical identities include:
 *
 *     cpu_0
 *     gpu_0
 *     qpu_0
 *     node_0
 *     device_0
 *     memory_bank_0
 *     address_0
 *
 * A numeric literal supplied by a program remains legal when it is program
 * data or a semantic requirement.
 *
 * The prohibition applies only when the grammar turns a value into a
 * universal machine limit or implementation assumption.
 */


/*
 * ============================================================================
 * 47. FORBIDDEN DEPENDENCIES
 * ============================================================================
 *
 * This grammar must never depend upon:
 *
 *     runtime implementation;
 *     hardware discovery;
 *     physical topology;
 *     physical addresses;
 *     target-specific allocation;
 *     cache implementation;
 *     synchronization implementation;
 *     scheduler implementation;
 *     optimizer implementation;
 *     routing implementation;
 *     QEC implementation;
 *     ZQN implementation;
 *     HAL implementation;
 *     vendor APIs.
 */


/*
 * ============================================================================
 * 48. COMPLETION CRITERIA
 * ============================================================================
 *
 * shared-memory.g4 is complete when:
 *
 * [ ] It is a valid ANTLR4 parser grammar.
 *
 * [ ] It uses tokenVocab = ZamaniLexer.
 *
 * [ ] It introduces no lexer rules.
 *
 * [ ] It introduces no unnecessary lexer tokens.
 *
 * [ ] It reuses Memory's canonical memoryPlace.
 *
 * [ ] It reuses Memory's canonical memoryQualifiedName.
 *
 * [ ] It does not redefine generic memory operations.
 *
 * [ ] It does not duplicate ownership syntax.
 *
 * [ ] It does not duplicate borrowing syntax.
 *
 * [ ] It does not duplicate lifetime syntax.
 *
 * [ ] It does not duplicate allocation syntax.
 *
 * [ ] It does not duplicate distributed-memory syntax.
 *
 * [ ] It does not duplicate accelerator-memory syntax.
 *
 * [ ] It does not duplicate quantum-memory syntax.
 *
 * [ ] It does not duplicate concurrency primitives.
 *
 * [ ] Requirements remain distinct from constraints.
 *
 * [ ] Constraints remain distinct from preferences.
 *
 * [ ] Preferences remain distinct from hints.
 *
 * [ ] Shared-memory operation names remain open-world.
 *
 * [ ] Future shared-memory technologies do not require parser rewrites merely
 *     to obtain a new semantic operation name.
 *
 * [ ] No fixed participant count exists.
 *
 * [ ] No fixed memory capacity exists.
 *
 * [ ] No physical address is represented.
 *
 * [ ] No hardware identifier is required.
 *
 * [ ] No topology is hard-coded.
 *
 * [ ] No quantum gate enumeration exists.
 *
 * [ ] No physical qubit enumeration exists.
 *
 * [ ] No second quantum IR exists.
 *
 * [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 * [ ] Runtime behavior remains downstream.
 *
 * [ ] Resource discovery remains downstream.
 *
 * [ ] Scheduling remains downstream.
 *
 * [ ] Optimization remains downstream.
 *
 * [ ] Routing remains downstream.
 *
 * [ ] QEC remains downstream.
 *
 * [ ] ZQN remains downstream.
 *
 * [ ] HAL remains downstream.
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
 * [ ] Round-trip tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Shared-memory grammar answers:
 *
 *     "What shared-memory behavior does the program require or request?"
 *
 * It does NOT answer:
 *
 *     "Which physical memory implements it?"
 *
 *     "Which cache protocol implements it?"
 *
 *     "Which NUMA node implements it?"
 *
 *     "Which CPU implements it?"
 *
 *     "Which GPU implements it?"
 *
 *     "Which accelerator implements it?"
 *
 *     "Which node implements it?"
 *
 *     "Which physical address implements it?"
 *
 *     "Which synchronization primitive implements it?"
 *
 * Those questions belong downstream.
 *
 * Therefore:
 *
 *     one Zamani source
 *          |
 *          v
 *     one semantic meaning
 *          |
 *          +-------------------------------+
 *          |               |               |
 *          v               v               v
 *       tiny machine    large machine    future machine
 *          |               |               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                   target realization
 *
 * This preserves:
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
 * ============================================================================
 * END
 * ============================================================================
 */