/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/partitioning.g4
 *
 * Grammar:
 *     DistributedPartitioning
 *
 * Status:
 *     PRODUCTION DISTRIBUTED-PARTITIONING PARSER COMPONENT
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No mutable parser-global state.
 *     - No unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for DISTRIBUTED PARTITIONING INTENT.
 *
 * Partitioning describes how a logical distributed computation or logical
 * distributed data domain may be divided into independently addressable
 * partitions.
 *
 * The grammar describes semantic intent only.
 *
 * It does NOT implement:
 *
 *     - partitioning algorithms;
 *     - sharding algorithms;
 *     - hash functions;
 *     - range computation;
 *     - placement algorithms;
 *     - load balancing;
 *     - migration;
 *     - replication;
 *     - consistency;
 *     - consensus;
 *     - scheduling;
 *     - routing;
 *     - network transport;
 *     - storage engines;
 *     - resource allocation;
 *     - hardware discovery;
 *     - deployment;
 *     - runtime execution;
 *     - quantum routing;
 *     - quantum error correction;
 *     - ZQN;
 *     - quantum::ir.
 *
 * Those responsibilities belong to downstream semantic, compiler, resource,
 * distributed, storage, placement, routing, scheduling, resilience,
 * networking, runtime, and quantum subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani Source
 *          |
 *          v
 *     Canonical Lexer
 *          |
 *          v
 *     Canonical Parser
 *          |
 *          v
 *     DistributedPartitioning
 *          |
 *          v
 *     Domain-neutral Frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> distributed semantic analysis
 *          +--> security analysis
 *          |
 *          v
 *     Canonical Semantic Representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> resource representation
 *          |
 *          v
 *     optimization
 *          |
 *          +--> partition realization
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> migration
 *          +--> resilience
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime / deployment
 *
 * This grammar never constructs or modifies an IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed partitioning participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Partitioning syntax therefore describes WHAT the program needs or intends,
 * rather than WHICH physical machine, node, device, storage system, or
 * partitioning implementation must be used.
 *
 * The same source program may describe a workload that is ultimately
 * realized on:
 *
 *     - one execution resource;
 *     - one machine;
 *     - multiple machines;
 *     - embedded systems;
 *     - edge systems;
 *     - clusters;
 *     - HPC systems;
 *     - clouds;
 *     - federated systems;
 *     - heterogeneous CPU/GPU/FPGA/ASIC systems;
 *     - quantum-classical systems;
 *     - distributed quantum systems;
 *     - future computational substrates.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Partitioning vocabulary is OPEN-WORLD.
 *
 * The grammar MUST NOT enumerate a closed set such as:
 *
 *     hash
 *     range
 *     list
 *     round_robin
 *     consistent_hash
 *     modulo
 *     rendezvous
 *     directory
 *     geometric
 *
 * as permanent parser alternatives.
 *
 * Those are semantic names.
 *
 * A semantic registry may define:
 *
 *     partition::hash
 *     partition::range
 *     partition::list
 *     partition::custom
 *     vendor::partitioning::future_strategy
 *
 * without requiring a new lexer keyword for every new algorithm.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO finite partitioning capacity.
 *
 * It deliberately contains no:
 *
 *     MAX_PARTITIONS
 *     MAX_SHARDS
 *     MAX_NODES
 *     MAX_REPLICAS
 *     MAX_RANGES
 *     MAX_KEYS
 *     MAX_PARTITION_DEPTH
 *     MAX_PARTITION_LEVELS
 *     MAX_PARTITION_GROUPS
 *     MAX_PARTITION_ENTRIES
 *
 * Nor does it encode:
 *
 *     CPU counts;
 *     GPU counts;
 *     FPGA counts;
 *     QPU counts;
 *     node counts;
 *     memory capacity;
 *     storage capacity;
 *     network capacity;
 *     topology size;
 *     physical device identifiers.
 *
 * Arbitrary source cardinality is represented using ANTLR repetition and
 * recursive structures.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite ceiling.
 *
 * It does NOT claim that a parser, compiler, runtime, operating system, or
 * physical deployment possesses infinite resources.
 *
 * Actual limits are determined downstream by:
 *
 *     semantic validation;
 *     resource availability;
 *     compiler implementation;
 *     scheduler;
 *     deployment;
 *     runtime;
 *     physical hardware.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce universal limits equivalent to:
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
 *     MAX_PARTITIONS
 *     MAX_SHARDS
 *
 * Numeric values appearing in source remain PROGRAM VALUES.
 *
 * For example:
 *
 *     partition_count: desired_partitions;
 *
 * or:
 *
 *     partition_count: workload_size / target_parallelism;
 *
 * are semantic program expressions.
 *
 * They are NOT grammar-level limits.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Partitioning preserves the distinction between:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *
 * These concepts must not be silently conflated.
 *
 * For example:
 *
 *     requirement: partition_count >= required_partitions;
 *
 * is different from:
 *
 *     constraint: partition_count <= permitted_partitions;
 *
 * and different from:
 *
 *     preference: partition_count = preferred_partitions;
 *
 * and different from:
 *
 *     hint: partition_count = estimated_partitions;
 *
 * Semantic analysis determines the exact meaning and validity of each
 * property.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed partitioning declaration framing;
 *     - partitioning declaration names;
 *     - partitioning bodies;
 *     - partitioning property structure;
 *     - partitioning values;
 *     - partitioning expressions;
 *     - partitioning lists;
 *     - partitioning objects;
 *     - partitioning operations;
 *     - partitioning relationships;
 *     - partitioning source-level intent;
 *     - partitioning extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - resource expressions;
 *     - resource requirements;
 *     - resource constraints;
 *     - resource preferences;
 *     - resource hints;
 *     - generic placement;
 *     - replication;
 *     - consistency;
 *     - communication;
 *     - messages;
 *     - topology;
 *     - scheduling;
 *     - routing;
 *     - storage implementation;
 *     - hardware;
 *     - quantum semantics;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime behavior.
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * The canonical expression system remains owned by:
 *
 *     grammar/expressions/
 *
 * Resource expressions are composed through:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * Names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * or its canonical Names grammar.
 *
 * Therefore this file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     arithmetic expressions
 *     logical expressions
 *     comparison expressions
 *     literals
 *     operators
 *     punctuation tokens.
 *
 * ============================================================================
 * RELATIONSHIP TO PLACEMENT
 * ============================================================================
 *
 * Partitioning and placement are distinct.
 *
 * Partitioning answers:
 *
 *     "How is a logical workload/data domain divided?"
 *
 * Placement answers:
 *
 *     "Where may the resulting logical entities be realized?"
 *
 * Therefore this file may REFER TO placement intent but must not redefine the
 * placement grammar.
 *
 * Canonical placement syntax remains owned by:
 *
 *     grammar/resources/placement.g4
 *
 * Distributed placement framing remains owned by:
 *
 *     grammar/distributed/placement.g4
 *
 * ============================================================================
 * RELATIONSHIP TO REPLICATION
 * ============================================================================
 *
 * Partitioning and replication are distinct.
 *
 * Partitioning divides a logical domain.
 *
 * Replication determines whether and how copies of logical state or execution
 * entities exist.
 *
 * This grammar may represent references or properties concerning replication,
 * but it does not define replication algorithms or replication syntax.
 *
 * The canonical replication component remains:
 *
 *     grammar/distributed/replication.g4
 *
 * ============================================================================
 * RELATIONSHIP TO CONSISTENCY
 * ============================================================================
 *
 * Partitioning and consistency are distinct.
 *
 * A partition may participate in a consistency policy, but partitioning does
 * not define consistency semantics.
 *
 * Canonical consistency syntax remains:
 *
 *     grammar/distributed/consistency.g4
 *
 * ============================================================================
 * RELATIONSHIP TO NETWORKING
 * ============================================================================
 *
 * Partitioning may affect communication and routing requirements, but it does
 * not define network protocols or transport mechanisms.
 *
 * The grammar therefore permits symbolic networking/resource properties while
 * leaving transport selection downstream.
 *
 * It does not hard-code:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *
 * as mandatory partitioning implementations.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed quantum programs may use partitioning to describe logical
 * decomposition of:
 *
 *     - classical control data;
 *     - workloads;
 *     - logical state domains;
 *     - distributed execution tasks;
 *     - measurement/control metadata;
 *     - hybrid computation.
 *
 * Partitioning MUST NOT be interpreted by this grammar as permission to
 * arbitrarily clone quantum state.
 *
 * This file does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     pulse semantics
 *     calibration
 *     QEC
 *     ZQN
 *     quantum::ir
 *
 * Quantum semantics continue to cross the canonical:
 *
 *     quantum::ir
 *
 * boundary downstream.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Partitioning may describe logical decomposition of hardware/software
 * workloads, but does not define:
 *
 *     bus widths;
 *     register counts;
 *     memory-bank counts;
 *     FPGA fabric dimensions;
 *     ASIC topology;
 *     physical interconnects;
 *     clock domains.
 *
 * Hardware realization belongs to the hardware/HDL subsystems.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve a single structural representation of a
 * partitioning declaration.
 *
 * Conceptually:
 *
 *     DistributedPartitioning {
 *         name;
 *         properties;
 *         operations;
 *         source_span;
 *     }
 *
 * Property ordering and source spans MUST be preserved.
 *
 * Exact Rust AST type names belong to the frontend AST contract and are not
 * defined here.
 *
 * The grammar MUST NOT require a new AST node for every partitioning
 * algorithm.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for interpreting partition properties.
 *
 * It determines:
 *
 *     - whether a partitioning declaration is valid;
 *     - whether its target exists;
 *     - whether its key is type-valid;
 *     - whether its dimensions are meaningful;
 *     - whether its requirements can be satisfied;
 *     - whether constraints conflict;
 *     - whether preferences are merely advisory;
 *     - whether hints are applicable;
 *     - whether a strategy exists;
 *     - whether the requested partitioning is realizable.
 *
 * The parser does none of these things.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT create a partitioning IR.
 *
 * Parsed syntax lowers into the repository's canonical semantic model.
 *
 * Depending on the workload, semantic information may ultimately contribute
 * to:
 *
 *     classical representation;
 *     distributed execution representation;
 *     resource representation;
 *     storage representation;
 *     hardware representation;
 *     quantum::ir.
 *
 * Partitioning is therefore metadata/intent in the semantic pipeline, not a
 * competing IR authority.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser/frontend MUST preserve:
 *
 *     - declaration order;
 *     - property order;
 *     - operation order;
 *     - expression structure;
 *     - list ordering;
 *     - nested object structure;
 *     - qualified-name segment ordering;
 *     - source spans.
 *
 * The parser MUST NOT:
 *
 *     - evaluate expressions;
 *     - calculate partition counts;
 *     - select algorithms;
 *     - select nodes;
 *     - select devices;
 *     - perform placement;
 *     - perform replication;
 *     - perform routing;
 *     - perform scheduling.
 *
 * ============================================================================
 * DUPLICATE PROPERTY POLICY
 * ============================================================================
 *
 * Repeated properties are syntactically accepted.
 *
 * Example:
 *
 *     partition data {
 *         requirement: partition_count >= minimum;
 *         requirement: partition_count >= preferred_minimum;
 *     };
 *
 * The parser preserves both declarations.
 *
 * Semantic analysis determines whether they:
 *
 *     - compose;
 *     - conflict;
 *     - override;
 *     - are redundant;
 *     - are invalid.
 *
 * The parser must not silently discard information.
 *
 * ============================================================================
 * EMPTY BODY POLICY
 * ============================================================================
 *
 * An empty partitioning body is syntactically valid.
 *
 * Example:
 *
 *     partition data {};
 *
 * Semantic analysis determines whether an empty declaration is meaningful.
 *
 * This permits incremental construction, generated source, dialect extension,
 * and future semantic properties without requiring parser changes.
 *
 * ============================================================================
 * CONTEXTUAL KEYWORD POLICY
 * ============================================================================
 *
 * This grammar intentionally does NOT require a new global lexer token named:
 *
 *     PARTITION
 *     PARTITIONING
 *     SHARD
 *     RANGE
 *     HASH
 *     REBALANCE
 *
 * The introductory declaration word is parsed through the canonical
 * identifier system and is classified contextually by semantic analysis.
 *
 * This avoids lexical-keyword explosion and permits future partitioning
 * vocabulary without changing the lexer merely because a new algorithm or
 * partitioning concept is introduced.
 *
 * The semantic validator MUST reject a declaration whose contextual marker is
 * not an accepted partitioning marker.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no Rust code;
 *     - no I/O;
 *     - no randomness;
 *     - no hardware queries;
 *     - no environment queries;
 *     - no runtime callbacks.
 *
 * Parsing therefore depends only on the supplied token stream and grammar.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded Rust.
 *
 * Generated frontend integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust Edition 2021
 *
 * Generated Zamani code must remain safe Rust.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is intended to be imported by:
 *
 *     grammar/distributed/distributed.g4
 *
 * The aggregate distributed grammar should import:
 *
 *     DistributedPartitioning
 *
 * and route its partitioning branch to:
 *
 *     distributedPartitioningDeclaration
 *
 * The aggregate grammar MUST NOT redefine that rule.
 *
 * ============================================================================
 */

parser grammar DistributedPartitioning;

options {
    tokenVocab = ZamaniLexer;
}

import Names, ResourceExpressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable parser-composition entry point.
 *
 * Intended source shape:
 *
 *     partition data {
 *         key: user_id;
 *         strategy: partition::hash;
 *     };
 *
 * or:
 *
 *     partitioning workload {
 *         target: distributed::task;
 *         partition_count: desired_partitions;
 *     };
 *
 * The first identifier is contextually classified as:
 *
 *     partition
 *
 * or:
 *
 *     partitioning
 *
 * by semantic analysis.
 *
 * This avoids introducing a mandatory global PARTITION lexer token.
 */
distributedPartitioningDeclaration
    : partitioningMarker
      identifier
      distributedPartitioningBody?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. CONTEXTUAL PARTITIONING MARKER
 * ============================================================================
 *
 * The marker is deliberately an identifier.
 *
 * Semantic analysis MUST validate that its textual value is an accepted
 * partitioning contextual marker.
 *
 * The parser does not embed the lexical spelling of future partitioning
 * concepts.
 */
partitioningMarker
    : identifier
    ;


/*
 * ============================================================================
 * 3. PARTITIONING BODY
 * ============================================================================
 *
 * The body contains an arbitrary number of partitioning members.
 */
distributedPartitioningBody
    : LBRACE
      distributedPartitioningMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. PARTITIONING MEMBER
 * ============================================================================
 *
 * A member is either:
 *
 *     - a property;
 *     - an operation;
 *     - a relation.
 *
 * The grammar intentionally uses generic structures rather than creating
 * hundreds of algorithm-specific rules.
 */
distributedPartitioningMember
    : distributedPartitioningProperty
    | distributedPartitioningOperation
    | distributedPartitioningRelation
    ;


/*
 * ============================================================================
 * 5. GENERIC PROPERTY
 * ============================================================================
 *
 * Canonical forms:
 *
 *     key: value;
 *
 *     strategy: partition::hash;
 *
 *     partition_count: desired_count;
 *
 *     requirement: partition_count >= required_count;
 *
 *     constraint: locality_constraint;
 *
 *     preference: preferred_partitioning;
 *
 *     hint: estimated_partition_count;
 *
 * Property names are semantic identifiers.
 *
 * The grammar does not create a closed list of partitioning attributes.
 */
distributedPartitioningProperty
    : qualifiedName
      distributedPartitioningPropertySeparator
      distributedPartitioningValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. PROPERTY SEPARATOR
 * ============================================================================
 *
 * Both colon and assignment are accepted.
 *
 * This permits compatibility with existing repository styles:
 *
 *     key: value;
 *
 * and:
 *
 *     key = value;
 *
 * Semantic/style validation may establish which form is preferred for a
 * particular language version or dialect.
 */
distributedPartitioningPropertySeparator
    : COLON
    | ASSIGN
    ;


/*
 * ============================================================================
 * 7. PROPERTY VALUE
 * ============================================================================
 *
 * Values use canonical resource expressions or recursively structured
 * partitioning objects/lists.
 *
 * This means partitioning values may be:
 *
 *     scalar expressions;
 *     symbolic quantities;
 *     capability expressions;
 *     comparisons;
 *     names;
 *     calls;
 *     lists;
 *     nested objects.
 *
 * No partition-specific expression language is created.
 */
distributedPartitioningValue
    : resourceExpression
    | distributedPartitioningObject
    | distributedPartitioningList
    ;


/*
 * ============================================================================
 * 8. NESTED OBJECT
 * ============================================================================
 *
 * Nested objects permit structured partitioning intent without requiring a
 * new grammar rule for every future property.
 *
 * Example:
 *
 *     boundaries: {
 *         lower: lower_bound;
 *         upper: upper_bound;
 *     };
 */
distributedPartitioningObject
    : LBRACE
      distributedPartitioningProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * 9. LIST
 * ============================================================================
 *
 * Arbitrary-length lists are supported.
 *
 * There is no finite number of partition keys, ranges, dimensions, regions,
 * selectors, or other semantic values encoded here.
 */
distributedPartitioningList
    : LBRACKET
      distributedPartitioningListItem*
      RBRACKET
    ;


/*
 * ============================================================================
 * 10. LIST ITEM
 * ============================================================================
 */
distributedPartitioningListItem
    : distributedPartitioningValue
    ;


/*
 * ============================================================================
 * 11. OPTIONAL LIST
 * ============================================================================
 */
distributedPartitioningOptionalList
    : distributedPartitioningList?
    ;


/*
 * ============================================================================
 * 12. GENERIC PARTITIONING OPERATION
 * ============================================================================
 *
 * Canonical open-world operation form:
 *
 *     split(target, boundary);
 *
 *     merge(left, right);
 *
 *     rebalance(target, policy);
 *
 *     migrate(source, destination);
 *
 *     repartition(target, strategy);
 *
 * The operation name is semantic data.
 *
 * No fixed list of operations is encoded.
 *
 * This permits future operations without parser redesign.
 */
distributedPartitioningOperation
    : qualifiedName
      LPAREN
      resourceExpressionList?
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. PARTITIONING RELATION
 * ============================================================================
 *
 * A relation associates a partitioning declaration with another semantic
 * entity.
 *
 * Examples:
 *
 *     applies_to: dataset;
 *
 *     derives_from: source_partitioning;
 *
 *     composed_with: other_partitioning;
 *
 *     depends_on: upstream_partitioning;
 *
 *     references: distributed::workload;
 *
 * The relation name is intentionally open-world.
 */
distributedPartitioningRelation
    : distributedPartitioningRelationName
      distributedPartitioningPropertySeparator
      distributedPartitioningReferenceValue
      SEMICOLON?
    ;


distributedPartitioningRelationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 14. REFERENCE VALUE
 * ============================================================================
 *
 * References are represented using canonical expressions.
 *
 * This avoids creating a second name/reference grammar.
 */
distributedPartitioningReferenceValue
    : resourceExpression
    | distributedPartitioningObject
    | distributedPartitioningList
    ;


/*
 * ============================================================================
 * 15. PARTITIONING SPECIFICATION
 * ============================================================================
 *
 * Reusable payload entry point for another distributed grammar that already
 * owns the outer declaration framing.
 *
 * Example composition:
 *
 *     someDistributedConstruct {
 *         <partitioning specification>
 *     }
 */
distributedPartitioningSpecification
    : distributedPartitioningBody
    ;


/*
 * ============================================================================
 * 16. PARTITIONING PROPERTY LIST
 * ============================================================================
 *
 * Explicit reusable property-list boundary.
 *
 * It does not introduce a second property model.
 */
distributedPartitioningPropertyList
    : distributedPartitioningProperty*
    ;


/*
 * ============================================================================
 * 17. PARTITIONING EXPRESSION
 * ============================================================================
 *
 * Semantic wrapper around the canonical resource expression.
 *
 * No new expression grammar is created.
 */
distributedPartitioningExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 18. PARTITIONING QUANTITY
 * ============================================================================
 *
 * A partition quantity is any expression whose semantic type is suitable for
 * a partition quantity.
 *
 * Examples:
 *
 *     partition_count
 *
 *     workload_size / parallelism
 *
 *     required_partitions
 *
 *     available_partitions
 *
 * The grammar does not evaluate or validate the quantity.
 */
distributedPartitioningQuantity
    : resourceExpression
    ;


/*
 * ============================================================================
 * 19. PARTITIONING PREDICATE
 * ============================================================================
 *
 * Semantic wrapper for conditions such as:
 *
 *     partition_count >= required_partitions
 *
 *     workload_size > threshold
 *
 *     locality == preferred_locality
 *
 * Type checking belongs downstream.
 */
distributedPartitioningPredicate
    : resourceExpression
    ;


/*
 * ============================================================================
 * 20. PARTITIONING KEY
 * ============================================================================
 *
 * A key is represented as an expression rather than as a closed key grammar.
 *
 * This supports:
 *
 *     user_id
 *
 *     record.customer.id
 *
 *     hash_key
 *
 *     composite_key
 *
 *     derive_key(value)
 *
 *     namespace::key
 *
 * without imposing a fixed key model.
 */
distributedPartitioningKey
    : resourceExpression
    ;


/*
 * ============================================================================
 * 21. PARTITIONING BOUNDARY
 * ============================================================================
 *
 * A boundary is an expression.
 *
 * It may therefore be:
 *
 *     a literal;
 *     a symbolic value;
 *     a computed value;
 *     a collection;
 *     a domain-specific value;
 *     a future semantic object.
 *
 * The parser does not determine whether the boundary is valid.
 */
distributedPartitioningBoundary
    : resourceExpression
    ;


/*
 * ============================================================================
 * 22. PARTITIONING DIMENSION
 * ============================================================================
 *
 * Partition dimensions are semantic expressions.
 *
 * There is no maximum dimension count.
 */
distributedPartitioningDimension
    : resourceExpression
    ;


/*
 * ============================================================================
 * 23. PARTITIONING TARGET
 * ============================================================================
 *
 * The target is an abstract semantic subject.
 *
 * It may identify:
 *
 *     a dataset;
 *     a collection;
 *     a stream;
 *     a task;
 *     a service;
 *     a workload;
 *     a logical resource;
 *     a distributed computation;
 *     a future domain object.
 *
 * It does not identify a physical machine.
 */
distributedPartitioningTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * 24. PARTITIONING STRATEGY
 * ============================================================================
 *
 * Strategies are semantic names.
 *
 * Examples:
 *
 *     partition::hash
 *     partition::range
 *     partition::list
 *     partition::custom
 *     vendor::partitioning::strategy
 *
 * No strategy is hard-coded here.
 */
distributedPartitioningStrategy
    : qualifiedName
    ;


/*
 * ============================================================================
 * 25. PARTITIONING POLICY
 * ============================================================================
 *
 * Policies are semantic names.
 */
distributedPartitioningPolicy
    : qualifiedName
    ;


/*
 * ============================================================================
 * 26. PARTITIONING REQUIREMENT
 * ============================================================================
 *
 * This rule is a semantic wrapper only.
 *
 * The actual meaning of the expression is validated by resource and semantic
 * analysis.
 */
distributedPartitioningRequirement
    : resourceExpression
    ;


/*
 * ============================================================================
 * 27. PARTITIONING CONSTRAINT
 * ============================================================================
 */
distributedPartitioningConstraint
    : resourceExpression
    ;


/*
 * ============================================================================
 * 28. PARTITIONING PREFERENCE
 * ============================================================================
 */
distributedPartitioningPreference
    : resourceExpression
    ;


/*
 * ============================================================================
 * 29. PARTITIONING HINT
 * ============================================================================
 */
distributedPartitioningHint
    : resourceExpression
    ;


/*
 * ============================================================================
 * 30. PARTITIONING REFERENCE
 * ============================================================================
 *
 * A partitioning reference is a canonical qualified name.
 */
distributedPartitioningReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 31. OPTIONAL PARTITIONING REFERENCE
 * ============================================================================
 */
distributedPartitioningOptionalReference
    : distributedPartitioningReference?
    ;


/*
 * ============================================================================
 * 32. PARTITIONING NAME
 * ============================================================================
 *
 * Canonical name ownership remains in Names.
 */
distributedPartitioningName
    : identifier
    ;


/*
 * ============================================================================
 * 33. QUALIFIED PARTITIONING NAME
 * ============================================================================
 */
distributedPartitioningQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 34. PARTITIONING ARGUMENT LIST
 * ============================================================================
 *
 * Reuses canonical resource-expression syntax.
 */
distributedPartitioningArgumentList
    : resourceExpressionList
    ;


/*
 * ============================================================================
 * 35. OPTIONAL PARTITIONING ARGUMENT LIST
 * ============================================================================
 */
distributedPartitioningOptionalArgumentList
    : resourceExpressionList?
    ;


/*
 * ============================================================================
 * 36. PARTITIONING VALUE LIST
 * ============================================================================
 *
 * Arbitrary cardinality.
 */
distributedPartitioningValueList
    : distributedPartitioningValue
      (COMMA distributedPartitioningValue)*
      COMMA?
    ;


/*
 * ============================================================================
 * 37. OPTIONAL VALUE LIST
 * ============================================================================
 */
distributedPartitioningOptionalValueList
    : distributedPartitioningValueList?
    ;


/*
 * ============================================================================
 * 38. PARTITIONING OBJECT MEMBER
 * ============================================================================
 *
 * Reusable nested-object member boundary.
 */
distributedPartitioningObjectMember
    : distributedPartitioningProperty
    ;


/*
 * ============================================================================
 * 39. PARTITIONING METADATA
 * ============================================================================
 *
 * Metadata is deliberately represented as ordinary semantic properties.
 *
 * Examples:
 *
 *     metadata::owner: owner;
 *
 *     metadata::provenance: provenance;
 *
 *     metadata::version: version;
 *
 *     vendor::extension::property: value;
 *
 * No metadata vocabulary is hard-coded.
 */
distributedPartitioningMetadata
    : qualifiedName
      distributedPartitioningPropertySeparator
      distributedPartitioningValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 40. PARTITIONING EXTENSION
 * ============================================================================
 *
 * Future and dialect-specific partitioning constructs can be represented
 * without modifying the core grammar.
 *
 * Semantic validation determines whether the extension is:
 *
 *     standard;
 *     experimental;
 *     dialect-defined;
 *     vendor-defined;
 *     deprecated;
 *     unknown.
 */
distributedPartitioningExtension
    : qualifiedName
      distributedPartitioningPropertySeparator
      distributedPartitioningValue
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 41. PARTITIONING PATH
 * ============================================================================
 *
 * Canonical qualified-name path.
 */
distributedPartitioningPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 42. PARTITIONING CONDITION
 * ============================================================================
 */
distributedPartitioningCondition
    : resourceExpression
    ;


/*
 * ============================================================================
 * 43. PARTITIONING SELECTOR
 * ============================================================================
 *
 * A selector is an expression.
 *
 * This can represent:
 *
 *     key selection;
 *     partition selection;
 *     workload selection;
 *     logical-domain selection.
 *
 * Actual semantics belong downstream.
 */
distributedPartitioningSelector
    : resourceExpression
    ;


/*
 * ============================================================================
 * 44. PARTITIONING FILTER
 * ============================================================================
 */
distributedPartitioningFilter
    : resourceExpression
    ;


/*
 * ============================================================================
 * 45. PARTITIONING TRANSFORMATION
 * ============================================================================
 */
distributedPartitioningTransformation
    : resourceExpression
    ;


/*
 * ============================================================================
 * 46. PARTITIONING COMPOSITION
 * ============================================================================
 *
 * Partitioning specifications may be nested or composed.
 */
distributedPartitioningComposition
    : distributedPartitioningValue
    ;


/*
 * ============================================================================
 * 47. PARTITIONING RANGE
 * ============================================================================
 *
 * Range semantics remain open-world.
 *
 * This rule intentionally accepts an expression rather than imposing a
 * particular range representation.
 */
distributedPartitioningRange
    : resourceExpression
    ;


/*
 * ============================================================================
 * 48. PARTITIONING GROUP
 * ============================================================================
 *
 * A group is a semantic collection of partitioning values.
 */
distributedPartitioningGroup
    : distributedPartitioningList
    ;


/*
 * ============================================================================
 * 49. PARTITIONING SET
 * ============================================================================
 *
 * A set-like semantic value remains represented structurally as a list.
 *
 * Whether ordering matters is a semantic question.
 */
distributedPartitioningSet
    : distributedPartitioningList
    ;


/*
 * ============================================================================
 * 50. PARTITIONING MAP
 * ============================================================================
 *
 * A map-like semantic value is represented as an object.
 *
 * Semantic analysis determines key/value requirements.
 */
distributedPartitioningMap
    : distributedPartitioningObject
    ;


/*
 * ============================================================================
 * 51. PARTITIONING CARDINALITY
 * ============================================================================
 *
 * Cardinality is always an expression.
 *
 * There is deliberately no:
 *
 *     INT_MAX_PARTITIONS
 *     MAX_SHARDS
 *     MAX_PARTITION_COUNT
 *
 * encoded in the grammar.
 */
distributedPartitioningCardinality
    : resourceExpression
    ;


/*
 * ============================================================================
 * 52. PARTITIONING SCALE
 * ============================================================================
 *
 * Scaling is an expression/policy, not a fixed machine dimension.
 */
distributedPartitioningScale
    : resourceExpression
    ;


/*
 * ============================================================================
 * 53. PARTITIONING ELASTICITY
 * ============================================================================
 */
distributedPartitioningElasticity
    : resourceExpression
    ;


/*
 * ============================================================================
 * 54. PARTITIONING MIGRATION INTENT
 * ============================================================================
 *
 * Migration intent is represented as an expression.
 *
 * Actual migration is owned downstream.
 */
distributedPartitioningMigration
    : resourceExpression
    ;


/*
 * ============================================================================
 * 55. PARTITIONING REBALANCING INTENT
 * ============================================================================
 *
 * Actual rebalancing is downstream.
 */
distributedPartitioningRebalancing
    : resourceExpression
    ;


/*
 * ============================================================================
 * 56. PARTITIONING FAILURE INTENT
 * ============================================================================
 *
 * Failure behavior is semantic policy.
 *
 * It does not implement fault tolerance.
 */
distributedPartitioningFailurePolicy
    : resourceExpression
    ;


/*
 * ============================================================================
 * 57. PARTITIONING AVAILABILITY
 * ============================================================================
 */
distributedPartitioningAvailability
    : resourceExpression
    ;


/*
 * ============================================================================
 * 58. PARTITIONING DURABILITY
 * ============================================================================
 */
distributedPartitioningDurability
    : resourceExpression
    ;


/*
 * ============================================================================
 * 59. PARTITIONING LOCALITY
 * ============================================================================
 *
 * Locality remains an abstract semantic property.
 *
 * It does not select a physical node, NUMA region, memory bank, QPU, or
 * device.
 */
distributedPartitioningLocality
    : resourceExpression
    ;


/*
 * ============================================================================
 * 60. PARTITIONING AFFINITY
 * ============================================================================
 */
distributedPartitioningAffinity
    : resourceExpression
    ;


/*
 * ============================================================================
 * 61. PARTITIONING ANTI-AFFINITY
 * ============================================================================
 */
distributedPartitioningAntiAffinity
    : resourceExpression
    ;


/*
 * ============================================================================
 * 62. PARTITIONING COMMUNICATION PROPERTY
 * ============================================================================
 *
 * Communication characteristics may be described symbolically.
 *
 * Transport selection remains owned by networking.
 */
distributedPartitioningCommunication
    : resourceExpression
    ;


/*
 * ============================================================================
 * 63. PARTITIONING COST
 * ============================================================================
 */
distributedPartitioningCost
    : resourceExpression
    ;


/*
 * ============================================================================
 * 64. PARTITIONING PERFORMANCE
 * ============================================================================
 */
distributedPartitioningPerformance
    : resourceExpression
    ;


/*
 * ============================================================================
 * 65. PARTITIONING LATENCY
 * ============================================================================
 */
distributedPartitioningLatency
    : resourceExpression
    ;


/*
 * ============================================================================
 * 66. PARTITIONING THROUGHPUT
 * ============================================================================
 */
distributedPartitioningThroughput
    : resourceExpression
    ;


/*
 * ============================================================================
 * 67. PARTITIONING CAPABILITY
 * ============================================================================
 *
 * Capability is a semantic expression.
 *
 * Hardware/resource discovery remains downstream.
 */
distributedPartitioningCapability
    : resourceExpression
    ;


/*
 * ============================================================================
 * 68. PARTITIONING RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Resource requirements are expressed through the canonical resource
 * expression architecture.
 */
distributedPartitioningResourceRequirement
    : resourceExpression
    ;


/*
 * ============================================================================
 * 69. PARTITIONING RESOURCE CONSTRAINT
 * ============================================================================
 */
distributedPartitioningResourceConstraint
    : resourceExpression
    ;


/*
 * ============================================================================
 * 70. PARTITIONING RESOURCE PREFERENCE
 * ============================================================================
 */
distributedPartitioningResourcePreference
    : resourceExpression
    ;


/*
 * ============================================================================
 * 71. PARTITIONING RESOURCE HINT
 * ============================================================================
 */
distributedPartitioningResourceHint
    : resourceExpression
    ;


/*
 * ============================================================================
 * 72. PARTITIONING TARGET REFERENCE
 * ============================================================================
 *
 * A target reference remains symbolic.
 */
distributedPartitioningTargetReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 73. PARTITIONING SOURCE REFERENCE
 * ============================================================================
 */
distributedPartitioningSourceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 74. PARTITIONING DESTINATION REFERENCE
 * ============================================================================
 */
distributedPartitioningDestinationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 75. PARTITIONING VERSION
 * ============================================================================
 *
 * Version is semantic data, not a parser implementation version.
 */
distributedPartitioningVersion
    : resourceExpression
    ;


/*
 * ============================================================================
 * 76. PARTITIONING POLICY REFERENCE
 * ============================================================================
 */
distributedPartitioningPolicyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 77. PARTITIONING STRATEGY REFERENCE
 * ============================================================================
 */
distributedPartitioningStrategyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 78. PARTITIONING PLACEMENT REFERENCE
 * ============================================================================
 *
 * This is only a symbolic reference.
 *
 * The actual placement grammar remains owned by the resource/distributed
 * placement components.
 */
distributedPartitioningPlacementReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 79. PARTITIONING REPLICATION REFERENCE
 * ============================================================================
 *
 * Replication remains a separate grammar/semantic subsystem.
 */
distributedPartitioningReplicationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 80. PARTITIONING CONSISTENCY REFERENCE
 * ============================================================================
 *
 * Consistency remains a separate grammar/semantic subsystem.
 */
distributedPartitioningConsistencyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 81. PARTITIONING EXTENSION VALUE
 * ============================================================================
 */
distributedPartitioningExtensionValue
    : distributedPartitioningValue
    ;


/*
 * ============================================================================
 * 82. PARTITIONING EXPRESSION LIST
 * ============================================================================
 */
distributedPartitioningExpressionList
    : resourceExpressionList
    ;


/*
 * ============================================================================
 * 83. OPTIONAL EXPRESSION LIST
 * ============================================================================
 */
distributedPartitioningOptionalExpressionList
    : resourceExpressionList?
    ;


/*
 * ============================================================================
 * 84. PARTITIONING DECLARATION ALIAS
 * ============================================================================
 *
 * This alias exists only for parser composition compatibility.
 *
 * It does NOT represent another semantic partitioning construct.
 */
partitioningDeclaration
    : distributedPartitioningDeclaration
    ;