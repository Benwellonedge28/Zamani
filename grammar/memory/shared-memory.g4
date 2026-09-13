/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/shared-memory.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production shared-memory grammar component.
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
 * This file defines the SOURCE-LEVEL SHARED-MEMORY SYNTAX CONTRACT for
 * Zamani.
 *
 * Shared memory is treated as a semantic capability and access model rather
 * than as a particular physical implementation.
 *
 * The syntax may therefore describe shared-memory intent for:
 *
 *     - threads;
 *     - tasks;
 *     - processes;
 *     - accelerators;
 *     - heterogeneous systems;
 *     - NUMA systems;
 *     - unified-memory systems;
 *     - distributed shared-memory systems;
 *     - future memory technologies.
 *
 * This grammar does NOT decide which physical mechanism implements sharing.
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
 *     memory.g4
 *          |
 *          v
 *     shared-memory.g4
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     ownership/aliasing              concurrency
 *        analysis                     analysis
 *          |                               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                  canonical semantic IR
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *         classical      quantum      hardware/
 *            IR            IR          resource IR
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *              optimization / scheduling /
 *                 lowering / execution
 *
 * This grammar remains upstream of all physical realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - shared-memory source-level operation classification;
 *   - shared-memory access intent;
 *   - shared-memory visibility intent;
 *   - shared-memory synchronization intent;
 *   - shared-memory consistency intent;
 *   - shared-memory sharing/unsharing intent;
 *   - shared-memory semantic requirements;
 *   - shared-memory constraints;
 *   - shared-memory preferences;
 *   - shared-memory hints;
 *   - shared-memory extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - identifier syntax;
 *   - general expression syntax;
 *   - general statement syntax;
 *   - memory-place syntax;
 *   - memory-qualified-name syntax;
 *   - ownership checking;
 *   - borrow checking;
 *   - lifetime checking;
 *   - alias analysis;
 *   - race detection;
 *   - synchronization implementation;
 *   - atomic implementation;
 *   - cache coherence;
 *   - NUMA discovery;
 *   - physical memory discovery;
 *   - memory allocation;
 *   - memory deallocation;
 *   - memory layout;
 *   - physical addresses;
 *   - device addresses;
 *   - DMA;
 *   - thread creation;
 *   - process creation;
 *   - task scheduling;
 *   - distributed placement;
 *   - network communication;
 *   - hardware discovery;
 *   - hardware selection;
 *   - backend selection;
 *   - optimization;
 *   - routing;
 *   - quantum routing;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime execution;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR;
 *   - hardware IR.
 *
 * ============================================================================
 * CANONICAL MEMORY BOUNDARY
 * ============================================================================
 *
 * The canonical memory foundation is:
 *
 *     grammar/memory/memory.g4
 *
 * That grammar already owns the common memory-domain constructs used by
 * specialized memory grammars, including:
 *
 *     memoryPlace
 *     memoryQualifiedName
 *     memoryPlaceArgumentList
 *     memoryOperation
 *     memorySharing
 *
 * This file MUST NOT redefine those common rules.
 *
 * Instead, this file owns the shared-memory-specific refinement rules below.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST consume the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It MUST NOT define lexer rules.
 *
 * It MUST NOT introduce permanent global lexer tokens merely to represent
 * implementation-specific shared-memory technologies.
 *
 * Existing lexical vocabulary is preferred.
 *
 * ============================================================================
 * OPEN-WORLD KEYWORD POLICY
 * ============================================================================
 *
 * Shared memory must remain open-ended.
 *
 * The grammar MUST NOT require dedicated lexer tokens for every possible
 * sharing technology, synchronization technology, consistency model, or
 * hardware architecture.
 *
 * Examples of concepts that may be represented semantically include:
 *
 *     cache_coherent
 *     noncoherent
 *     numa
 *     unified
 *     accelerator_shared
 *     process_shared
 *     task_shared
 *     transactional
 *     persistent
 *     distributed_shared
 *     future_memory_domain
 *
 * Such concepts should normally be represented through the existing memory
 * qualified-name and metadata mechanisms rather than a closed lexer list.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The primary public rules are:
 *
 *     sharedMemory
 *     sharedMemoryOperation
 *     sharedMemoryAction
 *     sharedMemoryTarget
 *     sharedMemoryOptions
 *     sharedMemoryOption
 *     sharedMemoryAccess
 *     sharedMemoryVisibility
 *     sharedMemoryConsistency
 *     sharedMemorySynchronization
 *     sharedMemoryRequirement
 *     sharedMemoryConstraint
 *     sharedMemoryPreference
 *     sharedMemoryHint
 *     sharedMemoryExtension
 *
 * These rules are intentionally semantic-domain-specific while relying on
 * common memory syntax supplied by memory.g4.
 *
 * ============================================================================
 * SHARED MEMORY MODEL
 * ============================================================================
 *
 * Shared memory means that multiple execution contexts may be given semantic
 * access to the same memory object/place according to the program's declared
 * intent and downstream semantic rules.
 *
 * The grammar does NOT imply:
 *
 *     - cache coherence;
 *     - atomicity;
 *     - sequential consistency;
 *     - mutual exclusion;
 *     - lock ownership;
 *     - visibility timing;
 *     - a particular CPU;
 *     - a particular accelerator;
 *     - shared physical RAM.
 *
 * Those are separate semantic properties.
 *
 * ============================================================================
 * SHARING ACTIONS
 * ============================================================================
 *
 * The source-level sharing action vocabulary is intentionally small:
 *
 *     share
 *     unshare
 *     shared
 *
 * These represent intent.
 *
 * They do not directly invoke runtime operations.
 *
 * `shared` may be used as a semantic declaration/requirement describing that
 * a memory value or place participates in a shared-memory domain.
 *
 * `share` expresses an operation that establishes or requests sharing.
 *
 * `unshare` expresses an operation that ends or requests the ending of a
 * sharing relationship.
 *
 * Whether an explicit `unshare` is legal is determined by semantic analysis.
 *
 * ============================================================================
 * MEMORY TARGET
 * ============================================================================
 *
 * Shared-memory operations consume the canonical memory-place abstraction.
 *
 * The target may therefore be any source-level memory place supported by
 * memory.g4.
 *
 * Examples conceptually include:
 *
 *     x
 *     object.field
 *     values[index]
 *     object.field[index]
 *
 * The complete place grammar remains owned by the memory foundation and
 * expression system.
 *
 * This file MUST NOT create a competing place grammar.
 *
 * ============================================================================
 * ACCESS INTENT
 * ============================================================================
 *
 * Shared memory may expose one or more access intents:
 *
 *     read
 *     write
 *     read_write
 *
 * These names are semantic identifiers where the canonical lexer does not
 * reserve dedicated tokens.
 *
 * The grammar must preserve the distinction without determining whether the
 * target hardware supports the requested access mode.
 *
 * ============================================================================
 * VISIBILITY INTENT
 * ============================================================================
 *
 * Visibility is separate from synchronization.
 *
 * A source program may express visibility requirements such as:
 *
 *     visible
 *     eventually_visible
 *     immediately_visible
 *
 * The grammar must preserve these as source-level intent.
 *
 * Semantic analysis determines whether a requested visibility model is valid
 * and how it can be implemented.
 *
 * ============================================================================
 * CONSISTENCY INTENT
 * ============================================================================
 *
 * Consistency is separate from visibility and synchronization.
 *
 * A shared-memory operation may carry a consistency model represented by a
 * qualified name.
 *
 * Examples include:
 *
 *     relaxed
 *     acquire_release
 *     sequential
 *     transactional
 *     domain_specific
 *
 * These names are not physical machine promises.
 *
 * Semantic analysis and target lowering determine whether the requested model
 * can be implemented.
 *
 * ============================================================================
 * SYNCHRONIZATION INTENT
 * ============================================================================
 *
 * Synchronization requirements are represented independently from sharing.
 *
 * A shared-memory construct may request synchronization semantics without
 * specifying a concrete implementation.
 *
 * Examples of semantic synchronization domains include:
 *
 *     barrier
 *     fence
 *     atomic
 *     lock
 *     semaphore
 *     event
 *     channel
 *     transactional
 *
 * This grammar does not implement any of them.
 *
 * The actual synchronization construct remains owned by the appropriate
 * concurrency/effects subsystem when the syntax represents an executable
 * synchronization primitive.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * This file MUST NOT become a second concurrency grammar.
 *
 * In particular it MUST NOT define:
 *
 *     task
 *     thread
 *     actor
 *     future
 *     channel
 *     mutex
 *     barrier implementation
 *     scheduler
 *
 * merely because shared memory may interact with those concepts.
 *
 * Shared-memory syntax expresses memory-domain intent.
 *
 * Concurrency semantics remain downstream.
 *
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * A shared-memory requirement means that a requested semantic property must
 * be satisfied by compilation/execution.
 *
 * It does NOT identify:
 *
 *     a processor;
 *     a NUMA node;
 *     a device;
 *     an address;
 *     a memory bank;
 *     a physical topology.
 *
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict the legal implementation space.
 *
 * They remain distinct from requirements.
 *
 * For example, a source program may semantically constrain sharing behavior
 * without naming the physical implementation.
 *
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences express desirable implementation characteristics.
 *
 * They are not guarantees.
 *
 * A compiler/backend may select another implementation when required for
 * correctness or target compatibility.
 *
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints are non-binding guidance.
 *
 * A compiler, scheduler, runtime, or backend may ignore a hint.
 *
 * This file must never make a hint equivalent to a semantic requirement.
 *
 * ============================================================================
 * EXTENSION MODEL
 * ============================================================================
 *
 * Future shared-memory models must be representable without changing the
 * foundational memory grammar.
 *
 * Examples:
 *
 *     memory::shared::persistent(...)
 *     memory::shared::transactional(...)
 *     memory::shared::accelerator(...)
 *     memory::shared::distributed(...)
 *     memory::shared::future(...)
 *
 * The actual qualified-name representation remains owned by memory.g4.
 *
 * No fixed maximum qualification depth is imposed here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar contains no machine-size assumptions.
 *
 * It MUST NOT encode:
 *
 *     MAX_SHARED_MEMORY
 *     MAX_SHARED_OBJECTS
 *     MAX_SHARED_REGIONS
 *     MAX_SHARERS
 *     MAX_THREADS
 *     MAX_PROCESSES
 *     MAX_NODES
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_ADDRESSES
 *     MAX_MEMORY_SIZE
 *
 * It also MUST NOT encode:
 *
 *     fixed addresses;
 *     fixed memory banks;
 *     fixed NUMA nodes;
 *     fixed devices;
 *     fixed topology;
 *     fixed accelerator counts.
 *
 * Scaling is determined downstream by:
 *
 *     available resources;
 *     semantic validity;
 *     capabilities;
 *     compiler policies;
 *     runtime policies;
 *     deployment constraints.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Shared memory may participate in:
 *
 *     classical computing
 *     quantum-classical control
 *     hybrid computing
 *     accelerator computing
 *     HDL/hardware co-design
 *     distributed computing
 *     AI/ML
 *     scientific computing
 *     embedded systems
 *     future computational domains
 *
 * The grammar does not enumerate those domains.
 *
 * A new domain must not require changes to this grammar merely because its
 * values can participate in shared-memory semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST representation of a shared-memory construct must preserve:
 *
 *     - source span;
 *     - action;
 *     - target/place;
 *     - access intent;
 *     - visibility intent;
 *     - consistency intent;
 *     - synchronization intent;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - extension metadata;
 *     - original qualified operation name where applicable.
 *
 * The AST MUST NOT contain a physical allocation merely because a shared
 * memory construct was parsed.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the target;
 *     - checking ownership;
 *     - checking borrowing;
 *     - checking lifetimes;
 *     - checking aliasing;
 *     - checking race conditions;
 *     - checking synchronization requirements;
 *     - checking consistency requirements;
 *     - checking visibility requirements;
 *     - checking memory-space compatibility;
 *     - checking effect permissions;
 *     - checking resource capabilities;
 *     - determining whether sharing is legal;
 *     - determining whether unsharing is legal;
 *     - determining whether a target supports the requested model.
 *
 * The parser MUST NOT perform these checks.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar does not create an IR.
 *
 * The semantic lowering layer converts the AST into the repository's canonical
 * semantic representation.
 *
 * Shared-memory semantics must remain representable without introducing a
 * duplicate quantum IR.
 *
 * If a shared-memory operation concerns quantum data or logical quantum
 * resources, semantic lowering is responsible for determining the appropriate
 * representation before integration with `quantum::ir`.
 *
 * This grammar must never depend directly on `quantum::ir`.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical memory operations may lower into the repository's canonical
 * classical semantic representation.
 *
 * Shared-memory syntax does not assume:
 *
 *     CPU count;
 *     thread count;
 *     cache hierarchy;
 *     vector width;
 *     memory capacity.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum programs may contain classical shared-memory state used for:
 *
 *     measurement results;
 *     control decisions;
 *     parameter management;
 *     orchestration;
 *     hybrid execution.
 *
 * This grammar does not define quantum semantics.
 *
 * It must not:
 *
 *     allocate qubits;
 *     identify physical qubits;
 *     select a QPU;
 *     define coupling topology;
 *     define quantum synchronization;
 *     define QEC;
 *     define ZQN behavior.
 *
 * Those remain owned by the corresponding quantum subsystems.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware realization may map shared-memory intent onto:
 *
 *     shared RAM;
 *     coherent memory;
 *     non-coherent memory plus synchronization;
 *     unified memory;
 *     accelerator memory;
 *     distributed shared memory;
 *     other future mechanisms.
 *
 * Such decisions are downstream.
 *
 * The grammar must not name physical hardware unless the name itself is
 * explicitly part of the source program's semantic contract.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed shared-memory systems may require:
 *
 *     replication;
 *     coherence protocols;
 *     consistency protocols;
 *     migration;
 *     synchronization;
 *     communication.
 *
 * None of these implementations belong here.
 *
 * The distributed subsystem consumes semantic information produced downstream.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Shared-memory access may interact with:
 *
 *     permissions;
 *     capabilities;
 *     isolation;
 *     trust domains;
 *     information-flow constraints.
 *
 * This grammar preserves source-level metadata but does not perform security
 * analysis.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Parsing MUST NEVER:
 *
 *     allocate memory;
 *     map memory;
 *     unmap memory;
 *     create synchronization objects;
 *     inspect hardware;
 *     inspect the operating system;
 *     access physical addresses;
 *     contact another process;
 *     contact a network;
 *     execute a backend.
 *
 * Runtime interpretation occurs only after semantic validation and lowering.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar must contain:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no runtime actions;
 *     no environment access;
 *     no hardware discovery;
 *     no generated identifiers;
 *     no nondeterministic parser actions.
 *
 * Equivalent source must produce equivalent parse structure.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains grammar only.
 *
 * Generated Rust integration must be compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * The generated/runtime frontend implementation must enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing memory syntax must remain parseable.
 *
 * In particular, the existing memory foundation documents shared-memory
 * operations such as:
 *
 *     memory::shared(...)
 *     memory::share(...)
 *     memory::unshare(...)
 *
 * This component must preserve their intended source compatibility while
 * moving shared-memory-specific ownership into this file.
 *
 * `memory.g4` remains the compatibility-facing aggregator.
 *
 * ============================================================================
 * INTEGRATION WITH memory.g4
 * ============================================================================
 *
 * The current memory foundation already exposes:
 *
 *     memorySharing
 *
 * as part of its memory-domain operation structure.
 *
 * After this file is introduced, `memory.g4` should retain the aggregator
 * rule but delegate shared-memory-specific syntax to this component.
 *
 * Dependency direction:
 *
 *     memory.g4
 *          |
 *          +--> shared-memory.g4
 *
 * shared-memory.g4 MUST NOT depend on memory.g4.
 *
 * If the ANTLR build architecture does not permit parser-grammar imports,
 * the same ownership boundary must be implemented through the project's
 * established parser composition mechanism rather than by duplicating rules.
 *
 * ============================================================================
 * REQUIRED memory.g4 MIGRATION
 * ============================================================================
 *
 * The existing `memorySharing` rule must no longer contain the full
 * shared-memory-specific grammar.
 *
 * It should become an aggregation/bridge to:
 *
 *     sharedMemory
 *
 * while the canonical common rules remain in memory.g4:
 *
 *     memoryQualifiedName
 *     memoryPlace
 *     memoryPlaceArgumentList
 *
 * No second definition of those rules should be introduced here.
 *
 * ============================================================================
 * EXAMPLE SOURCE FOR TESTING
 * ============================================================================
 *
 * The following examples are conceptual test cases. Their exact tokenization
 * must follow the canonical Zamani lexer.
 *
 *     memory::shared(x);
 *
 *     memory::share(x);
 *
 *     memory::unshare(x);
 *
 *     memory::shared(x, read);
 *
 *     memory::shared(x, write);
 *
 *     memory::shared(x, read_write);
 *
 *     memory::shared(x, visibility);
 *
 *     memory::shared(x, consistency);
 *
 *     memory::shared(x, synchronization);
 *
 *     memory::share(x, access);
 *
 *     memory::unshare(x);
 *
 * These examples are semantic fixtures, not hard-coded machine requirements.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser/validation test suite must reject malformed shared-memory syntax,
 * including:
 *
 *     missing target;
 *     missing closing parenthesis;
 *     missing opening parenthesis;
 *     malformed option;
 *     malformed qualified operation;
 *     malformed argument separator;
 *     invalid punctuation;
 *     trailing tokens where the host grammar requires statement termination.
 *
 * Semantic tests, rather than parser tests, must reject:
 *
 *     invalid ownership;
 *     illegal unshare;
 *     incompatible lifetime;
 *     illegal aliasing;
 *     unsupported consistency requirement;
 *     unsupported synchronization requirement;
 *     insufficient capability;
 *     unavailable resources.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     one shared value;
 *     many shared values;
 *     deeply qualified memory domains;
 *     deeply nested expressions where supported by the host grammar;
 *     very large argument lists;
 *     very large programs;
 *     zero implementation assumptions;
 *     heterogeneous execution;
 *     distributed execution.
 *
 * No test may encode a production limit such as:
 *
 *     32 shared values;
 *     64 shared values;
 *     1024 shared values.
 *
 * Any test fixture size is a test parameter, not a grammar restriction.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required integration tests include:
 *
 *     classical + shared memory
 *     classical + concurrency + shared memory
 *     quantum + classical shared state
 *     quantum + shared classical control
 *     hybrid + shared memory
 *     hardware + shared memory
 *     HDL + software shared state
 *     distributed + shared memory
 *     AI + shared tensors/data
 *     accelerator + shared memory
 *
 * These tests must verify syntax composition, not hardware implementation.
 *
 * ============================================================================
 * SCALABILITY AUDIT
 * ============================================================================
 *
 * The grammar must permit an unbounded number of syntactically valid shared
 * memory constructs, subject only to parser/tool/runtime resource limits.
 *
 * There must be:
 *
 *     no fixed shared-memory count;
 *     no fixed sharer count;
 *     no fixed target count;
 *     no fixed nesting limit encoded by the grammar;
 *     no fixed device count;
 *     no fixed node count;
 *     no fixed thread count;
 *     no fixed process count;
 *     no fixed memory capacity.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Every numeric literal introduced during future modification must be reviewed
 * and classified as one of:
 *
 *     semantic constant;
 *     lexical constant;
 *     test fixture;
 *     implementation limit;
 *     target-specific limit;
 *     accidental hard-coding.
 *
 * Accidental machine/resource limits MUST NOT enter this grammar.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling must be able to:
 *
 *     syntax-highlight shared-memory constructs;
 *     produce source spans;
 *     format shared-memory syntax;
 *     inspect parse trees;
 *     serialize/deserialize AST representation;
 *     report diagnostics without executing memory operations.
 *
 * ============================================================================
 * DOCUMENTATION INTEGRATION
 * ============================================================================
 *
 * Documentation must describe:
 *
 *     sharing intent;
 *     access intent;
 *     visibility;
 *     consistency;
 *     synchronization;
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     portability.
 *
 * Documentation must explicitly state that shared-memory syntax does not imply
 * a particular physical memory implementation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 *   [ ] shared-memory syntax ownership is unambiguous;
 *   [ ] no common memory rule is duplicated;
 *   [ ] no lexer rules are introduced;
 *   [ ] no physical resource is hard-coded;
 *   [ ] memory-place syntax comes from the canonical memory foundation;
 *   [ ] general expression syntax is not duplicated;
 *   [ ] shared-memory semantics are downstream from parsing;
 *   [ ] memory.g4 integration is complete;
 *   [ ] existing memory::shared syntax remains compatible;
 *   [ ] memory::share remains compatible;
 *   [ ] memory::unshare remains compatible;
 *   [ ] AST requirements are documented;
 *   [ ] semantic requirements are documented;
 *   [ ] classical integration is documented;
 *   [ ] quantum integration is documented;
 *   [ ] hardware integration is documented;
 *   [ ] distributed integration is documented;
 *   [ ] security integration is documented;
 *   [ ] runtime boundaries are enforced;
 *   [ ] deterministic parsing is verified;
 *   [ ] positive tests exist;
 *   [ ] negative tests exist;
 *   [ ] boundary tests exist;
 *   [ ] cross-domain tests exist;
 *   [ ] scalability tests exist;
 *   [ ] compatibility tests exist;
 *   [ ] Rust 1.97/1.97.1 generated integration succeeds;
 *   [ ] generated Rust uses no unsafe code;
 *   [ ] documentation and grammar agree;
 *   [ ] grammar dependency direction remains acyclic.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PUBLIC SHARED-MEMORY ENTRY POINT
 * ============================================================================
 *
 * This is the reusable shared-memory grammar component.
 *
 * The surrounding memory grammar is responsible for determining where a
 * shared-memory construct is legal.
 */
sharedMemory
    : sharedMemoryOperation
    ;


/*
 * ============================================================================
 * SHARED-MEMORY OPERATION
 * ============================================================================
 *
 * The operation name is supplied through the canonical memory qualified-name
 * system.
 *
 * The operation itself is intentionally open-world so that future memory
 * domains can be introduced without changing the foundational lexer.
 */
sharedMemoryOperation
    : memoryQualifiedName
      LPAREN
      sharedMemoryArguments?
      RPAREN
    ;


/*
 * ============================================================================
 * SHARED-MEMORY ARGUMENTS
 * ============================================================================
 *
 * The first argument, when present, identifies the source-level memory target.
 *
 * Remaining arguments describe shared-memory intent.
 *
 * The common memory place grammar remains owned by memory.g4.
 */
sharedMemoryArguments
    : sharedMemoryTarget
      (
          COMMA
          sharedMemoryOption
      )*
    ;


/*
 * ============================================================================
 * SHARED-MEMORY TARGET
 * ============================================================================
 *
 * Do not redefine memoryPlace here.
 *
 * This bridge deliberately consumes the canonical memory-place rule.
 */
sharedMemoryTarget
    : memoryPlace
    ;


/*
 * ============================================================================
 * SHARED-MEMORY OPTIONS
 * ============================================================================
 */
sharedMemoryOptions
    : sharedMemoryOption+
    ;


/*
 * ============================================================================
 * SHARED-MEMORY OPTION
 * ============================================================================
 *
 * The option vocabulary is intentionally open.
 *
 * A qualified memory-domain name can represent future semantic properties
 * without introducing a new lexer keyword for each property.
 */
sharedMemoryOption
    : sharedMemoryAccess
    | sharedMemoryVisibility
    | sharedMemoryConsistency
    | sharedMemorySynchronization
    | sharedMemoryRequirement
    | sharedMemoryConstraint
    | sharedMemoryPreference
    | sharedMemoryHint
    | sharedMemoryExtension
    ;


/*
 * ============================================================================
 * ACCESS INTENT
 * ============================================================================
 *
 * Access names are identifiers resolved by semantic analysis.
 *
 * The grammar does not introduce machine-specific access mechanisms.
 */
sharedMemoryAccess
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * VISIBILITY INTENT
 * ============================================================================
 */
sharedMemoryVisibility
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * CONSISTENCY INTENT
 * ============================================================================
 */
sharedMemoryConsistency
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * SYNCHRONIZATION INTENT
 * ============================================================================
 */
sharedMemorySynchronization
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * The requirement itself remains represented by the memory-domain qualified
 * naming system.
 */
sharedMemoryRequirement
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 */
sharedMemoryConstraint
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 */
sharedMemoryPreference
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 */
sharedMemoryHint
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * EXTENSION
 * ============================================================================
 *
 * Future shared-memory semantics remain open through qualified names.
 */
sharedMemoryExtension
    : memoryQualifiedName
    ;


/*
 * ============================================================================
 * SHARED-MEMORY ACTION
 * ============================================================================
 *
 * This rule provides a semantic classification bridge for the established
 * memory-sharing operations.
 *
 * The canonical memory namespace remains the owner of the qualified name.
 *
 * Semantic analysis maps the operation name to one of:
 *
 *     shared
 *     share
 *     unshare
 *
 * or an extension.
 *
 * The grammar does not execute the operation.
 */
sharedMemoryAction
    : memoryQualifiedName
    ;