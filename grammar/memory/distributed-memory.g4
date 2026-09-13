/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/distributed-memory.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar component
 *
 * Status:
 *     Production distributed-memory grammar component.
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only
 *     No unsafe code
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL DISTRIBUTED-MEMORY SYNTAX CONTRACT
 * for Zamani.
 *
 * Distributed memory is represented as computation intent rather than as
 * a description of a particular machine, cluster, interconnect, operating
 * system, NUMA topology, accelerator, provider, or network.
 *
 * The grammar therefore describes:
 *
 *     - distributed-memory intent;
 *     - distributed-memory operations;
 *     - logical memory domains;
 *     - replication intent;
 *     - migration intent;
 *     - remote-access intent;
 *     - consistency intent;
 *     - visibility intent;
 *     - ownership/transfer intent;
 *     - placement requirements;
 *     - resource requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - extension points.
 *
 * It does NOT describe how those semantics are physically implemented.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     core/parser composition
 *          |
 *          v
 *     grammar/memory/memory.g4
 *          |
 *          v
 *     distributed-memory.g4
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
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     memory semantics       distributed semantics
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *              canonical semantic IR
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *      classical    quantum      hardware
 *         IR          IR          IR
 *          |           |           |
 *          +-----------+-----------+
 *                      |
 *                      v
 *          routing / scheduling / optimization
 *                      |
 *                      v
 *              distributed runtime
 *
 * IMPORTANT:
 *
 * This grammar is upstream of:
 *
 *     - distributed placement;
 *     - node discovery;
 *     - transport selection;
 *     - network routing;
 *     - scheduling;
 *     - replication implementation;
 *     - consistency implementation;
 *     - hardware selection;
 *     - backend selection;
 *     - runtime execution.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed-memory operation classification;
 *     - distributed-memory semantic intent;
 *     - distributed-memory operation structure;
 *     - distributed-memory access intent;
 *     - distributed-memory replication intent;
 *     - distributed-memory migration intent;
 *     - distributed-memory remote-access intent;
 *     - distributed-memory consistency intent;
 *     - distributed-memory visibility intent;
 *     - distributed-memory placement intent;
 *     - distributed-memory resource requirements;
 *     - distributed-memory constraints;
 *     - distributed-memory preferences;
 *     - distributed-memory hints;
 *     - distributed-memory extension points.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifier spelling;
 *     - general expressions;
 *     - general statements;
 *     - general declarations;
 *     - general type syntax;
 *     - memory-place syntax;
 *     - memory-qualified-name syntax;
 *     - ownership checking;
 *     - borrow checking;
 *     - lifetime checking;
 *     - alias analysis;
 *     - allocation;
 *     - deallocation;
 *     - physical addresses;
 *     - virtual addresses;
 *     - node discovery;
 *     - node identifiers;
 *     - cluster topology;
 *     - network topology;
 *     - network routing;
 *     - transport protocols;
 *     - RPC;
 *     - message delivery;
 *     - serialization implementation;
 *     - replication algorithms;
 *     - consensus algorithms;
 *     - cache coherence;
 *     - distributed coherence implementation;
 *     - distributed garbage collection;
 *     - distributed scheduling;
 *     - placement algorithms;
 *     - load balancing;
 *     - fault detection;
 *     - fault recovery;
 *     - resilience policy;
 *     - hardware discovery;
 *     - backend selection;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - canonical classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - hardware IR;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL MEMORY BOUNDARY
 * ============================================================================
 *
 * The common memory-domain syntax is owned by:
 *
 *     grammar/memory/memory.g4
 *
 * This file MUST consume the memory foundation rather than redefine it.
 *
 * The common memory foundation owns concepts such as:
 *
 *     memoryPlace
 *     memoryQualifiedName
 *     memoryPlaceArgumentList
 *     memoryOperation
 *     memorySharing
 *
 * Specialized distributed-memory rules in this file refine those concepts.
 *
 * This file MUST NOT create competing definitions of:
 *
 *     memory place
 *     general expression
 *     general type
 *     lifetime
 *     ownership
 *     allocation
 *     generic resource expressions
 *
 * ============================================================================
 * PARSER COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser-grammar component consumed by the repository's
 * grammar composition layer.
 *
 * The composition layer is responsible for supplying the canonical:
 *
 *     lexer vocabulary;
 *     expression rules;
 *     type rules;
 *     memory foundation rules;
 *     source-location information;
 *     AST construction.
 *
 * If the repository's ANTLR composition mechanism uses imported parser
 * grammars, this component is imported by that composition layer.
 *
 * If the repository instead combines grammar fragments during generation,
 * these rules are incorporated there without duplicating their ownership.
 *
 * This file therefore MUST NOT introduce a second lexer.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer remains responsible for all lexical definitions.
 *
 * This grammar MUST NOT define lexer rules.
 *
 * Distributed-memory concepts remain open-world wherever possible.
 *
 * A future distributed-memory domain must not require adding a new global
 * lexer keyword merely because a new technology appears.
 *
 * For example, the following semantic domains may be represented through
 * qualified identifiers/metadata without changing this grammar:
 *
 *     cluster
 *     shard
 *     partition
 *     remote
 *     replicated
 *     persistent
 *     transactional
 *     accelerator
 *     quantum
 *     future_domain
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * Distributed memory MUST scale according to available resources.
 *
 * The grammar contains no fixed limits for:
 *
 *     nodes;
 *     processes;
 *     tasks;
 *     shards;
 *     replicas;
 *     partitions;
 *     memory regions;
 *     addresses;
 *     devices;
 *     links;
 *     machines;
 *     clusters;
 *     data volume;
 *     memory capacity;
 *     communication capacity.
 *
 * It MUST NOT define:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_SHARDS
 *     MAX_PARTITIONS
 *     MAX_REMOTE_MEMORY
 *     MAX_DISTRIBUTED_MEMORY
 *     MAX_CLUSTER_SIZE
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *
 * Nor may it encode:
 *
 *     node 0
 *     node 1
 *     device 0
 *     fixed cluster size
 *     fixed topology
 *     fixed address width
 *     fixed memory capacity.
 *
 * A program may describe an arbitrary number of distributed resources,
 * subject only to the parser/compiler/runtime resources and semantic
 * requirements available to the execution environment.
 *
 * ============================================================================
 * SEMANTIC MODEL
 * ============================================================================
 *
 * Distributed memory means that a memory object or memory domain may have
 * logically distributed realization across execution resources.
 *
 * The source language describes the semantic property.
 *
 * It does NOT imply:
 *
 *     one process per node;
 *     one memory object per machine;
 *     one network hop;
 *     one transport protocol;
 *     one consistency algorithm;
 *     one replication algorithm;
 *     one placement strategy;
 *     one serialization format;
 *     one physical memory technology.
 *
 * Those are implementation decisions.
 *
 * ============================================================================
 * PRIMARY PUBLIC RULES
 * ============================================================================
 *
 *     distributedMemory
 *     distributedMemoryOperation
 *     distributedMemoryAction
 *     distributedMemoryTarget
 *     distributedMemoryOptions
 *     distributedMemoryOption
 *     distributedMemoryAccess
 *     distributedMemoryConsistency
 *     distributedMemoryVisibility
 *     distributedMemoryReplication
 *     distributedMemoryMigration
 *     distributedMemoryPlacement
 *     distributedMemoryRequirement
 *     distributedMemoryConstraint
 *     distributedMemoryPreference
 *     distributedMemoryHint
 *     distributedMemoryExtension
 *
 * These rules intentionally describe distributed-memory semantics without
 * implementing them.
 *
 * ============================================================================
 * DISTRIBUTED MEMORY ROOT
 * ============================================================================
 *
 * A distributed-memory construct has one logical target and zero or more
 * semantic options.
 *
 * The target remains a source-level memory place.
 *
 * This is essential for POCO-REAF:
 *
 *     source memory identity
 *
 * must remain independent from:
 *
 *     physical location.
 *
 * ============================================================================
 */

distributedMemory
    : distributedMemoryOperation
    ;

/*
 * A distributed-memory operation is represented by a qualified memory
 * operation plus an optional distributed-memory option list.
 *
 * The exact operation name remains extensible.
 *
 * This allows future domains without requiring grammar changes for every
 * new distributed-memory technology.
 */
distributedMemoryOperation
    : memoryQualifiedName
      LPAREN
      distributedMemoryArguments?
      RPAREN
    ;

/*
 * Distributed-memory arguments are intentionally delegated to the existing
 * expression/type infrastructure.
 *
 * The first argument is conventionally the logical memory target.
 *
 * Semantic analysis, rather than this grammar, determines whether the
 * operation actually requires a target, a value, a region, or another
 * semantic object.
 */
distributedMemoryArguments
    : distributedMemoryArgument
      (COMMA distributedMemoryArgument)*
    ;

distributedMemoryArgument
    : expression
    ;

/*
 * Semantic action classification.
 *
 * The action name is intentionally an identifier rather than a closed
 * enumeration of every future distributed-memory operation.
 *
 * Examples of semantic names include:
 *
 *     distributed
 *     remote
 *     replicate
 *     migrate
 *     partition
 *     shard
 *     gather
 *     scatter
 *     publish
 *     unpublish
 *
 * Classification is semantic analysis responsibility.
 */
distributedMemoryAction
    : memoryQualifiedName
    ;

/*
 * A distributed-memory target is always a logical memory place.
 *
 * It is never a physical node/address/device.
 */
distributedMemoryTarget
    : memoryPlace
    ;

/*
 * Options are deliberately extensible.
 *
 * Each option carries a semantic key and an expression value.
 *
 * Examples:
 *
 *     consistency = relaxed
 *     visibility = eventual
 *     replication = desired
 *     placement = locality
 *     reliability = required
 *
 * None of these selects a physical implementation.
 */
distributedMemoryOptions
    : distributedMemoryOption
      (COMMA distributedMemoryOption)*
    ;

distributedMemoryOption
    : IDENTIFIER
      ASSIGN
      expression
    ;

/*
 * Distributed-memory access intent.
 *
 * Access mode is distinct from placement, consistency, and replication.
 *
 * Semantic examples include:
 *
 *     read
 *     write
 *     read_write
 *     remote_read
 *     remote_write
 *
 * These are semantic identifiers, not physical operations.
 */
distributedMemoryAccess
    : memoryQualifiedName
    ;

/*
 * Consistency is intentionally independent of replication and visibility.
 *
 * The grammar accepts an open semantic name.
 *
 * Examples:
 *
 *     relaxed
 *     eventual
 *     causal
 *     strong
 *     sequential
 *     transactional
 *
 * The grammar does not promise that every target supports every model.
 */
distributedMemoryConsistency
    : memoryQualifiedName
    ;

/*
 * Visibility is independent from consistency.
 *
 * Examples:
 *
 *     local
 *     remote
 *     eventual
 *     immediate
 *     explicit
 *
 * The semantic layer validates whether the requested model is implementable.
 */
distributedMemoryVisibility
    : memoryQualifiedName
    ;

/*
 * Replication intent.
 *
 * The grammar expresses semantic intent, not a fixed replica count.
 *
 * A count, when present in source, is an expression rather than a grammar
 * constant and is therefore subject to normal semantic/resource validation.
 *
 * A program may also request qualitative replication without specifying a
 * count.
 */
distributedMemoryReplication
    : memoryQualifiedName
      (LPAREN distributedMemoryReplicationArguments? RPAREN)?
    ;

distributedMemoryReplicationArguments
    : distributedMemoryReplicationArgument
      (COMMA distributedMemoryReplicationArgument)*
    ;

distributedMemoryReplicationArgument
    : expression
    ;

/*
 * Migration intent.
 *
 * Migration identifies a semantic transition of the logical memory object.
 *
 * It does not select:
 *
 *     a machine;
 *     a node;
 *     an address;
 *     a network;
 *     a transport;
 *     a scheduler.
 */
distributedMemoryMigration
    : memoryQualifiedName
      LPAREN
      distributedMemoryArguments?
      RPAREN
    ;

/*
 * Placement intent is intentionally symbolic.
 *
 * Examples may include semantic policies such as:
 *
 *     locality
 *     affinity
 *     proximity
 *     balanced
 *     topology_aware
 *     energy_aware
 *
 * Actual placement remains owned by resource/hardware/runtime systems.
 */
distributedMemoryPlacement
    : memoryQualifiedName
    ;

/*
 * A requirement is a mandatory semantic property.
 *
 * Requirements are not device selections.
 */
distributedMemoryRequirement
    : REQUIREMENT
      distributedMemoryOption
    ;

/*
 * A constraint restricts legal implementation choices.
 *
 * It is not equivalent to a preference or hint.
 */
distributedMemoryConstraint
    : CONSTRAINT
      distributedMemoryOption
    ;

/*
 * A preference describes a desired implementation property but does not
 * become a correctness requirement.
 */
distributedMemoryPreference
    : PREFERENCE
      distributedMemoryOption
    ;

/*
 * A hint provides non-binding implementation guidance.
 */
distributedMemoryHint
    : HINT
      distributedMemoryOption
    ;

/*
 * Open-world extension point.
 *
 * Future distributed-memory semantics may be introduced through qualified
 * names without changing the foundational memory grammar.
 */
distributedMemoryExtension
    : memoryQualifiedName
      LPAREN
      distributedMemoryArguments?
      RPAREN
    ;

/*
 * ============================================================================
 * STRUCTURAL INTEGRATION RULE
 * ============================================================================
 *
 * A host grammar SHOULD integrate distributed-memory statements through the
 * memory-domain composition layer rather than adding a second statement
 * hierarchy here.
 *
 * Conceptually:
 *
 *     memoryStatement
 *         |
 *         +--> ownership
 *         +--> borrowing
 *         +--> allocation
 *         +--> deallocation
 *         +--> shared memory
 *         +--> distributed memory
 *         +--> memory constraints
 *
 * This file owns only the distributed-memory branch.
 *
 * ============================================================================
 * MEMORY SAFETY
 * ============================================================================
 *
 * Distributed memory syntax MUST NOT imply that an ordinary reference can
 * safely outlive its owner.
 *
 * Distributed access must continue to obey:
 *
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     aliasing;
 *     synchronization;
 *     capability;
 *     effect;
 *     resource
 *
 * semantics defined elsewhere.
 *
 * A remote location is not automatically a valid borrow.
 *
 * A replicated location is not automatically independently mutable.
 *
 * A migrated object does not automatically change source-level ownership.
 *
 * Those are semantic-analysis decisions.
 *
 * ============================================================================
 * OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * grammar/memory/ownership.g4 owns ownership syntax.
 *
 * This file MUST NOT redefine ownership modes.
 *
 * Distributed-memory operations may consume ownership-qualified values, but
 * whether a transfer is legal is determined by ownership analysis.
 *
 * ============================================================================
 * BORROWING INTEGRATION
 * ============================================================================
 *
 * grammar/memory/borrowing.g4 owns borrowing syntax.
 *
 * This file MUST NOT redefine:
 *
 *     &
 *     &mut
 *     borrow types
 *     borrow lifetimes
 *
 * A remote operation may semantically reject a borrow if its lifetime,
 * ownership, transport, or execution model cannot guarantee validity.
 *
 * ============================================================================
 * LIFETIME INTEGRATION
 * ============================================================================
 *
 * grammar/memory/lifetimes.g4 owns lifetime syntax.
 *
 * Distributed execution MUST NOT introduce an implicit infinite lifetime.
 *
 * The grammar merely preserves lifetime information supplied by the source.
 *
 * ============================================================================
 * ALLOCATION INTEGRATION
 * ============================================================================
 *
 * grammar/memory/allocation.g4 owns allocation syntax.
 *
 * Distributed placement does not itself allocate memory.
 *
 * A distributed-memory request may require allocation during semantic
 * lowering, but the allocation implementation belongs to the allocation and
 * resource systems.
 *
 * ============================================================================
 * DEALLOCATION INTEGRATION
 * ============================================================================
 *
 * grammar/memory/deallocation.g4 owns deallocation syntax.
 *
 * Distributed release may require coordination, but coordination belongs to
 * the distributed runtime/resource implementation.
 *
 * ============================================================================
 * SHARED MEMORY INTEGRATION
 * ============================================================================
 *
 * grammar/memory/shared-memory.g4 owns shared-memory semantics.
 *
 * Distributed shared memory may therefore be represented by combining:
 *
 *     shared-memory semantics
 *
 * with:
 *
 *     distributed-memory semantics.
 *
 * Neither grammar should duplicate the other's rules.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Distributed memory may interact with:
 *
 *     tasks;
 *     futures;
 *     actors;
 *     channels;
 *     parallel execution;
 *     synchronization.
 *
 * Those constructs remain owned by:
 *
 *     grammar/concurrency/
 *
 * This file does not define them.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Distributed-memory resource requirements are interpreted by:
 *
 *     grammar/resources/
 *
 * Resource semantics may consider:
 *
 *     memory capacity;
 *     bandwidth;
 *     latency;
 *     reliability;
 *     locality;
 *     energy;
 *     scalability;
 *     portability.
 *
 * This grammar does not decide whether resources exist.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Physical hardware information remains owned by:
 *
 *     grammar/hardware/
 *
 * and downstream hardware abstraction layers.
 *
 * This file must never encode:
 *
 *     CPU identifiers;
 *     GPU identifiers;
 *     FPGA identifiers;
 *     QPU identifiers;
 *     NUMA node identifiers;
 *     physical addresses;
 *     interconnect identifiers.
 *
 * ============================================================================
 * DISTRIBUTED SYSTEM INTEGRATION
 * ============================================================================
 *
 * Distributed execution semantics are consumed by:
 *
 *     grammar/distributed/
 *
 * and subsequently by semantic analysis/runtime infrastructure.
 *
 * The grammar should permit distributed-memory intent to coexist with:
 *
 *     node;
 *     service;
 *     remote execution;
 *     replication;
 *     consistency;
 *     fault tolerance;
 *     placement.
 *
 * Those concepts must remain owned by their respective subsystems.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed memory may participate in hybrid quantum-classical execution.
 *
 * Examples include:
 *
 *     measurement result distribution;
 *     parameter distribution;
 *     classical control state;
 *     distributed quantum simulation state;
 *     orchestration metadata.
 *
 * This file does NOT define:
 *
 *     qubits;
 *     quantum gates;
 *     physical qubits;
 *     quantum topology;
 *     QEC;
 *     ZQN;
 *     quantum routing.
 *
 * If a distributed-memory construct ultimately affects quantum computation,
 * semantic lowering decides how that information reaches the canonical
 * quantum semantic boundary.
 *
 * The grammar MUST NOT depend directly on `quantum::ir`.
 *
 * ============================================================================
 * HDL / HARDWARE CO-DESIGN
 * ============================================================================
 *
 * Distributed memory may be used by hardware/software co-design programs.
 *
 * This grammar does not define:
 *
 *     wires;
 *     clocks;
 *     ports;
 *     registers;
 *     hardware state machines.
 *
 * Those remain owned by:
 *
 *     grammar/hdl/
 *
 * and the corresponding hardware IR.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Distributed memory can support:
 *
 *     tensor partitioning;
 *     dataset sharding;
 *     distributed training;
 *     model replication;
 *     distributed inference.
 *
 * This grammar does not define AI or data semantics.
 *
 * Those remain owned by:
 *
 *     grammar/ai/
 *     grammar/data/
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * A distributed-memory construct must not implicitly grant:
 *
 *     remote access;
 *     network access;
 *     cross-domain authority;
 *     data publication;
 *     replication permission;
 *     remote mutation permission.
 *
 * Capability and security analysis determine whether the requested operation
 * is authorized.
 *
 * Security semantics remain owned by:
 *
 *     grammar/security/
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no generated random identifiers;
 *     - no environment-dependent parsing;
 *     - no provider calls.
 *
 * Parsing is therefore deterministic with respect to the source and grammar.
 *
 * ============================================================================
 * SECURITY / PARSER PURITY
 * ============================================================================
 *
 * Parsing a distributed-memory construct MUST NEVER:
 *
 *     allocate runtime memory;
 *     access distributed memory;
 *     connect to a node;
 *     contact a provider;
 *     inspect a cluster;
 *     discover hardware;
 *     perform RPC;
 *     perform network I/O;
 *     perform serialization;
 *     invoke a scheduler;
 *     invoke a runtime;
 *     invoke an allocator.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST representation must preserve at least:
 *
 *     - source span;
 *     - qualified operation name;
 *     - logical target;
 *     - operation arguments;
 *     - access intent;
 *     - consistency intent;
 *     - visibility intent;
 *     - replication intent;
 *     - migration intent;
 *     - placement intent;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - extension metadata.
 *
 * The AST MUST NOT silently resolve:
 *
 *     node;
 *     address;
 *     device;
 *     network;
 *     transport;
 *     backend;
 *     topology.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *     1. target existence;
 *     2. target type;
 *     3. ownership;
 *     4. borrowing;
 *     5. lifetime;
 *     6. aliasing;
 *     7. mutability;
 *     8. distributed-access legality;
 *     9. consistency requirements;
 *     10. visibility requirements;
 *     11. replication requirements;
 *     12. migration legality;
 *     13. placement requirements;
 *     14. resource requirements;
 *     15. capability requirements;
 *     16. security/effect requirements;
 *     17. cross-domain compatibility;
 *     18. target capability satisfaction.
 *
 * None of these checks belongs in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file does NOT define an IR.
 *
 * The semantic lowering layer converts distributed-memory AST nodes into the
 * repository's canonical semantic representation.
 *
 * Distributed-memory semantics must remain independent of:
 *
 *     parser implementation;
 *     physical address representation;
 *     network implementation;
 *     provider API;
 *     runtime implementation.
 *
 * ============================================================================
 * NO DUPLICATE QUANTUM IR
 * ============================================================================
 *
 * This grammar must never create:
 *
 *     DistributedQuantumIR
 *     QuantumMemoryIR
 *     QuantumMemoryGate
 *
 * merely to represent distributed memory.
 *
 * If a distributed-memory operation affects quantum computation, its semantic
 * information is lowered through the repository's established semantic
 * pipeline and, where appropriate, ultimately into `quantum::ir`.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use distributed-memory information for:
 *
 *     partitioning;
 *     placement;
 *     replication;
 *     communication planning;
 *     synchronization;
 *     memory movement;
 *     optimization;
 *     scheduling.
 *
 * These are compiler decisions.
 *
 * The grammar only records source intent.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may realize distributed-memory semantics using any
 * supported mechanism, including future mechanisms.
 *
 * The grammar makes no assumption about:
 *
 *     RPC;
 *     MPI;
 *     shared-memory transport;
 *     message passing;
 *     RDMA;
 *     provider APIs;
 *     cloud storage;
 *     distributed object stores;
 *     quantum-network transport;
 *     custom interconnects.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing memory syntax remains owned by memory.g4.
 *
 * Adding distributed-memory syntax must not invalidate programs that do not
 * use distributed-memory constructs.
 *
 * Unknown future qualified distributed-memory operations must remain capable
 * of being diagnosed as semantic/extension errors rather than causing the
 * entire foundational memory grammar to require redesign.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is no grammar-level maximum for:
 *
 *     operation count;
 *     argument count;
 *     logical memory objects;
 *     regions;
 *     partitions;
 *     replicas;
 *     nodes;
 *     distributed domains;
 *     source-program size.
 *
 * Practical limits are imposed only by:
 *
 *     source representation;
 *     parser implementation;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_SHARDS
 *     MAX_PARTITIONS
 *     MAX_MEMORY
 *     MAX_DISTRIBUTED_MEMORY
 *     MAX_REMOTE_MEMORY
 *     MAX_DEVICES
 *     MAX_LINKS
 *     MAX_CLUSTER_SIZE
 *
 * Forbidden:
 *
 *     node0
 *     node1
 *     device0
 *     address0
 *     address1
 *
 * Forbidden:
 *
 *     fixed cluster topology;
 *     fixed replica count;
 *     fixed memory capacity;
 *     fixed machine count;
 *     fixed network count.
 *
 * Any actual resource limit belongs downstream to:
 *
 *     resource policy;
 *     capability model;
 *     target description;
 *     runtime configuration;
 *     deployment policy.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     - basic distributed-memory operation;
 *     - remote access;
 *     - replication intent;
 *     - migration intent;
 *     - partition/sharding intent;
 *     - consistency intent;
 *     - visibility intent;
 *     - placement intent;
 *     - resource requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - qualified extension operations;
 *     - nested expressions;
 *     - generic memory places;
 *     - cross-domain memory usage.
 *
 * Negative tests MUST cover:
 *
 *     - malformed distributed operations;
 *     - malformed argument lists;
 *     - malformed options;
 *     - missing targets where required semantically;
 *     - malformed qualified names;
 *     - invalid option syntax;
 *     - accidental physical-address syntax when prohibited;
 *     - invalid composition with unrelated memory syntax.
 *
 * Boundary tests MUST cover:
 *
 *     - one distributed object;
 *     - arbitrarily long argument lists within parser-resource limits;
 *     - deeply qualified extension names within parser-resource limits;
 *     - large source programs;
 *     - large numbers of independent distributed-memory constructs.
 *
 * Scalability tests MUST verify that parsing does not contain artificial
 * limits corresponding to:
 *
 *     nodes;
 *     replicas;
 *     shards;
 *     partitions;
 *     memory capacity;
 *     device count;
 *     cluster size.
 *
 * Determinism tests MUST verify identical parse structure for identical source.
 *
 * Cross-domain tests MUST include at least:
 *
 *     classical + distributed memory
 *     quantum + distributed memory
 *     quantum + classical + distributed memory
 *     HDL + distributed memory
 *     hardware + distributed memory
 *     AI + distributed memory
 *     data + distributed memory
 *     distributed + security
 *     distributed + concurrency
 *     distributed + resources
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] It defines distributed-memory syntax only.
 *     [ ] It consumes the canonical memory foundation.
 *     [ ] It does not redefine memory places.
 *     [ ] It does not redefine expressions.
 *     [ ] It does not redefine types.
 *     [ ] It does not define lexer rules.
 *     [ ] It has no physical machine assumptions.
 *     [ ] It has no fixed distributed-resource limits.
 *     [ ] It has no runtime behavior.
 *     [ ] It has no hardware discovery.
 *     [ ] It has no network I/O.
 *     [ ] It has no provider-specific semantics.
 *     [ ] It does not duplicate quantum::ir.
 *     [ ] It integrates with ownership.
 *     [ ] It integrates with borrowing.
 *     [ ] It integrates with lifetimes.
 *     [ ] It integrates with allocation/deallocation.
 *     [ ] It integrates with shared memory.
 *     [ ] It integrates with concurrency.
 *     [ ] It integrates with resources.
 *     [ ] It integrates with distributed execution.
 *     [ ] It integrates with security.
 *     [ ] It supports classical/quantum/hardware cross-domain use.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Rust-generated integration remains compatible with Rust 1.97/1.97.1.
 *     [ ] No unsafe implementation is required.
 *
 * ============================================================================
 */