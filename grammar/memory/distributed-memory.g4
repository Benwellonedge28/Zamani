/*
 * Zamani — Distributed Memory Grammar
 * File: grammar/memory/distributed-memory.g4
 *
 * ========================================================================
 * FILE CONTRACT
 * ========================================================================
 *
 * Purpose
 * -------
 * Defines portable source syntax for expressing distributed-memory intent.
 *
 * Distributed memory means that a logical memory object may be represented
 * across independently addressable execution/memory domains. The language
 * describes WHAT distribution means to the program, not HOW a compiler or
 * runtime realizes it.
 *
 * Owns
 * ----
 * - distributed-memory declarations/specifications
 * - distributed-memory operations
 * - logical distribution targets
 * - partition/sharding intent
 * - replication intent
 * - migration intent
 * - distribution consistency intent
 * - distribution placement intent
 * - communication intent associated with distributed memory
 * - fault/recovery intent associated with distributed memory
 * - distributed-memory requirements
 * - distributed-memory constraints
 * - distributed-memory preferences
 * - distributed-memory hints
 * - open-world distributed-memory extensions/metadata
 *
 * Does NOT own
 * -------------
 * - lexical definitions
 * - identifiers
 * - general expressions
 * - general types
 * - ownership checking
 * - borrowing checking
 * - lifetime checking
 * - allocation/deallocation semantics
 * - physical memory addresses
 * - physical node identifiers
 * - fixed node counts
 * - fixed replica counts
 * - fixed shard counts
 * - network topology
 * - routing algorithms
 * - placement algorithms
 * - scheduling
 * - communication implementation
 * - persistence implementation
 * - distributed consensus implementation
 * - runtime behavior
 * - hardware selection
 * - accelerator realization
 * - quantum hardware realization
 * - canonical IR
 * - quantum::ir
 * - QEC
 * - ZQN
 * - HAL
 *
 * Architectural boundary
 * ----------------------
 *
 * Source
 *   -> ZamaniLexer
 *   -> Zamani parser/composition root
 *   -> domain-neutral AST
 *   -> semantic/resource/effect analysis
 *   -> canonical semantic model
 *   -> canonical IR
 *   -> optimization / partitioning / placement / replication /
 *      communication / scheduling / resilience
 *   -> target-specific realization
 *
 * DistributedMemory is therefore a SOURCE-SYNTAX CONTRACT only.
 *
 * ========================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ========================================================================
 *
 * This grammar MUST NOT impose physical limits.
 *
 * Forbidden as language limits:
 *
 *   MAX_NODES
 *   MAX_REPLICAS
 *   MAX_SHARDS
 *   MAX_PARTITIONS
 *   MAX_MEMORY
 *   MAX_NETWORK_LINKS
 *   MAX_PROCESSES
 *   MAX_WORKERS
 *   MAX_DEVICES
 *   NODE_0 ... NODE_N
 *   REPLICA_0 ... REPLICA_N
 *
 * Counts expressed by a program are DATA or semantic requirements and are
 * therefore valid. A grammar/compiler implementation limit is not a
 * language semantic limit.
 *
 * Example of portable intent:
 *
 *   distribute data across domains
 *   replication factor = desired_replication
 *
 * The value may be determined at compile time, deployment time, or runtime.
 *
 * The language MUST NOT require a programmer to identify physical machines,
 * NICs, memory banks, sockets, NUMA nodes, network routes, or device IDs.
 *
 * ========================================================================
 * TOKEN POLICY
 * ========================================================================
 *
 * This grammar intentionally adds no lexer keywords.
 *
 * Existing generic identifiers are used as open-world semantic names.
 * This prevents distributed-memory concepts from becoming an ever-growing
 * hard-coded keyword list and permits future capabilities without changing
 * the lexical vocabulary.
 *
 * Canonical repository convention:
 *
 *   tokenVocab = ZamaniLexer
 *   IDENT      = canonical identifier token
 *
 * If the repository's canonical lexer evolves, token changes belong in the
 * canonical lexer/token contract, never in this parser grammar.
 *
 * ========================================================================
 * IMPORT POLICY
 * ========================================================================
 *
 * Memory is the canonical memory-domain foundation.
 *
 * Names and Expressions are shared language foundations. This grammar does
 * not duplicate identifier, qualified-name, or expression syntax.
 *
 * ========================================================================
 * AST CONTRACT
 * ========================================================================
 *
 * Every accepted construct maps to domain-neutral AST data representing:
 *
 *   operation/name
 *   targets
 *   arguments
 *   options
 *   requirements
 *   constraints
 *   preferences
 *   hints
 *   metadata/extensions
 *   source spans
 *
 * No AST node produced by this grammar may encode:
 *
 *   a physical node ID
 *   a physical address
 *   a hardware-specific replica
 *   a fixed cluster topology
 *
 * Generic operation/attribute structures should be preferred where the
 * existing AST already provides them.
 *
 * ========================================================================
 * SEMANTIC CONTRACT
 * ========================================================================
 *
 * Semantic analysis is responsible for determining whether a distributed
 * memory specification is meaningful and compatible with:
 *
 *   types
 *   ownership
 *   borrowing
 *   lifetimes
 *   memory regions
 *   resources
 *   capabilities
 *   effects
 *   concurrency
 *   distributed execution
 *   communication
 *   resilience
 *
 * The grammar only establishes structural validity.
 *
 * Requirement != capability != constraint != preference != hint
 *
 * These distinctions MUST be preserved downstream.
 *
 * ========================================================================
 * IR CONTRACT
 * ========================================================================
 *
 * This grammar does not define a distributed-memory IR.
 *
 * Distributed-memory semantics lower through the repository's canonical
 * semantic/IR architecture. Classical, distributed, accelerator, HDL, and
 * quantum lowering remain separate downstream responsibilities.
 *
 * If a distributed-memory operation eventually affects quantum execution,
 * the quantum semantic path continues through the existing canonical
 * quantum::ir boundary. This file MUST NOT introduce another quantum IR.
 *
 * ========================================================================
 * COMPILER / RUNTIME CONTRACT
 * ========================================================================
 *
 * Compiler responsibilities:
 *   - validate semantic requirements
 *   - determine feasible realizations
 *   - lower logical distribution
 *   - choose/coordinate partitioning and placement
 *   - perform communication and synchronization lowering
 *   - preserve observable semantics
 *
 * Runtime responsibilities:
 *   - discover actual resources
 *   - realize placement
 *   - execute communication
 *   - manage failures/recovery
 *   - monitor capabilities
 *
 * Hardware/backend responsibilities:
 *   - actual memory/network/device realization
 *
 * ========================================================================
 * DIAGNOSTICS CONTRACT
 * ========================================================================
 *
 * Diagnostics must preserve source spans from the parsed construct.
 *
 * Examples of semantic diagnostics:
 *
 *   invalid distribution policy
 *   incompatible consistency requirement
 *   impossible capability requirement
 *   conflicting placement constraints
 *   unsupported migration policy
 *   invalid replication specification
 *   incompatible memory type
 *
 * These are semantic errors, not parser keyword errors.
 *
 * ========================================================================
 * TEST CONTRACT
 * ========================================================================
 *
 * Positive:
 *   - logical distribution
 *   - partitioning
 *   - replication
 *   - migration
 *   - consistency
 *   - placement intent
 *   - communication intent
 *   - fault/recovery intent
 *   - requirements
 *   - constraints
 *   - preferences
 *   - hints
 *   - arbitrary user-defined extensions
 *
 * Negative:
 *   - malformed argument lists
 *   - malformed options
 *   - missing targets
 *   - malformed assignments
 *   - malformed nested specifications
 *
 * Boundary:
 *   - empty option sets where permitted
 *   - nested specifications
 *   - symbolic values
 *   - expression-valued policies
 *   - qualified names
 *
 * Scalability:
 *   - one logical memory object
 *   - arbitrary number of partitions
 *   - arbitrary number of logical replicas
 *   - arbitrary number of logical domains
 *   - dynamically computed distribution values
 *   - very large qualified names/metadata where supported by the lexer
 *
 * Determinism:
 *   - identical source + identical lexer/parser configuration produces the
 *     same parse structure.
 *
 * Compatibility:
 *   - existing memory syntax remains reusable
 *   - no new lexer keyword is required
 *   - future distributed-memory operations can use the extension mechanism
 *     without invalidating existing syntax
 *
 * ========================================================================
 * HARD-CODING AUDIT
 * ========================================================================
 *
 * No physical resource cardinality, topology, address, device ID, or
 * implementation-specific limit is encoded here.
 *
 * ========================================================================
 * COMPLETION CRITERIA
 * ========================================================================
 *
 * This file is complete when:
 *
 *   [x] It is parser-only.
 *   [x] It uses ZamaniLexer.
 *   [x] It composes with Memory.
 *   [x] It does not duplicate Names/Expressions.
 *   [x] It introduces no mandatory distributed-memory lexer keywords.
 *   [x] It is open-world for future distributed-memory operations.
 *   [x] It distinguishes semantic intent from physical realization.
 *   [x] It has no fixed resource/topology limits.
 *   [x] It preserves downstream AST/semantic/IR ownership.
 *   [x] It preserves quantum::ir as the quantum semantic boundary.
 *   [x] It supports requirements, constraints, preferences and hints.
 *   [x] It supports distribution, partitioning, replication and migration.
 *   [x] It supports consistency and communication intent.
 *   [x] It supports resilience/recovery intent.
 *   [x] It supports extensibility without lexer expansion.
 *
 * ========================================================================
 */

parser grammar DistributedMemory;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Memory is the canonical memory-domain foundation.
 *
 * Names and Expressions provide the repository's canonical naming and
 * expression syntax. They must not be duplicated here.
 */
import Memory, Names, Expressions;


/*
 * ========================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================
 *
 * `distributedMemory` is the public rule consumed by the memory grammar
 * composition layer.
 *
 * It intentionally permits both:
 *
 *   a single distributed-memory operation/specification
 *
 * and:
 *
 *   a braced distributed-memory specification containing multiple members.
 *
 * The semantic layer determines whether the selected construct is valid in
 * its surrounding context.
 */
distributedMemory
    : distributedMemoryConstruct
    ;


/*
 * A distributed-memory construct can be either an operation or a
 * declarative specification.
 */
distributedMemoryConstruct
    : distributedMemoryOperation
    | distributedMemorySpecification
    ;


/*
 * ========================================================================
 * DISTRIBUTED-MEMORY OPERATIONS
 * ========================================================================
 *
 * The action name is an IDENT rather than a fixed lexer keyword.
 *
 * This deliberately permits:
 *
 *   distribute(...)
 *   partition(...)
 *   replicate(...)
 *   migrate(...)
 *   rebalance(...)
 *   synchronize(...)
 *   recover(...)
 *
 * and future operations without requiring a new lexer token for every
 * distributed-memory feature.
 *
 * Semantic validation determines which action names are standardized,
 * experimental, dialect-defined, or unknown.
 */
distributedMemoryOperation
    : distributedMemoryAction
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * Open-world action name.
 *
 * A qualified name permits namespace-owned operations without introducing
 * vendor/framework keywords into the core language.
 */
distributedMemoryAction
    : qualifiedName
    ;


/*
 * ========================================================================
 * DECLARATIVE DISTRIBUTED-MEMORY SPECIFICATION
 * ========================================================================
 *
 * A specification is a collection of members. Members use identifiers and
 * expressions rather than a closed enumeration of implementation concepts.
 */
distributedMemorySpecification
    : LBRACE
      distributedMemoryMember*
      RBRACE
    ;


/*
 * A member may be:
 *
 *   action-like
 *   assignment-like
 *   nested specification
 *
 * This gives the semantic layer an extensible representation while keeping
 * syntax deterministic.
 */
distributedMemoryMember
    : distributedMemoryProperty
    | distributedMemoryMemberOperation
    | distributedMemoryNestedMember
    ;


/*
 * Property:
 *
 *   identifier = expression
 *
 * Values may be constants, variables, generic expressions, resource
 * expressions, capability expressions, or values computed later.
 */
distributedMemoryProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * Member operation:
 *
 *   action(...)
 *
 * The action remains open-world.
 */
distributedMemoryMemberOperation
    : distributedMemoryAction
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * Nested specification:
 *
 *   name { ... }
 *
 * This is useful for hierarchical logical distribution descriptions.
 */
distributedMemoryNestedMember
    : identifier
      distributedMemorySpecification
    ;


/*
 * ========================================================================
 * ARGUMENTS
 * ========================================================================
 *
 * Arguments are expression-based so that values can be:
 *
 *   constants
 *   variables
 *   symbolic values
 *   computed values
 *   generic resource expressions
 *   capability expressions
 *   target-independent policies
 */
distributedMemoryArgumentList
    : distributedMemoryArgument
      (COMMA distributedMemoryArgument)*
    ;


distributedMemoryArgument
    : expression
    ;


/*
 * ========================================================================
 * DISTRIBUTED-MEMORY TARGET
 * ========================================================================
 *
 * A target is a logical memory object/name, not a physical address or
 * machine/device identifier.
 *
 * The semantic layer may resolve this against:
 *
 *   memory regions
 *   declarations
 *   allocations
 *   data objects
 *   shared-memory objects
 *   distributed resources
 *
 * It MUST NOT assume that a qualified name is a physical node.
 */
distributedMemoryTarget
    : qualifiedName
    ;


/*
 * ========================================================================
 * DISTRIBUTION
 * ========================================================================
 *
 * Distribution intent describes logical decomposition across execution or
 * memory domains.
 */
distributedMemoryDistribution
    : identifier
      LPAREN
      distributedMemoryTarget
      (COMMA distributedMemoryArgumentList)?
      RPAREN
    ;


/*
 * Explicit logical partitioning/sharding intent.
 *
 * Partition counts, shapes, keys, ranges, and policies are expressions.
 * They are therefore not fixed by this grammar.
 */
distributedMemoryPartition
    : identifier
      LPAREN
      distributedMemoryTarget
      COMMA
      distributedMemoryArgumentList
      RPAREN
    ;


/*
 * ========================================================================
 * REPLICATION
 * ========================================================================
 *
 * Replication is expressed as semantic intent.
 *
 * A replication factor is an expression, not a compile-time grammar limit.
 *
 * Example semantic forms:
 *
 *   replicate(target, factor)
 *   replicate(target, policy)
 *   replicate(target, factor, consistency)
 */
distributedMemoryReplication
    : identifier
      LPAREN
      distributedMemoryTarget
      COMMA
      distributedMemoryArgumentList
      RPAREN
    ;


/*
 * ========================================================================
 * MIGRATION
 * ========================================================================
 *
 * Migration expresses logical movement of distributed state.
 *
 * It does not identify physical machines, memory banks, network routes, or
 * hardware addresses.
 */
distributedMemoryMigration
    : identifier
      LPAREN
      distributedMemoryArgumentList
      RPAREN
    ;


/*
 * ========================================================================
 * PLACEMENT
 * ========================================================================
 *
 * Placement describes logical placement requirements/preferences.
 *
 * Actual node selection, topology mapping, NUMA placement, accelerator
 * selection, and deployment realization are downstream.
 */
distributedMemoryPlacement
    : identifier
      LPAREN
      distributedMemoryTarget
      (COMMA distributedMemoryArgumentList)?
      RPAREN
    ;


/*
 * ========================================================================
 * CONSISTENCY
 * ========================================================================
 *
 * Consistency models are semantic values.
 *
 * The grammar does not enumerate a fixed set such as:
 *
 *   strong
 *   eventual
 *   causal
 *
 * because new models and dialect-defined models must remain possible.
 */
distributedMemoryConsistency
    : identifier
      LPAREN
      distributedMemoryTarget
      (COMMA distributedMemoryArgumentList)?
      RPAREN
    ;


/*
 * ========================================================================
 * COMMUNICATION
 * ========================================================================
 *
 * Communication intent may describe:
 *
 *   synchronization
 *   exchange
 *   transfer
 *   collective
 *   streaming
 *   ordering
 *   latency/bandwidth requirements
 *
 * Actual protocols, links and routes remain downstream.
 */
distributedMemoryCommunication
    : identifier
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
    ;


/*
 * ========================================================================
 * RESILIENCE / RECOVERY
 * ========================================================================
 *
 * Distributed memory can express logical resilience requirements without
 * prescribing an implementation.
 */
distributedMemoryRecovery
    : identifier
      LPAREN
      distributedMemoryTarget
      (COMMA distributedMemoryArgumentList)?
      RPAREN
    ;


/*
 * ========================================================================
 * RESOURCE REQUIREMENTS
 * ========================================================================
 *
 * Requirements are not implementation decisions.
 *
 * Example:
 *
 *   requires capability("distributed.memory")
 *
 * The grammar does not restrict the value to a fixed capability registry.
 */
distributedMemoryRequirement
    : identifier
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
    ;


/*
 * ========================================================================
 * CONSTRAINTS
 * ========================================================================
 *
 * Constraints describe conditions that a valid realization must satisfy.
 */
distributedMemoryConstraint
    : identifier
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
    ;


/*
 * ========================================================================
 * PREFERENCES
 * ========================================================================
 *
 * Preferences guide realization but do not necessarily make a target
 * invalid.
 */
distributedMemoryPreference
    : identifier
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
    ;


/*
 * ========================================================================
 * HINTS
 * ========================================================================
 *
 * Hints are advisory information for compilers/runtimes.
 */
distributedMemoryHint
    : identifier
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
    ;


/*
 * ========================================================================
 * EXTENSIONS
 * ========================================================================
 *
 * Dialects, vendors, research extensions, future distributed-memory
 * mechanisms, and implementation-specific metadata can be represented
 * through qualified names.
 *
 * Such extensions must still declare their semantic/AST/IR compatibility
 * outside this grammar.
 */
distributedMemoryExtension
    : qualifiedName
      LPAREN
      distributedMemoryArgumentList?
      RPAREN
    ;


/*
 * ========================================================================
 * SPECIALIZED DISTRIBUTED-MEMORY SPECIFICATION
 * ========================================================================
 *
 * This rule provides a semantic grouping for the standard conceptual
 * categories without forcing those categories into lexer keywords.
 *
 * The surrounding memory composition layer may use this rule where a
 * distributed-memory-specific clause is expected.
 */
distributedMemoryOption
    : distributedMemoryDistribution
    | distributedMemoryPartition
    | distributedMemoryReplication
    | distributedMemoryMigration
    | distributedMemoryPlacement
    | distributedMemoryConsistency
    | distributedMemoryCommunication
    | distributedMemoryRecovery
    | distributedMemoryRequirement
    | distributedMemoryConstraint
    | distributedMemoryPreference
    | distributedMemoryHint
    | distributedMemoryExtension
    ;


/*
 * A sequence of distributed-memory options.
 */
distributedMemoryOptions
    : distributedMemoryOption*
    ;


/*
 * ========================================================================
 * IDENTIFIER ADAPTER
 * ========================================================================
 *
 * The repository's current memory grammar convention uses IDENT.
 *
 * Do not redefine the canonical identifier syntax here.
 */
identifier
    : IDENT
    ;