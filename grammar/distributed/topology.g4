/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/distributed/topology.g4
 *
 * GRAMMAR
 * -------
 * DistributedTopology
 *
 * STATUS
 * ------
 * PRODUCTION-READY DISTRIBUTED TOPOLOGY INTENT GRAMMAR
 *
 * ANTLR
 * -----
 * ANTLR4 parser grammar
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97+ / Rust 2021
 *
 * SAFETY
 * ------
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no semantic predicates;
 *   - no actions;
 *   - no unsafe implementation;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware discovery;
 *   - no resource allocation;
 *   - no runtime callbacks;
 *   - no randomness;
 *   - no environment inspection.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL LOGICAL DISTRIBUTED TOPOLOGY INTENT.
 *
 * A distributed topology describes relationships between logical computational
 * participants.
 *
 * It may describe:
 *
 *   - logical nodes;
 *   - logical groups;
 *   - logical relationships;
 *   - logical links;
 *   - endpoint relationships;
 *   - topology properties;
 *   - topology requirements;
 *   - topology constraints;
 *   - topology preferences;
 *   - topology hints;
 *   - topology predicates;
 *   - topology metadata.
 *
 * It does NOT describe a physical topology.
 *
 * ============================================================================
 * 2. TOPOLOGY MEANING
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What logical relationship does the program require or prefer?"
 *
 * It does NOT answer:
 *
 *     "Which physical machine, device, network, accelerator, QPU, cable,
 *      transport or route realizes that relationship?"
 *
 * The realization pipeline remains:
 *
 *     logical topology
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     resource/capability negotiation
 *          |
 *          v
 *     placement
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * 3. POCO-REAF
 * ============================================================================
 *
 * A topology declaration is portable source intent.
 *
 * The same source may be realized on:
 *
 *   - a tiny system;
 *   - one process;
 *   - multiple processes;
 *   - multicore systems;
 *   - clusters;
 *   - HPC systems;
 *   - cloud systems;
 *   - edge systems;
 *   - heterogeneous CPU/GPU/FPGA/ASIC systems;
 *   - accelerator systems;
 *   - quantum-classical systems;
 *   - distributed quantum systems;
 *   - future computational substrates.
 *
 * Physical realization is therefore never encoded as a universal grammar
 * constant.
 *
 * ============================================================================
 * 4. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * -------------
 *
 *   - distributed topology declaration syntax;
 *   - logical topology participants;
 *   - logical topology groups;
 *   - logical topology relationships;
 *   - logical topology links;
 *   - topology-local metadata;
 *   - topology-local requirements;
 *   - topology-local constraints;
 *   - topology-local preferences;
 *   - topology-local hints;
 *   - topology-local predicates.
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *   - identifiers;
 *   - qualified names;
 *   - general expressions;
 *   - type expressions;
 *   - resource allocation;
 *   - capability discovery;
 *   - placement;
 *   - routing;
 *   - scheduling;
 *   - networking transport;
 *   - network addresses;
 *   - sockets;
 *   - packets;
 *   - replication;
 *   - consistency;
 *   - consensus;
 *   - fault tolerance;
 *   - deployment;
 *   - hardware topology;
 *   - quantum physical topology;
 *   - quantum routing;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * 5. OPEN-WORLD REQUIREMENT
 * ============================================================================
 *
 * The grammar MUST NOT enumerate topology families such as:
 *
 *   ring
 *   mesh
 *   torus
 *   tree
 *   star
 *   grid
 *   hypercube
 *   dragonfly
 *   fat_tree
 *   heavy_hex
 *   butterfly
 *   fully_connected
 *   sparse
 *
 * Such names remain semantic data.
 *
 * A future topology family must not require modification of this grammar.
 *
 * ============================================================================
 * 6. SCALABILITY REQUIREMENT
 * ============================================================================
 *
 * This grammar imposes no language-level upper bound on:
 *
 *   - topology declarations;
 *   - topology members;
 *   - logical nodes;
 *   - logical groups;
 *   - logical relationships;
 *   - endpoints;
 *   - topology dimensions;
 *   - properties;
 *   - requirements;
 *   - constraints;
 *   - preferences;
 *   - hints;
 *   - metadata.
 *
 * There is deliberately no:
 *
 *   MAX_NODES
 *   MAX_EDGES
 *   MAX_LINKS
 *   MAX_GROUPS
 *   MAX_TOPOLOGIES
 *   MAX_NETWORK_SIZE
 *   MAX_DEVICES
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_QPUS
 *   MAX_MEMORY
 *
 * Practical parser/compiler/runtime limits are implementation or deployment
 * concerns and MUST NOT become language semantics.
 *
 * ============================================================================
 * 7. LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar defines NO lexer rules.
 *
 * All tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the canonical lexical composition.
 *
 * The topology grammar MUST NOT define private lexer rules.
 *
 * ============================================================================
 * 8. NAME AUTHORITY
 * ============================================================================
 *
 * Names are owned by:
 *
 *     grammar/core/names.g4
 *
 * This grammar consumes:
 *
 *     identifier
 *     qualifiedName
 *
 * It does not redefine them.
 *
 * ============================================================================
 * 9. EXPRESSION AUTHORITY
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/
 *
 * This grammar consumes:
 *
 *     expression
 *     optionalExpressionList
 *
 * It does not define another expression hierarchy.
 *
 * This is important because topology properties, requirements and predicates
 * may contain arbitrary future expressions without requiring topology grammar
 * changes.
 *
 * ============================================================================
 * 10. RESOURCE / CAPABILITY AUTHORITY
 * ============================================================================
 *
 * Resource and capability semantics remain downstream.
 *
 * This grammar only preserves their source-level expressions.
 *
 * Examples:
 *
 *     requires capability("distributed.communication");
 *
 *     requires topology(required_topology);
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     constraint latency <= latency_budget;
 *
 *     prefer locality;
 *
 *     hint hierarchical;
 *
 * The grammar does not evaluate these expressions.
 *
 * ============================================================================
 * 11. PUBLIC API
 * ============================================================================
 *
 * Exactly one public entry point is exported:
 *
 *     distributedTopologyConstruct
 *
 * The distributed composition root consumes that rule.
 *
 * ============================================================================
 */

parser grammar DistributedTopology;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC COMPOSITION BOUNDARY
 * ============================================================================
 *
 * The higher-level distributed grammar consumes only this rule.
 */
distributedTopologyConstruct
    : distributedTopologyDeclaration
    ;


/* ============================================================================
 * TOPOLOGY DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     topology cluster {
 *         node worker;
 *     }
 *
 *     topology distributed_execution {
 *         node producer;
 *         node consumer;
 *
 *         edge(producer, consumer);
 *     }
 *
 * `topology` is intentionally contextual here rather than introducing another
 * mandatory lexical token solely for this grammar.
 *
 * The semantic layer validates the declaration marker.
 *
 * This avoids making topology syntax depend on a private lexer token while
 * preserving the open-world lexical architecture.
 */
distributedTopologyDeclaration
    : distributedTopologyKeyword
      qualifiedName
      distributedTopologyTypeClause?
      distributedTopologyGenericParameters?
      distributedTopologyExtendsClause?
      distributedTopologyBody
    ;


/* ============================================================================
 * CONTEXTUAL TOPOLOGY KEYWORD
 * ============================================================================
 *
 * The canonical lexer currently keeps open-world names available as
 * identifiers. The semantic layer validates that this identifier has the
 * canonical spelling:
 *
 *     topology
 *
 * This rule therefore does not introduce a second lexical authority.
 *
 * If the canonical lexer later promotes `topology` to a reserved keyword,
 * this single rule is the integration point that must be switched to the
 * canonical TOPOLOGY token. No other topology rule should change.
 */
distributedTopologyKeyword
    : identifier
    ;


/* ============================================================================
 * OPTIONAL TOPOLOGY TYPE
 * ============================================================================
 *
 * Examples:
 *
 *     topology cluster { ... }
 *     topology logical_network { ... }
 *     topology distributed_execution { ... }
 *
 * `cluster`, `logical_network`, etc. are semantic names.
 *
 * They are NOT a closed enumeration.
 */
distributedTopologyTypeClause
    : COLON
      qualifiedName
    ;


/* ============================================================================
 * GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters describe symbolic topology parameters.
 *
 * They are not physical resource limits.
 *
 * Examples:
 *
 *     topology network<N> { ... }
 *
 *     topology fabric<NodeKind, Policy> { ... }
 *
 * Any interpretation belongs downstream.
 */
distributedTopologyGenericParameters
    : LESS_THAN
      distributedTopologyGenericParameter
      (
          COMMA
          distributedTopologyGenericParameter
      )*
      COMMA?
      GREATER_THAN
    ;


distributedTopologyGenericParameter
    : identifier
      distributedTopologyGenericBound?
      (
          ASSIGN
          expression
      )?
    ;


distributedTopologyGenericBound
    : COLON
      qualifiedName
    ;


/* ============================================================================
 * EXTENSION
 * ============================================================================
 *
 * Extension names are semantic names.
 *
 * This grammar does not define inheritance semantics.
 */
distributedTopologyExtendsClause
    : EXTENDS
      qualifiedName
      (
          COMMA
          qualifiedName
      )*
    ;


/* ============================================================================
 * TOPOLOGY BODY
 * ============================================================================
 *
 * Repetition is intentionally unbounded at the language level.
 */
distributedTopologyBody
    : LBRACE
      distributedTopologyMember*
      RBRACE
    ;


/* ============================================================================
 * TOPOLOGY MEMBER
 * ============================================================================
 *
 * A topology body contains only topology-owned constructs.
 *
 * General executable statements do not belong here.
 */
distributedTopologyMember
    : distributedTopologyNode
    | distributedTopologyGroup
    | distributedTopologyEdge
    | distributedTopologyLink
    | distributedTopologyRequirement
    | distributedTopologyConstraint
    | distributedTopologyPreference
    | distributedTopologyHint
    | distributedTopologyPredicate
    | distributedTopologyProperty
    | distributedTopologySection
    ;


/* ============================================================================
 * LOGICAL NODE
 * ============================================================================
 *
 * A node is a logical participant.
 *
 * It is NOT:
 *
 *     - a physical CPU;
 *     - a physical machine;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a network interface;
 *     - a cloud instance.
 *
 * Example:
 *
 *     node worker;
 *
 *     node worker {
 *         role: compute;
 *     }
 */
distributedTopologyNode
    : NODE
      identifier
      distributedTopologyMemberBody?
      SEMICOLON?
    ;


/* ============================================================================
 * LOGICAL GROUP
 * ============================================================================
 *
 * A group is a logical topology participant collection.
 *
 * The expression remains responsible for describing the group's membership or
 * semantic definition.
 *
 * Examples:
 *
 *     group workers;
 *
 *     group workers = worker_set;
 *
 *     group compute_workers {
 *         role: compute;
 *     }
 */
distributedTopologyGroup
    : GROUP
      identifier
      (
          ASSIGN
          expression
      )?
      distributedTopologyMemberBody?
      SEMICOLON?
    ;


/* ============================================================================
 * LOGICAL EDGE
 * ============================================================================
 *
 * The word `edge` is intentionally contextual.
 *
 * The semantic layer validates the canonical spelling.
 *
 * An edge expresses a logical relationship between endpoints.
 *
 * It does not select a route or transport.
 *
 * Examples:
 *
 *     edge(worker_a, worker_b);
 *
 *     edge(producer, consumer) {
 *         latency: latency_budget;
 *     }
 */
distributedTopologyEdge
    : distributedTopologyEdgeKeyword
      LPAREN
      distributedTopologyEndpointList
      RPAREN
      distributedTopologyMemberBody?
      SEMICOLON?
    ;


distributedTopologyEdgeKeyword
    : identifier
    ;


/* ============================================================================
 * LOGICAL LINK
 * ============================================================================
 *
 * A link is a logical relationship declaration.
 *
 * It does not imply TCP, UDP, QUIC, MPI, RDMA, Ethernet, optical transport,
 * wireless transport, quantum communication hardware, or any other concrete
 * implementation.
 */
distributedTopologyLink
    : distributedTopologyLinkKeyword
      LPAREN
      distributedTopologyEndpointList
      RPAREN
      distributedTopologyMemberBody?
      SEMICOLON?
    ;


distributedTopologyLinkKeyword
    : identifier
    ;


/* ============================================================================
 * ENDPOINT LIST
 * ============================================================================
 *
 * Endpoint cardinality is language-unbounded.
 *
 * Endpoint expressions are preserved for semantic resolution.
 *
 * The semantic layer determines:
 *
 *     - whether endpoints exist;
 *     - whether the relationship is valid;
 *     - whether endpoint kinds are compatible;
 *     - whether the relationship is directed;
 *     - whether the relationship is realizable.
 */
distributedTopologyEndpointList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * Properties are intentionally identifier-based.
 *
 * This prevents a new topology characteristic from requiring a new keyword.
 *
 * Examples:
 *
 *     locality: logical;
 *     direction: ordered;
 *     semantics: communication;
 *     metric: latency;
 */
distributedTopologyProperty
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * This rule does not evaluate them.
 */
distributedTopologyRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Constraints restrict valid realization.
 */
distributedTopologyConstraint
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences guide downstream realization and optimization.
 */
distributedTopologyPreference
    : PREFER
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * HINT
 * ============================================================================
 *
 * Hints are advisory.
 *
 * They MUST NOT be interpreted as mandatory physical realization decisions.
 */
distributedTopologyHint
    : HINT
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * PREDICATE
 * ============================================================================
 *
 * A predicate is an extensible semantic relation.
 *
 * Examples:
 *
 *     connected(a, b);
 *     locality(worker_a, worker_b);
 *     symmetric(link);
 *     directed(edge);
 *
 * Predicate meaning is semantic, not parser-owned.
 */
distributedTopologyPredicate
    : identifier
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * NESTED TOPOLOGY SECTION
 * ============================================================================
 *
 * Sections provide extensible topology-local metadata without allowing
 * arbitrary executable statements.
 *
 * Examples:
 *
 *     metadata {
 *         owner: application;
 *     }
 *
 *     properties {
 *         locality: logical;
 *     }
 *
 * Section names are semantic identifiers.
 */
distributedTopologySection
    : identifier
      distributedTopologyMemberBody
    ;


/* ============================================================================
 * MEMBER BODY
 * ============================================================================
 *
 * A member body contains metadata/specification members only.
 *
 * It deliberately does NOT contain:
 *
 *     topology declarations;
 *     physical placement;
 *     routing commands;
 *     scheduling commands;
 *     network transport commands;
 *     executable statements.
 */
distributedTopologyMemberBody
    : LBRACE
      distributedTopologyMetadataMember*
      RBRACE
    ;


/* ============================================================================
 * METADATA MEMBER
 * ============================================================================
 *
 * Metadata members are intentionally narrower than top-level topology
 * members.
 *
 * This prevents accidental recursive topology construction.
 */
distributedTopologyMetadataMember
    : distributedTopologyProperty
    | distributedTopologyRequirement
    | distributedTopologyConstraint
    | distributedTopologyPreference
    | distributedTopologyHint
    | distributedTopologyPredicate
    | distributedTopologySection
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/
 *
 * Specifically consumes:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     optionalExpressionList
 *
 *
 * EXPORTS
 * -------
 *
 *     distributedTopologyConstruct
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/distributed/distributed.g4
 *
 * The distributed composition root must consume:
 *
 *     distributedTopologyConstruct
 *
 * and must not recreate topology syntax.
 *
 *
 * AST_OWNER
 * ---------
 *
 *     domain-neutral frontend AST
 *
 * The AST must preserve:
 *
 *     topology declaration
 *     topology name
 *     topology type
 *     generic parameters
 *     extension names
 *     member ordering
 *     node declarations
 *     group declarations
 *     relationship ordering
 *     endpoint expressions
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     predicates
 *     metadata
 *     source spans
 *
 * This grammar does not define Rust AST structs.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     distributed semantic analysis
 *     name resolution
 *     type checking
 *     resource/capability analysis
 *     topology validation
 *     policy analysis
 *     provenance
 *
 * The parser does not:
 *
 *     - resolve nodes;
 *     - validate topology semantics;
 *     - query hardware;
 *     - inspect resources;
 *     - choose a target;
 *     - select a route;
 *     - select a transport.
 *
 *
 * IR_OWNER
 * --------
 *
 * This grammar creates no IR.
 *
 * Topology intent proceeds through the canonical semantic representation.
 *
 * No:
 *
 *     DistributedTopologyIR
 *
 * is introduced merely because the source syntax is located in this file.
 *
 * If topology information is required by quantum compilation, it reaches
 * the established semantic quantum boundary and subsequently `quantum::ir`
 * where appropriate.
 *
 *
 * RESOURCE BOUNDARY
 * -----------------
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * This grammar only preserves resource expressions such as:
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     requires capability("distributed.communication");
 *
 *
 * CAPABILITY BOUNDARY
 * -------------------
 *
 * Capability discovery and negotiation are downstream.
 *
 *
 * PLACEMENT BOUNDARY
 * ------------------
 *
 * Placement remains owned by:
 *
 *     grammar/distributed/placement.g4
 *     grammar/execution/placement.g4
 *     grammar/hardware/placement.g4
 *
 * depending on the semantic layer involved.
 *
 * This file does not place a logical participant onto a physical target.
 *
 *
 * ROUTING BOUNDARY
 * ----------------
 *
 * Routing is downstream.
 *
 * This file does not define:
 *
 *     route(...)
 *     path selection
 *     shortest path
 *     routing algorithm
 *     packet forwarding
 *     quantum SWAP insertion
 *
 *
 * SCHEDULING BOUNDARY
 * -------------------
 *
 * Scheduling is downstream.
 *
 * This file does not define:
 *
 *     schedule(...)
 *     execution slots
 *     timing allocation
 *     processor assignment
 *     communication reservation
 *
 *
 * NETWORKING BOUNDARY
 * -------------------
 *
 * Networking owns:
 *
 *     - endpoint realization;
 *     - addresses;
 *     - protocols;
 *     - transports;
 *     - sockets;
 *     - packets;
 *     - network security.
 *
 * A topology edge/link is therefore transport-neutral.
 *
 *
 * HARDWARE BOUNDARY
 * -----------------
 *
 * Hardware topology owns physical topology.
 *
 * This file must never encode:
 *
 *     physical device IDs
 *     physical CPU IDs
 *     physical GPU IDs
 *     FPGA coordinates
 *     ASIC fabric coordinates
 *     physical memory addresses
 *     rack coordinates
 *     cable identifiers
 *     vendor topology inventories
 *
 *
 * QUANTUM BOUNDARY
 * ----------------
 *
 * This grammar may describe logical relationships involving quantum execution
 * domains.
 *
 * It does not define:
 *
 *     physical qubits
 *     coupling maps
 *     native gate sets
 *     pulse routing
 *     calibration
 *     QEC
 *     ZQN
 *
 * Quantum computation continues through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 *
 * HDL BOUNDARY
 * ------------
 *
 * Hardware/HDL relationships remain logical at this layer.
 *
 * Physical synthesis and realization remain downstream.
 *
 *
 * EFFECT BOUNDARY
 * ---------------
 *
 * A topology declaration does not itself grant:
 *
 *     network access
 *     distributed execution
 *     native execution
 *     foreign execution
 *     hardware access
 *
 * Effects are determined by the semantic effect system.
 *
 *
 * POLICY BOUNDARY
 * ---------------
 *
 * Policies remain owned by:
 *
 *     grammar/policies/
 *     grammar/security/
 *     grammar/execution/
 *
 * A topology preference is not a security permission.
 *
 *
 * PROVENANCE BOUNDARY
 * -------------------
 *
 * The semantic model must preserve source provenance for:
 *
 *     topology declaration
 *     node declarations
 *     group declarations
 *     relationships
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     predicates
 *
 * This supports deterministic builds, diagnostics, auditing and explanation.
 *
 * ============================================================================
 * CONTEXTUAL KEYWORD CONTRACT
 * ============================================================================
 *
 * This file deliberately treats:
 *
 *     topology
 *     edge
 *     link
 *
 * as contextual identifiers rather than adding private lexer rules.
 *
 * The semantic layer MUST validate their canonical spellings.
 *
 * This gives the language an open lexical surface while still allowing the
 * topology grammar to remain stable.
 *
 * If these spellings are promoted to canonical reserved tokens in the future,
 * only these three contextual rules should change:
 *
 *     distributedTopologyKeyword
 *     distributedTopologyEdgeKeyword
 *     distributedTopologyLinkKeyword
 *
 * No topology semantic rule should require modification.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * PARSER ERRORS
 * -------------
 *
 *     malformed topology declaration
 *     missing topology name
 *     malformed generic parameter list
 *     malformed topology body
 *     malformed node declaration
 *     malformed group declaration
 *     malformed relationship
 *     malformed endpoint list
 *     malformed property
 *     malformed requirement
 *     malformed constraint
 *     malformed preference
 *     malformed hint
 *     malformed predicate
 *     malformed metadata section
 *
 * SEMANTIC ERRORS
 * ---------------
 *
 *     contextual keyword has invalid spelling
 *     duplicate topology declaration
 *     unknown node reference
 *     unknown group reference
 *     invalid endpoint
 *     incompatible endpoint kinds
 *     contradictory topology requirements
 *     unsatisfiable constraints
 *     invalid property type
 *     invalid topology relationship
 *     unsupported topology semantic
 *     unavailable capability
 *     insufficient resources
 *     forbidden policy
 *     invalid placement
 *     invalid routing requirement
 *
 * Semantic errors MUST NOT be encoded as parser actions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     grammar version
 *     lexer version
 *     parser configuration
 *
 * parsing is deterministic.
 *
 * No semantic decision may depend on:
 *
 *     hardware;
 *     runtime state;
 *     network state;
 *     wall-clock time;
 *     randomness;
 *     environment variables.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parse tree must preserve enough structure for the frontend to retain:
 *
 *     - declaration order;
 *     - member order;
 *     - endpoint order;
 *     - expression structure;
 *     - metadata order;
 *     - source locations.
 *
 * Semantic normalization occurs after parsing.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS
 * --------------
 *
 * [x] No fixed node count.
 * [x] No fixed edge count.
 * [x] No fixed link count.
 * [x] No fixed group count.
 * [x] No fixed topology count.
 * [x] No fixed machine count.
 * [x] No fixed CPU count.
 * [x] No fixed GPU count.
 * [x] No fixed FPGA count.
 * [x] No fixed QPU count.
 * [x] No fixed memory capacity.
 * [x] No fixed network size.
 * [x] No fixed topology family enumeration.
 * [x] No vendor enumeration.
 * [x] No transport enumeration.
 * [x] No physical device enumeration.
 * [x] No physical address syntax.
 * [x] No routing algorithm.
 * [x] No scheduling algorithm.
 * [x] No resource allocation.
 * [x] No hardware discovery.
 * [x] No runtime actions.
 * [x] No embedded Rust.
 * [x] No unsafe code.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * topology cluster {
 *     node producer;
 *     node consumer;
 *     edge(producer, consumer);
 * }
 *
 *
 * topology logical_network {
 *     node producer;
 *     node consumer;
 *
 *     edge(producer, consumer) {
 *         latency: latency_budget;
 *     }
 * }
 *
 *
 * topology distributed_execution {
 *     group workers = worker_set;
 *
 *     requires capability("distributed.communication");
 *     requires topology(required_topology);
 *
 *     prefer locality;
 *     hint hierarchical;
 * }
 *
 *
 * topology hybrid_execution {
 *     node classical_domain;
 *     node quantum_domain;
 *
 *     link(classical_domain, quantum_domain) {
 *         semantics: control;
 *     }
 * }
 *
 *
 * topology parameterized<NodeKind> {
 *     node root;
 * }
 *
 *
 * NEGATIVE
 * --------
 *
 * topology
 *
 * topology {
 *
 * topology cluster {
 *     node;
 *
 * topology cluster {
 *     edge();
 * }
 *
 * topology cluster {
 *     edge(a);
 * }
 *
 * topology cluster {
 *     edge(a, );
 * }
 *
 *
 * SEMANTIC NEGATIVES
 * ------------------
 *
 *     edge(unknown_a, unknown_b);
 *
 *     requires impossible_requirement;
 *
 *     requires unavailable_capability;
 *
 * These must parse structurally and fail during semantic validation when
 * appropriate.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Stress tests MUST parameterize topology size externally.
 *
 * Test generators may vary:
 *
 *     - number of topology members;
 *     - number of nodes;
 *     - number of groups;
 *     - number of relationships;
 *     - endpoint cardinality;
 *     - metadata depth;
 *     - expression complexity;
 *     - generic parameter count;
 *     - declaration count.
 *
 * No stress-test value may become a language-level maximum.
 *
 * The language is therefore open-ended with respect to topology cardinality.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The topology grammar must compose with:
 *
 *     classical computation;
 *     quantum computation;
 *     HDL;
 *     hardware intent;
 *     AI/learning;
 *     data processing;
 *     concurrency;
 *     networking;
 *     resource requirements;
 *     effects;
 *     policies;
 *     contracts;
 *     provenance.
 *
 * Example semantic intent:
 *
 *     topology hybrid_system {
 *         node classical;
 *         node quantum;
 *         node accelerator;
 *
 *         link(classical, quantum);
 *         link(classical, accelerator);
 *
 *         requires capability("quantum.measurement");
 *         requires capability("tensor.compute");
 *     }
 *
 * The topology grammar does not need to know how those capabilities are
 * physically realized.
 *
 * ============================================================================
 * INTEGRATION WITH DISTRIBUTED COMPOSITION
 * ============================================================================
 *
 * `grammar/distributed/distributed.g4` already establishes:
 *
 *     distributedTopologyConstruct
 *
 * as the topology composition boundary.
 *
 * It must continue to import:
 *
 *     DistributedTopology
 *
 * and consume:
 *
 *     distributedTopologyConstruct
 *
 * It must NOT:
 *
 *     - duplicate topology declaration syntax;
 *     - define a second topology rule;
 *     - introduce topology-family enumeration;
 *     - perform physical topology realization.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING FILES
 * ============================================================================
 *
 * REQUIRED EXISTING INTEGRATIONS
 * ------------------------------
 *
 * grammar/core/names.g4
 *     -> identifier
 *     -> qualifiedName
 *
 * grammar/expressions/
 *     -> expression
 *     -> optionalExpressionList
 *
 * grammar/distributed/distributed.g4
 *     -> distributedTopologyConstruct
 *
 * grammar/distributed/placement.g4
 *     -> placement semantics
 *
 * grammar/distributed/communication.g4
 *     -> communication semantics
 *
 * grammar/distributed/messaging.g4
 *     -> messaging semantics
 *
 * grammar/distributed/replication.g4
 *     -> replication semantics
 *
 * grammar/networking/
 *     -> concrete transport/network semantics
 *
 * grammar/resources/
 *     -> resource/capability semantics
 *
 * grammar/security/
 *     -> authorization/security semantics
 *
 * grammar/policies/
 *     -> policy semantics
 *
 * grammar/execution/
 *     -> execution/placement/scheduling semantics
 *
 * grammar/hardware/
 *     -> physical hardware topology and realization
 *
 * grammar/quantum/
 *     -> quantum semantics
 *
 * quantum::ir
 *     -> canonical quantum IR boundary
 *
 * ============================================================================
 * LEXER INTEGRATION REQUIRED BEFORE MERGE
 * ============================================================================
 *
 * The repository currently uses a canonical lexical architecture.
 *
 * This file intentionally avoids requiring new topology-specific lexer rules.
 *
 * Therefore the immediate topology grammar has no dependency on new:
 *
 *     TOPOLOGY
 *     EDGE
 *     LINK
 *
 * lexer tokens.
 *
 * This avoids the current inconsistency where topology syntax referenced
 * lexical tokens that were not consistently established by the canonical
 * lexical vocabulary.
 *
 * If the language specification later promotes these words to reserved
 * keywords, the promotion must occur centrally in:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/keywords.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and ONLY the three contextual adapter rules in this file should change.
 *
 * ============================================================================
 * AST INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * The frontend AST must not infer physical realization from this grammar.
 *
 * A topology node must remain a logical topology node.
 *
 * It must not automatically become:
 *
 *     CpuNode
 *     GpuNode
 *     FpgaNode
 *     AsicNode
 *     QpuNode
 *     CloudNode
 *
 * without explicit downstream semantic information.
 *
 * ============================================================================
 * QUANTUM INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * A logical distributed quantum topology must remain independent from physical
 * quantum topology.
 *
 * The source path is:
 *
 *     topology intent
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic distributed model
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * This grammar must never perform quantum routing.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * The grammar itself contains no Rust implementation.
 *
 * Generated parser/frontend integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] There is one public topology entry point.
 * [x] Names come from the canonical names grammar.
 * [x] Expressions come from the canonical expression grammar.
 * [x] Topology families are open-world.
 * [x] Node cardinality is unbounded at language level.
 * [x] Group cardinality is unbounded at language level.
 * [x] Relationship cardinality is unbounded at language level.
 * [x] Endpoint cardinality is unbounded at language level.
 * [x] No physical topology is encoded.
 * [x] No routing is encoded.
 * [x] No scheduling is encoded.
 * [x] No placement is encoded.
 * [x] No transport is encoded.
 * [x] No hardware inventory is encoded.
 * [x] No quantum physical topology is encoded.
 * [x] No quantum gate set is encoded.
 * [x] No QEC is encoded.
 * [x] No ZQN is encoded.
 * [x] No IR is created.
 * [x] No Rust actions exist.
 * [x] No unsafe implementation is required.
 * [x] No resource limits are encoded.
 * [x] No vendor limits are encoded.
 * [x] No fixed topology-family enumeration exists.
 * [x] Member bodies cannot recursively declare entire topologies.
 * [x] Structured values use the canonical expression system.
 * [x] Distributed composition consumes the public entry point.
 * [x] Semantic validation remains downstream.
 * [x] Provenance can be preserved.
 * [x] Deterministic parsing is preserved.
 *
 * Repository integration must additionally verify:
 *
 * [ ] `distributed.g4` imports this grammar exactly once.
 * [ ] No other distributed grammar defines a competing topology root.
 * [ ] Frontend AST preserves topology source structure.
 * [ ] Semantic analysis recognizes the contextual `topology`, `edge` and
 *     `link` spellings.
 * [ ] Topology tests exist under `grammar/tests/distributed/`.
 * [ ] Cross-domain tests exist.
 * [ ] Negative tests exist.
 * [ ] Scalability tests exist.
 * [ ] Determinism tests exist.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 *     topology syntax
 *          |
 *          v
 *     logical topology intent
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          +--> resources
 *          +--> capabilities
 *          +--> effects
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     placement / routing / scheduling
 *          |
 *          v
 *     target realization
 *
 * Therefore:
 *
 *     SOURCE TOPOLOGY
 *
 * never becomes:
 *
 *     PHYSICAL TOPOLOGY
 *
 * merely because it is compiled.
 *
 * The same source topology can be retained while its realization changes
 * across machine sizes, hardware classes, deployment environments and future
 * computational substrates.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */