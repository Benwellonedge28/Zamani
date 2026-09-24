/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/shared-data.g4
 *
 * Grammar:
 *     SharedData
 *
 * Status:
 *     Production hybrid shared-data boundary grammar.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY the source-level syntax for sharing logical data
 * between computational domains.
 *
 * The supported conceptual domains include:
 *
 *     classical
 *     quantum
 *     accelerator
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     future computational domains
 *
 * "Shared data" means a logical value, view, stream, buffer, tensor,
 * measurement result, intermediate result, or other semantic data object
 * whose lifetime or visibility crosses a computational-domain boundary.
 *
 * This file does NOT define:
 *
 *     - data types;
 *     - data schemas;
 *     - collections;
 *     - streams;
 *     - serialization;
 *     - memory allocation;
 *     - ownership implementation;
 *     - borrowing implementation;
 *     - synchronization implementation;
 *     - networking;
 *     - transport;
 *     - DMA;
 *     - quantum measurement semantics;
 *     - quantum IR;
 *     - hardware topology;
 *     - accelerator implementation;
 *     - scheduling;
 *     - routing;
 *     - runtime resource discovery;
 *     - physical placement;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * Those remain owned by their respective subsystems.
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
 *     ZamaniParser
 *          |
 *          v
 *     SharedData syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+---------+----------+----------+
 *     |              |          |          |
 *     v              v          v          v
 * classical      quantum    accelerator   HDL
 * semantics      semantics   semantics    semantics
 *                    |
 *                    v
 *                quantum::ir
 *                    |
 *                    v
 *          canonical semantic model
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       optimize   route    schedule
 *                              |
 *                              v
 *                           runtime
 *
 * Shared-data syntax therefore represents a semantic dependency/boundary.
 *
 * It does NOT prescribe how that boundary is physically realized.
 *
 * ============================================================================
 * CANONICAL COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is imported by:
 *
 *     grammar/hybrid/Hybrid.g4
 *
 * through the canonical hybrid composition layer.
 *
 * It MUST NOT import:
 *
 *     ZamaniParser
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical lexical vocabulary remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The canonical parser composition root remains:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 * SINGLE-OWNER CONTRACT
 * ============================================================================
 *
 * This file owns:
 *
 *     sharedDataConstruct
 *     sharedDataDeclaration
 *     sharedDataAccess
 *     sharedDataPublish
 *     sharedDataConsume
 *     sharedDataTransfer
 *     sharedDataBinding
 *     sharedDataSynchronization
 *     sharedDataPolicy
 *
 * It does NOT own the definitions of:
 *
 *     expression
 *     typeExpression
 *     typeAnnotation
 *     qualifiedName
 *     argumentList
 *     block
 *     statement
 *     lambdaExpression
 *
 * Those belong to the canonical language grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Shared data MUST remain independent of:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     node count
 *     memory capacity
 *     storage capacity
 *     network size
 *     topology size
 *     register width
 *     tensor rank
 *     vector width
 *
 * This grammar therefore contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * A source program may express a resource requirement through the canonical
 * resource grammar, for example:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("shared.memory");
 *
 *     requires capability("quantum.measurement");
 *
 * This grammar merely represents the shared-data relationship.
 *
 * Resource satisfaction belongs downstream.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * The central distinction is:
 *
 *     DATA IDENTITY
 *          !=
 *     DATA VISIBILITY
 *          !=
 *     DATA OWNERSHIP
 *          !=
 *     DATA STORAGE
 *          !=
 *     DATA LOCATION
 *          !=
 *     DATA TRANSPORT
 *          !=
 *     DATA SYNCHRONIZATION
 *          !=
 *     DATA PHYSICAL PLACEMENT
 *
 * The source language may describe the first few semantic properties.
 *
 * The compiler/runtime determines the physical realization.
 *
 * ============================================================================
 * DOMAIN-NEUTRAL SHARED DATA
 * ============================================================================
 *
 * The same shared-data construct can connect:
 *
 *     classical -> quantum
 *     quantum -> classical
 *     classical -> accelerator
 *     accelerator -> classical
 *     quantum -> accelerator
 *     accelerator -> quantum
 *     HDL -> software
 *     software -> HDL
 *     distributed -> local
 *     local -> distributed
 *     AI -> classical
 *     classical -> AI
 *     data -> quantum
 *     quantum -> data
 *     future-domain -> future-domain
 *
 * The grammar does not enumerate all possible domain pairs.
 *
 * Domain names remain extensible semantic names.
 *
 * ============================================================================
 * OWNERSHIP AND BORROWING
 * ============================================================================
 *
 * "shared" in this file does NOT mean a particular Rust ownership mechanism.
 *
 * It means that the source-level logical data object is made available across
 * a declared semantic boundary.
 *
 * Whether the implementation uses:
 *
 *     copy
 *     move
 *     borrow
 *     reference
 *     shared memory
 *     message passing
 *     DMA
 *     serialization
 *     zero-copy transport
 *     accelerator memory
 *     distributed memory
 *     quantum/classical interface
 *
 * is decided downstream.
 *
 * ============================================================================
 * QUANTUM SAFETY
 * ============================================================================
 *
 * Shared data MUST NOT imply that quantum state can be freely copied.
 *
 * The semantic/type system must determine whether a value is:
 *
 *     copyable
 *     movable
 *     borrowable
 *     linear
 *     affine
 *     classical
 *     quantum
 *     measurement-derived
 *
 * This grammar only records the source-level sharing intent.
 *
 * Quantum no-cloning constraints, measurement semantics, linearity,
 * ownership, and quantum state validity remain semantic/type-system concerns.
 *
 * ============================================================================
 * DATA IDENTITY
 * ============================================================================
 *
 * A shared-data declaration establishes a logical name.
 *
 * It does not establish:
 *
 *     physical address
 *     device address
 *     memory bank
 *     NUMA node
 *     GPU memory address
 *     QPU register
 *     network endpoint
 *     filesystem path
 *
 * Such information belongs downstream.
 *
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The hybrid composition grammar should consume exactly:
 *
 *     sharedDataConstruct
 *
 * for shared-data syntax.
 *
 * No other hybrid grammar file should duplicate these alternatives.
 *
 * ============================================================================
 */

parser grammar SharedData;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 2. PUBLIC SHARED-DATA CONSTRUCT
 * ============================================================================
 *
 * The public entry point deliberately has a small stable surface.
 *
 * New shared-data implementation forms should normally be added underneath
 * this rule rather than changing the root hybrid composition.
 */
sharedDataConstruct
    : sharedDataDeclaration
    | sharedDataAccess
    | sharedDataPublish
    | sharedDataConsume
    | sharedDataTransfer
    | sharedDataBinding
    | sharedDataSynchronization
    | sharedDataPolicy
    ;


/*
 * ============================================================================
 * 3. SHARED DATA DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     shared data value: T;
 *
 *     shared data value: T = expression;
 *
 *     shared data value;
 *
 * The exact type semantics belong to the type system.
 *
 * The grammar intentionally does not introduce a new SharedDataType.
 */
sharedDataDeclaration
    : K_SHARED
      K_DATA
      IDENTIFIER
      typeAnnotation?
      sharedDataInitializer?
      SEMICOLON
    ;


sharedDataInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 4. SHARED DATA ACCESS
 * ============================================================================
 *
 * Access identifies an existing logical shared-data object.
 *
 * Example conceptual form:
 *
 *     shared data value;
 *
 * followed by an ordinary expression/use site.
 *
 * The dedicated access form is intentionally explicit for domain tooling.
 */
sharedDataAccess
    : K_SHARED
      K_DATA
      qualifiedName
      sharedDataAccessClause?
      SEMICOLON
    ;


sharedDataAccessClause
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 5. PUBLISH
 * ============================================================================
 *
 * Publishing makes a logical value available to another semantic domain.
 *
 * It does NOT mean:
 *
 *     network send
 *     DMA
 *     physical copy
 *     memory copy
 *     device transfer
 *
 * Those are downstream realizations.
 *
 * The source-level meaning is:
 *
 *     make this logical value available at the declared boundary.
 *
 * Example:
 *
 *     shared data publish value to quantum;
 *
 * The target domain is an open semantic identifier.
 */
sharedDataPublish
    : K_SHARED
      K_DATA
      K_PUBLISH
      expression
      K_TO
      sharedDataDomain
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. CONSUME
 * ============================================================================
 *
 * Consuming identifies a logical shared-data value that a domain will use.
 *
 * Example:
 *
 *     shared data consume result from quantum;
 *
 * The implementation decides how the value becomes available.
 */
sharedDataConsume
    : K_SHARED
      K_DATA
      K_CONSUME
      IDENTIFIER
      K_FROM
      sharedDataDomain
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. TRANSFER
 * ============================================================================
 *
 * Transfer expresses a semantic domain crossing.
 *
 * It is deliberately NOT called:
 *
 *     copy
 *     memcpy
 *     dma
 *     send
 *
 * because the implementation may legally choose a zero-copy, move,
 * reference, message-passing, memory-sharing, or other representation.
 *
 * Example:
 *
 *     shared data transfer value from classical to quantum;
 *
 * The source expresses the semantic crossing.
 *
 * The compiler determines realization.
 */
sharedDataTransfer
    : K_SHARED
      K_DATA
      K_TRANSFER
      expression
      K_FROM
      sharedDataDomain
      K_TO
      sharedDataDomain
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. SHARED DATA BINDING
 * ============================================================================
 *
 * A binding gives a logical name to data crossing a domain boundary.
 *
 * Example:
 *
 *     shared data result = measurement;
 *
 * The type system determines whether the value can legally cross the
 * boundary.
 */
sharedDataBinding
    : K_SHARED
      K_DATA
      IDENTIFIER
      typeAnnotation?
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. SYNCHRONIZATION
 * ============================================================================
 *
 * Shared-data synchronization establishes a semantic visibility/order
 * boundary.
 *
 * It does NOT establish:
 *
 *     clock frequency
 *     hardware barrier
 *     bus barrier
 *     network barrier
 *     device fence
 *     thread count
 *     queue identity
 *
 * Those are downstream.
 *
 * Example:
 *
 *     shared data synchronize value;
 *
 * or:
 *
 *     shared data synchronize(value);
 */
sharedDataSynchronization
    : K_SHARED
      K_DATA
      K_SYNCHRONIZE
      sharedDataSynchronizationTarget?
      SEMICOLON
    ;


sharedDataSynchronizationTarget
    : LPAREN
      expression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 10. SHARED-DATA POLICY
 * ============================================================================
 *
 * Policy describes semantic properties of the shared-data boundary.
 *
 * This grammar does not hard-code the policy vocabulary.
 *
 * The policy key is an identifier/qualified name and the value is an ordinary
 * Zamani expression.
 *
 * This permits future properties without continuously expanding the grammar.
 *
 * Example conceptual forms:
 *
 *     shared data policy consistency = causal;
 *
 *     shared data policy locality = preferred;
 *
 *     shared data policy persistence = durable;
 *
 *     shared data policy ordering = ordered;
 *
 * Semantic validation determines whether the selected policy exists and what
 * it means.
 */
sharedDataPolicy
    : K_SHARED
      K_DATA
      K_POLICY
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. DOMAIN IDENTIFIER
 * ============================================================================
 *
 * Domain identifiers remain open-ended.
 *
 * Examples:
 *
 *     classical
 *     quantum
 *     accelerator
 *     hdl
 *     hardware
 *     distributed
 *     ai
 *     data
 *     networking
 *     future_domain
 *
 * The grammar does not enumerate them.
 *
 * This is mandatory for POCO-REAF and future-domain extensibility.
 */
sharedDataDomain
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * 12. DOMAIN PAIR
 * ============================================================================
 *
 * This rule is intentionally reusable by semantic tooling and future grammar
 * extensions.
 */
sharedDataDomainPair
    : sharedDataDomain
      K_TO
      sharedDataDomain
    ;


/*
 * ============================================================================
 * 13. EXPLICIT DOMAIN BOUNDARY
 * ============================================================================
 *
 * This construct is useful where source code needs to make the boundary
 * visible without requesting a physical transfer implementation.
 *
 * Conceptual:
 *
 *     shared data boundary classical to quantum;
 *
 * This remains a semantic relationship.
 */
sharedDataBoundary
    : K_SHARED
      K_DATA
      K_BOUNDARY
      sharedDataDomainPair
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. VALUE-BASED SHARING
 * ============================================================================
 *
 * A generic expression may itself be shared.
 *
 * This rule exists so semantic analysis can preserve the fact that the
 * expression is intentionally crossing a domain boundary.
 */
sharedDataValue
    : expression
    ;


/*
 * ============================================================================
 * 15. RESOURCE REQUIREMENT ADAPTER
 * ============================================================================
 *
 * Shared-data syntax may carry resource requirements, but it does not own the
 * universal resource grammar.
 *
 * Therefore the requirement expression remains an ordinary expression.
 *
 * Conceptual:
 *
 *     shared data requires capability("shared.memory");
 *
 * Resource satisfaction belongs to resources/hardware/compile/execution.
 */
sharedDataRequirement
    : K_SHARED
      K_DATA
      K_REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. CAPABILITY ADAPTER
 * ============================================================================
 *
 * Capability names remain data/semantic identifiers.
 *
 * This grammar does not discover whether a capability exists.
 */
sharedDataCapability
    : K_SHARED
      K_DATA
      K_CAPABILITY
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. PREFERENCE ADAPTER
 * ============================================================================
 *
 * Preferences remain advisory.
 *
 * They must never silently become requirements.
 */
sharedDataPreference
    : K_SHARED
      K_DATA
      K_PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. EXTENDED PUBLIC CONSTRUCT
 * ============================================================================
 *
 * The extended entry point is kept separate from the minimal stable entry
 * point so that repository integration can adopt additional shared-data
 * metadata without redefining the basic constructs.
 *
 * The canonical hybrid dispatcher may consume this rule once all associated
 * token vocabulary is available.
 */
sharedDataExtendedConstruct
    : sharedDataBoundary
    | sharedDataRequirement
    | sharedDataCapability
    | sharedDataPreference
    ;


/*
 * ============================================================================
 * 19. SEMANTIC OWNERSHIP
 * ============================================================================
 *
 * The semantic layer MUST distinguish at least:
 *
 *     shared logical value
 *     shared logical reference
 *     shared stream
 *     shared tensor/data object
 *     shared measurement result
 *     shared accelerator result
 *     shared HDL/software interface value
 *
 * The grammar does not introduce separate AST types for these unless the
 * canonical AST contract explicitly requires them.
 *
 * Preferred AST shape:
 *
 *     generic shared-data operation
 *
 * containing semantic fields such as:
 *
 *     source
 *     destination
 *     value
 *     type
 *     ownership intent
 *     visibility
 *     synchronization intent
 *     resource requirements
 *     attributes
 *     source span
 *
 * This prevents a proliferation of backend-specific AST nodes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. AST CONTRACT
 * ============================================================================
 *
 * Every shared-data construct must preserve:
 *
 *     - complete source span;
 *     - logical data identity;
 *     - participating domain(s);
 *     - value/expression;
 *     - declared type when present;
 *     - operation kind;
 *     - policy metadata;
 *     - requirement metadata;
 *     - capability metadata;
 *     - preference metadata;
 *     - synchronization intent;
 *     - source spelling where required for diagnostics/refactoring.
 *
 * The parser MUST NOT resolve:
 *
 *     - physical addresses;
 *     - devices;
 *     - memory banks;
 *     - network endpoints;
 *     - physical qubits;
 *     - accelerator IDs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. TYPE INTEGRATION
 * ============================================================================
 *
 * `typeAnnotation` and `typeExpression` remain canonical type-system rules.
 *
 * This file must not define:
 *
 *     Shared<T>
 *     SharedData<T>
 *     SharedMemory<T>
 *
 * as competing universal types merely because a value is shared.
 *
 * Sharing is a semantic property of the operation/boundary.
 *
 * A canonical type system may later expose a formal shared/reference type if
 * that is independently specified.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. MEMORY INTEGRATION
 * ============================================================================
 *
 * Shared data may eventually be realized through:
 *
 *     local memory
 *     shared memory
 *     device memory
 *     distributed memory
 *     persistent memory
 *     accelerator memory
 *     managed memory
 *     message passing
 *     zero-copy references
 *
 * This grammar does not choose among them.
 *
 * Memory ownership remains with:
 *
 *     grammar/memory/
 *
 * Resource realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. DATA INTEGRATION
 * ============================================================================
 *
 * Data semantics remain owned by:
 *
 *     grammar/data/
 *
 * In particular, this file does NOT duplicate:
 *
 *     records
 *     schemas
 *     collections
 *     streams
 *     transformations
 *     serialization
 *     queries
 *     pipelines
 *
 * Shared-data syntax merely identifies a cross-domain relationship involving
 * a logical data value.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical expressions remain owned by the canonical expression/classical
 * grammar.
 *
 * Example:
 *
 *     shared data publish classical_result to quantum;
 *
 * The expression `classical_result` is not redefined here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * Shared data can carry:
 *
 *     - classical parameters into quantum computation;
 *     - measurement results into classical computation;
 *     - semantic quantum operation results;
 *     - control information;
 *     - logical data associated with quantum execution.
 *
 * The shared-data grammar MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - define qubits;
 *     - define physical qubits;
 *     - define quantum circuits;
 *     - define quantum IR;
 *     - define QEC;
 *     - define ZQN.
 *
 * All quantum semantic lowering ultimately goes through:
 *
 *     quantum::ir
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. MEASUREMENT SAFETY
 * ============================================================================
 *
 * A measurement result is classical data only after semantic validation of the
 * corresponding quantum operation.
 *
 * This grammar does not assume that:
 *
 *     quantum state -> classical copy
 *
 * is universally valid.
 *
 * Instead:
 *
 *     quantum operation
 *          |
 *          v
 *     semantic measurement
 *          |
 *          v
 *     measurement result
 *          |
 *          v
 *     shared-data boundary
 *
 * The type/effect/quantum semantic layers enforce the actual rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Shared data may cross:
 *
 *     host <-> accelerator
 *
 * but the grammar does not encode:
 *
 *     GPU0
 *     FPGA0
 *     accelerator0
 *
 * nor:
 *
 *     CUDA
 *     ROCm
 *     vendor-specific memory
 *
 * unless such syntax is explicitly introduced by an interoperability dialect.
 *
 * Portable source remains accelerator-independent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. HDL INTEGRATION
 * ============================================================================
 *
 * Shared data can represent software/hardware co-design boundaries.
 *
 * Examples:
 *
 *     software value -> hardware module
 *
 *     hardware result -> software value
 *
 *     accelerator stream -> classical computation
 *
 * The grammar does not define:
 *
 *     wire widths
 *     physical pins
 *     clock frequencies
 *     register addresses
 *     bus implementations
 *
 * HDL owns hardware-intent syntax.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Shared data may cross distributed execution domains.
 *
 * The source must remain independent of:
 *
 *     node count
 *     node IDs
 *     network topology
 *     provider
 *     IP addresses
 *     ports
 *     transport protocol
 *
 * A logical shared value may be lowered to:
 *
 *     message passing
 *     replicated state
 *     distributed memory
 *     remote reference
 *     streaming
 *     collective communication
 *
 * as determined by downstream compilation/runtime.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Shared data may connect:
 *
 *     model -> classical
 *     classical -> model
 *     dataset -> model
 *     model -> accelerator
 *     quantum -> model
 *     model -> quantum
 *
 * Tensor dimensions, model sizes, batch sizes, dataset sizes, and accelerator
 * resources remain expressions/semantic values rather than grammar limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. OWNERSHIP / LIFETIME CONTRACT
 * ============================================================================
 *
 * The grammar does not decide when shared data is destroyed.
 *
 * Semantic/runtime systems may determine:
 *
 *     lexical lifetime
 *     scope lifetime
 *     task lifetime
 *     region lifetime
 *     stream lifetime
 *     distributed lifetime
 *     persistent lifetime
 *
 * If source syntax eventually exposes explicit lifetime policy, it should be
 * represented as metadata and mapped into the canonical memory/ownership
 * semantics rather than creating a second ownership system here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SYNCHRONIZATION CONTRACT
 * ============================================================================
 *
 * A shared-data synchronization operation represents a semantic ordering or
 * visibility requirement.
 *
 * It does not select:
 *
 *     mutex
 *     barrier
 *     fence
 *     event
 *     semaphore
 *     network acknowledgement
 *     hardware signal
 *
 * The concurrency/scheduling/runtime layers choose an implementation capable
 * of preserving the semantic contract.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. CONSISTENCY CONTRACT
 * ============================================================================
 *
 * Consistency is represented through policy metadata rather than hard-coded
 * implementation choices.
 *
 * Possible semantic values may include:
 *
 *     strong
 *     causal
 *     eventual
 *     session
 *     ordered
 *     application_defined
 *
 * The actual supported vocabulary belongs to the data/semantic specification.
 *
 * This grammar deliberately does not enumerate it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. ORDERING CONTRACT
 * ============================================================================
 *
 * Shared data may require:
 *
 *     ordered visibility
 *     unordered visibility
 *
 * but this does not imply a particular execution scheduler.
 *
 * The scheduler must preserve semantic ordering only where required.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. COPY / MOVE / BORROW CONTRACT
 * ============================================================================
 *
 * The source-level shared-data boundary MUST NOT assume copying.
 *
 * Semantic analysis may select:
 *
 *     copy
 *     move
 *     borrow
 *     reference
 *     view
 *     stream
 *
 * subject to:
 *
 *     type rules
 *     ownership rules
 *     quantum rules
 *     effects
 *     capabilities
 *     resource constraints
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. SECURITY CONTRACT
 * ============================================================================
 *
 * Shared data does not automatically grant access.
 *
 * A declaration cannot silently authorize:
 *
 *     filesystem access
 *     network access
 *     hardware access
 *     device access
 *     remote execution
 *     secret access
 *
 * Security/capability analysis remains authoritative.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. PRIVACY CONTRACT
 * ============================================================================
 *
 * Shared-data syntax may participate in privacy policy, but does not implement
 * cryptography.
 *
 * It must not silently imply:
 *
 *     encryption
 *     decryption
 *     anonymization
 *     secure transport
 *     trusted execution
 *
 * Those are semantic/security/interoperability concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements attached to shared data remain symbolic and scalable.
 *
 * Valid conceptual relationships include:
 *
 *     memory >= required_memory
 *
 *     bandwidth >= required_bandwidth
 *
 *     capability("shared.memory")
 *
 *     capability("quantum.measurement")
 *
 *     capability("accelerator.compute")
 *
 * The grammar must never translate such requirements into:
 *
 *     fixed device IDs
 *     fixed node counts
 *     fixed memory capacities
 *     fixed qubit counts
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST remain free of:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It must also contain no equivalent indirect restrictions such as:
 *
 *     exactly_8_domains
 *     exactly_32_values
 *     device_0
 *     gpu_0
 *     qpu_0
 *     node_0
 *
 * as universal language constructs.
 *
 * A numeric value appearing in a user expression is allowed because it may be
 * program data.
 *
 * Example:
 *
 *     shared data batch_size = 1024;
 *
 * is valid source semantics.
 *
 * A grammar-level rule saying that batch_size cannot exceed 1024 is not.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. SCALABILITY CONTRACT
 * ============================================================================
 *
 * Shared data syntax is structurally unbounded.
 *
 * The grammar imposes no limit on:
 *
 *     number of shared values
 *     number of domain crossings
 *     number of consumers
 *     number of producers
 *     number of policies
 *     number of synchronization boundaries
 *     number of data dependencies
 *     number of domains
 *     expression size
 *     collection size
 *     stream size
 *     tensor dimensions
 *     distributed participants
 *
 * Practical limits belong to the parser/compiler/runtime environment and must
 * not become language semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. FUTURE DOMAIN EXTENSIBILITY
 * ============================================================================
 *
 * A future domain can participate without modifying the fundamental shared-data
 * model.
 *
 * Examples:
 *
 *     photonic
 *     neuromorphic
 *     analog
 *     optical
 *     molecular
 *     biological
 *     cryogenic
 *     probabilistic
 *     reconfigurable
 *     future_domain
 *
 * A new domain supplies:
 *
 *     semantic domain identity
 *     type/effect rules
 *     capability model
 *     AST semantic interpretation
 *     IR lowering
 *     compiler integration
 *
 * It does not require a new shared-data architecture.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source
 *     language version
 *     lexical vocabulary
 *     grammar
 *
 * Parsing must NOT depend on:
 *
 *     hardware
 *     device availability
 *     runtime state
 *     network state
 *     current time
 *     randomness
 *     environment variables
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     missing shared keyword
 *     missing data keyword
 *     missing identifier
 *     malformed initializer
 *     malformed domain
 *     missing source domain
 *     missing destination domain
 *     malformed policy
 *     malformed synchronization
 *     malformed expression
 *     missing delimiter
 *
 * Diagnostics must preserve source spans.
 *
 * Semantic diagnostics remain downstream and should distinguish:
 *
 *     type error
 *     ownership error
 *     illegal quantum sharing
 *     unavailable capability
 *     unsatisfied resource requirement
 *     illegal domain crossing
 *     invalid synchronization requirement
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. ERROR RECOVERY
 * ============================================================================
 *
 * The parser should recover at stable shared-data boundaries such as:
 *
 *     SEMICOLON
 *     RBRACE
 *
 * where the canonical parser recovery strategy permits.
 *
 * Recovery must never invent:
 *
 *     values
 *     types
 *     domains
 *     devices
 *     resources
 *     quantum operations
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes semantic shared-data information after AST creation.
 *
 * The pipeline is:
 *
 *     SharedData parse tree
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *     classical semantics  quantum semantics
 *                              |
 *                              v
 *                          quantum::ir
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     target realization
 *
 * No compiler backend should consume this grammar directly as its semantic
 * model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * In particular, it must not introduce:
 *
 *     SharedDataIR
 *     QuantumSharedDataIR
 *     AcceleratorSharedDataIR
 *     PhysicalSharedBufferIR
 *
 * merely because those would be convenient.
 *
 * The semantic model must use the repository's canonical IR boundaries.
 *
 * Quantum portions ultimately use:
 *
 *     quantum::ir
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may realize shared data through:
 *
 *     local references
 *     shared memory
 *     message passing
 *     remote memory
 *     accelerator transfers
 *     zero-copy views
 *     serialization
 *     streams
 *     other valid mechanisms
 *
 * The runtime must preserve source semantics.
 *
 * It must not reinterpret a preference as a requirement or silently discard a
 * mandatory sharing/visibility constraint.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Shared-data dependencies are inputs to scheduling.
 *
 * The scheduler determines:
 *
 *     execution ordering
 *     synchronization
 *     overlap
 *     resource use
 *     transfer timing
 *
 * This grammar does not assign physical time.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. ROUTING CONTRACT
 * ============================================================================
 *
 * If shared data crosses physical resources, routing may determine:
 *
 *     communication path
 *     physical connection
 *     transport
 *     placement
 *
 * The grammar does not encode the path.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. QEC / ZQN CONTRACT
 * ============================================================================
 *
 * Shared data may carry semantic information associated with quantum
 * computation.
 *
 * It must not define:
 *
 *     error-correction code
 *     code distance
 *     syndrome algorithm
 *     decoder
 *     noise model
 *     fault model
 *
 * QEC owns error correction.
 *
 * ZQN owns quantum noise/fault semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is a new modular component.
 *
 * It must not silently change the meaning of existing generic `shared`
 * constructs elsewhere in the repository.
 *
 * Migration policy:
 *
 *     old shared-data syntax
 *          |
 *          v
 *     compatibility mapping
 *          |
 *          v
 *     sharedDataConstruct
 *
 * Existing syntax should retain its documented meaning.
 *
 * Any incompatible change requires language-version review.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     shared data value;
 *
 *     shared data value: T;
 *
 *     shared data value: T = expression;
 *
 *     shared data publish value to quantum;
 *
 *     shared data consume result from quantum;
 *
 *     shared data transfer value from classical to quantum;
 *
 *     shared data transfer value from quantum to classical;
 *
 *     shared data synchronize value;
 *
 *     shared data policy consistency = expression;
 *
 *     shared data capability qualified::name;
 *
 *     shared data requires expression;
 *
 * Required cross-domain cases:
 *
 *     classical -> quantum
 *     quantum -> classical
 *     classical -> accelerator
 *     accelerator -> classical
 *     quantum -> accelerator
 *     accelerator -> quantum
 *     software -> HDL
 *     HDL -> software
 *     local -> distributed
 *     distributed -> local
 *     AI -> classical
 *     classical -> AI
 *
 * Required negative cases:
 *
 *     missing identifier
 *     missing destination domain
 *     missing source domain
 *     missing assignment expression
 *     malformed policy
 *     malformed synchronization
 *     malformed transfer
 *     malformed delimiter
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. BOUNDARY TESTS
 * ============================================================================
 *
 * Boundary tests must cover:
 *
 *     one shared value
 *     many shared values
 *     many consumers
 *     many producers
 *     nested hybrid regions
 *     repeated domain crossings
 *     deeply nested expressions
 *     large symbolic resource expressions
 *     long qualified names
 *
 * No test is allowed to establish an artificial universal maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. SCALABILITY TESTS
 * ============================================================================
 *
 * The same source semantics must remain valid for:
 *
 *     tiny computation
 *     embedded computation
 *     single-machine computation
 *     multicore computation
 *     GPU computation
 *     FPGA computation
 *     accelerator computation
 *     quantum computation
 *     heterogeneous computation
 *     distributed computation
 *     cluster computation
 *     cloud computation
 *     future computational substrates
 *
 * provided that the target satisfies the semantic requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. DETERMINISM TESTS
 * ============================================================================
 *
 * Identical source + identical language version must produce identical:
 *
 *     token classification
 *     parse structure
 *
 * independently of:
 *
 *     machine size
 *     available hardware
 *     runtime resource inventory
 *     deployment topology
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. HARD-CODING TESTS
 * ============================================================================
 *
 * Automated validation should scan this file for:
 *
 *     MAX_*
 *     fixed device identifiers
 *     fixed resource counts
 *     fixed topology
 *     fixed memory capacity
 *     fixed qubit capacity
 *     fixed accelerator count
 *
 * Any finding must be classified as:
 *
 *     language semantic
 *     explicit user constraint
 *     target constraint
 *     implementation limit
 *     test fixture
 *     accidental hard-coding
 *
 * Accidental hard-coding fails validation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * No unsafe Rust is required.
 *
 * Repository Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * This grammar does not create any Rust dependency.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. INTEGRATION WITH HYBRID.G4
 * ============================================================================
 *
 * The canonical hybrid composition grammar should import:
 *
 *     SharedData
 *
 * and add exactly one shared-data entry to its public hybrid composition:
 *
 *     | sharedDataConstruct
 *
 * or, if the repository chooses to keep extended forms behind the same
 * boundary:
 *
 *     | sharedDataConstruct
 *     | sharedDataExtendedConstruct
 *
 * Preferred production choice:
 *
 *     | sharedDataConstruct
 *
 * The extended forms should be promoted into the stable public surface only
 * after their token vocabulary and semantic contracts are finalized.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. INTEGRATION WITH ZAMANIPARSER.G4
 * ============================================================================
 *
 * No direct import is required here.
 *
 * The dependency direction remains:
 *
 *     SharedData
 *          |
 *          v
 *       Hybrid
 *          |
 *          v
 *     ZamaniParser
 *
 * This prevents SharedData from becoming another universal parser root.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 60. TOKEN CONTRACT
 * ============================================================================
 *
 * This grammar expects the canonical lexer vocabulary to provide the following
 * stable lexical concepts:
 *
 *     K_SHARED
 *     K_DATA
 *     K_PUBLISH
 *     K_CONSUME
 *     K_TRANSFER
 *     K_SYNCHRONIZE
 *     K_POLICY
 *     K_BOUNDARY
 *     K_REQUIRES
 *     K_CAPABILITY
 *     K_PREFER
 *     K_TO
 *     K_FROM
 *
 * together with canonical:
 *
 *     IDENTIFIER
 *     ASSIGN
 *     SEMICOLON
 *     LPAREN
 *     RPAREN
 *
 * and the normal expression/type/name vocabulary.
 *
 * IMPORTANT:
 *
 * These tokens must be verified against:
 *
 *     grammar/lexer/tokens.g4
 *
 * before ANTLR generation.
 *
 * If a token does not yet exist, it belongs in the canonical lexer vocabulary,
 * NOT as a lexer rule in this file.
 *
 * The implementation must not create a second lexical authority here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 61. TOKEN-MIGRATION RULE
 * ============================================================================
 *
 * If the current canonical lexer does not yet expose one of the K_* tokens,
 * the integration task is:
 *
 *     grammar/lexer/keywords.g4
 *          |
 *          v
 *     grammar/lexer/tokens.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     SharedData
 *
 * Do NOT solve missing tokens by adding:
 *
 *     SHARED : 'shared';
 *     DATA : 'data';
 *
 * to this parser grammar.
 *
 * That would violate the single-lexer invariant.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 62. NO DUPLICATE DATA GRAMMAR
 * ============================================================================
 *
 * `shared-data.g4` must not import or duplicate:
 *
 *     grammar/data/data.g4
 *     grammar/data/records.g4
 *     grammar/data/collections.g4
 *     grammar/data/streams.g4
 *     grammar/data/transformations.g4
 *     grammar/data/serialization.g4
 *
 * Data semantics are referenced through:
 *
 *     expression
 *     typeExpression
 *     qualifiedName
 *
 * and semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 63. NO DUPLICATE MEMORY GRAMMAR
 * ============================================================================
 *
 * Do not duplicate:
 *
 *     ownership
 *     borrowing
 *     allocation
 *     memory regions
 *     address spaces
 *     persistence
 *
 * Shared-data semantics may refer to them through semantic attributes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 64. NO DUPLICATE CONCURRENCY GRAMMAR
 * ============================================================================
 *
 * Synchronization here is a semantic boundary.
 *
 * It must not duplicate:
 *
 *     mutex
 *     channel
 *     actor
 *     task
 *     future
 *     barrier
 *     semaphore
 *
 * Those remain concurrency/effects concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 65. NO VENDOR LOCK-IN
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     CUDA
 *     ROCm
 *     OpenCL
 *     vendor QPU API
 *     vendor FPGA primitive
 *     vendor accelerator ID
 *     cloud-provider identifier
 *
 * Such syntax belongs to interoperability/dialect layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 66. FINAL INVARIANTS
 * ============================================================================
 *
 * Invariant 1
 *
 * Shared data describes logical semantic relationships.
 *
 * Invariant 2
 *
 * Shared data does not select physical hardware.
 *
 * Invariant 3
 *
 * Shared data does not allocate memory.
 *
 * Invariant 4
 *
 * Shared data does not perform transport.
 *
 * Invariant 5
 *
 * Shared data does not schedule execution.
 *
 * Invariant 6
 *
 * Shared data does not perform routing.
 *
 * Invariant 7
 *
 * Shared data does not implement QEC.
 *
 * Invariant 8
 *
 * Shared data does not implement ZQN.
 *
 * Invariant 9
 *
 * Shared data does not create another quantum IR.
 *
 * Invariant 10
 *
 * Quantum semantics ultimately reach quantum::ir.
 *
 * Invariant 11
 *
 * Domain names remain extensible.
 *
 * Invariant 12
 *
 * Resource quantities remain expressions.
 *
 * Invariant 13
 *
 * Requirements remain distinct from preferences.
 *
 * Invariant 14
 *
 * Logical sharing does not imply physical copying.
 *
 * Invariant 15
 *
 * Parsing remains deterministic.
 *
 * Invariant 16
 *
 * Hardware availability cannot change parse meaning.
 *
 * Invariant 17
 *
 * No universal machine-size limit exists.
 *
 * Invariant 18
 *
 * No unsafe Rust is required.
 *
 * Invariant 19
 *
 * Rust 1.97 / 1.97.1 remains supported.
 *
 * Invariant 20
 *
 * Existing grammar ownership remains authoritative.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 67. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] Canonical lexer tokens have been verified.
 *
 * [ ] SharedData imports successfully into Hybrid.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] No duplicate expression grammar exists here.
 *
 * [ ] No duplicate type grammar exists here.
 *
 * [ ] No duplicate data grammar exists here.
 *
 * [ ] No duplicate memory grammar exists here.
 *
 * [ ] No duplicate concurrency grammar exists here.
 *
 * [ ] No physical resource assumptions exist here.
 *
 * [ ] AST mapping is documented.
 *
 * [ ] Semantic mapping is documented.
 *
 * [ ] IR mapping is documented.
 *
 * [ ] quantum::ir integration is documented.
 *
 * [ ] Compiler integration is documented.
 *
 * [ ] Runtime integration is documented.
 *
 * [ ] Scheduling dependency semantics are documented.
 *
 * [ ] Routing boundary is documented.
 *
 * [ ] QEC boundary is documented.
 *
 * [ ] ZQN boundary is documented.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] Compatibility policy is documented.
 *
 * [ ] Rust 1.97 / 1.97.1 integration passes.
 *
 * [ ] No unsafe Rust is introduced.
 *
 * ============================================================================
 *
 * FINAL PRINCIPLE
 * ============================================================================
 *
 *     Shared Data
 *          =
 *     logical semantic interoperability
 *
 *     NOT
 *
 *     physical data movement.
 *
 * Therefore:
 *
 *     One source
 *          |
 *          v
 *     One semantic shared-data meaning
 *          |
 *          v
 *     Many possible realizations
 *          |
 *          +--> local memory
 *          +--> shared memory
 *          +--> accelerator memory
 *          +--> distributed memory
 *          +--> message passing
 *          +--> zero-copy
 *          +--> quantum/classical boundary
 *          +--> hardware interface
 *          +--> future substrate
 *
 * The implementation chooses the realization subject to the semantic
 * requirements and available resources.
 *
 * This is the shared-data foundation required for:
 *
 *     Zamani — From Atom to Everywhere
 *
 * and:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */