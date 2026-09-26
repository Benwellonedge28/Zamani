/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/topology.g4
 *
 * Grammar:
 *     DistributedTopology
 *
 * Status:
 *     Production distributed-topology parser component
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No environment queries.
 *     - No target discovery.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL DISTRIBUTED TOPOLOGY INTENT.
 *
 * A distributed topology describes logical relationships between distributed
 * computational entities.
 *
 * It does NOT describe a particular physical network, machine, chip,
 * processor, accelerator, QPU, FPGA fabric, rack, cloud region, cable,
 * physical link, routing table, or hardware inventory.
 *
 * The topology therefore remains portable across:
 *
 *     - a single execution resource;
 *     - multiple processes;
 *     - multiple machines;
 *     - clusters;
 *     - HPC systems;
 *     - clouds;
 *     - edge systems;
 *     - heterogeneous systems;
 *     - CPU/GPU/FPGA systems;
 *     - quantum-classical systems;
 *     - distributed quantum systems;
 *     - future computational substrates.
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Topology answers:
 *
 *     "What logical relationship between computational entities is required
 *      or preferred?"
 *
 * It does NOT answer:
 *
 *     "Which physical resources realize that relationship?"
 *
 * Therefore:
 *
 *     topology
 *         |
 *         v
 *     placement
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A topology declaration describes portable semantic intent.
 *
 * The same source topology can be lowered differently depending on available:
 *
 *     resources;
 *     capabilities;
 *     interconnects;
 *     execution domains;
 *     network technologies;
 *     accelerators;
 *     quantum devices;
 *     deployment environments.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed topology declarations;
 *     - topology names;
 *     - logical topology members;
 *     - logical nodes;
 *     - logical groups;
 *     - logical edges;
 *     - logical links;
 *     - topology relationships;
 *     - topology properties;
 *     - topology requirements;
 *     - topology constraints;
 *     - topology preferences;
 *     - topology hints;
 *     - topology predicates;
 *     - topology metadata;
 *     - topology extensions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - generic type semantics;
 *     - physical hardware topology;
 *     - network transport;
 *     - network addresses;
 *     - sockets;
 *     - packets;
 *     - routing algorithms;
 *     - placement algorithms;
 *     - scheduling algorithms;
 *     - resource allocation;
 *     - device discovery;
 *     - replication;
 *     - consistency;
 *     - consensus;
 *     - fault tolerance;
 *     - recovery;
 *     - QEC;
 *     - ZQN;
 *     - quantum gates;
 *     - quantum::ir;
 *     - hardware calibration;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     ZamaniLexer
 *          |
 *          +--> Names
 *          |
 *          +--> Expressions
 *          |
 *          v
 *     DistributedTopology
 *
 * This grammar deliberately does NOT define:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     operators
 *     punctuation
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Topology families are semantic names rather than closed grammar
 * alternatives.
 *
 * The grammar MUST NOT enumerate:
 *
 *     ring
 *     mesh
 *     torus
 *     tree
 *     star
 *     grid
 *     hypercube
 *     fat_tree
 *     dragonfly
 *     heavy_hex
 *     butterfly
 *     fully_connected
 *     sparse
 *
 * Any such topology can be represented as a name/property/relationship and
 * classified by semantic analysis.
 *
 * Future topology families therefore do not require parser modification.
 *
 * ============================================================================
 * NO HARD-CODED LIMITS
 * ============================================================================
 *
 * This grammar contains no limits on:
 *
 *     nodes;
 *     groups;
 *     edges;
 *     links;
 *     topology declarations;
 *     topology dimensions;
 *     topology properties;
 *     relationship count;
 *     participant count;
 *     machine count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     memory;
 *     bandwidth;
 *     devices;
 *     execution domains.
 *
 * There is deliberately no:
 *
 *     MAX_NODES
 *     MAX_EDGES
 *     MAX_LINKS
 *     MAX_GROUPS
 *     MAX_TOPOLOGIES
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *
 * ANTLR repetition operators remain unbounded at the language level.
 *
 * ============================================================================
 * REQUIREMENT / REALIZATION SEPARATION
 * ============================================================================
 *
 * A topology may express:
 *
 *     requires capability("communication");
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     constraint latency <= latency_budget;
 *
 *     prefer locality;
 *
 *     hint hierarchical;
 *
 * These are semantic statements.
 *
 * They do not select a physical network or machine.
 *
 * ============================================================================
 * TOPOLOGY / PLACEMENT SEPARATION
 * ============================================================================
 *
 * Topology:
 *
 *     describes relationships.
 *
 * Placement:
 *
 *     determines where logical entities may be realized.
 *
 * Routing:
 *
 *     determines paths through an actual realizable topology.
 *
 * Scheduling:
 *
 *     determines temporal resource usage.
 *
 * This grammar therefore MUST NOT implement:
 *
 *     place(...)
 *     route(...)
 *     schedule(...)
 *
 * ============================================================================
 * TOPOLOGY / NETWORKING SEPARATION
 * ============================================================================
 *
 * A logical topology edge does not imply:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     Ethernet;
 *     wireless;
 *     optical transport;
 *     quantum communication hardware.
 *
 * The networking subsystem determines the concrete transport.
 *
 * ============================================================================
 * TOPOLOGY / HARDWARE SEPARATION
 * ============================================================================
 *
 * A logical topology node is not a physical device.
 *
 * It may eventually be realized by:
 *
 *     a process;
 *     a machine;
 *     a CPU;
 *     a GPU;
 *     an FPGA;
 *     an accelerator;
 *     a QPU;
 *     a heterogeneous execution domain;
 *     a future execution substrate.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed topology may describe relationships between:
 *
 *     logical quantum execution domains;
 *     distributed quantum workloads;
 *     quantum/classical services;
 *     logical quantum resources;
 *     classical coordination domains.
 *
 * It MUST NOT define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     GateKind;
 *     coupling maps;
 *     SWAP insertion;
 *     pulse routing;
 *     calibration;
 *     QEC;
 *     ZQN;
 *     quantum::ir.
 *
 * Quantum semantics continue through:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Distributed topology may describe logical relationships involving hardware
 * or accelerator execution domains.
 *
 * It MUST NOT describe:
 *
 *     FPGA coordinates;
 *     ASIC coordinates;
 *     physical pins;
 *     physical registers;
 *     physical memory addresses;
 *     vendor-specific fabric topology;
 *     fixed device inventory.
 *
 * Hardware topology owns physical hardware topology.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser tree must preserve enough structure for a domain-neutral AST to
 * represent at least:
 *
 *     topology name;
 *     topology type/name;
 *     generic parameters;
 *     members;
 *     logical node names;
 *     group names;
 *     edge/link relationships;
 *     relationship direction;
 *     endpoint expressions;
 *     property expressions;
 *     requirement expressions;
 *     constraint expressions;
 *     preference expressions;
 *     hint expressions;
 *     predicate expressions;
 *     metadata;
 *     source ordering;
 *     source spans.
 *
 * Conceptual AST:
 *
 *     TopologyDeclaration
 *         name
 *         topology_kind
 *         parameters
 *         members
 *         source_span
 *
 *     TopologyNode
 *     TopologyGroup
 *     TopologyEdge
 *     TopologyLink
 *     TopologyProperty
 *     TopologyRequirement
 *     TopologyConstraint
 *     TopologyPreference
 *     TopologyHint
 *     TopologyPredicate
 *     TopologyMetadata
 *
 * These are AST/semantic concepts.
 *
 * This grammar does not define Rust AST types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving topology names;
 *     - resolving logical node references;
 *     - resolving groups;
 *     - checking endpoint compatibility;
 *     - validating relationship direction;
 *     - validating topology predicates;
 *     - checking property types;
 *     - checking units;
 *     - checking requirements;
 *     - checking constraints;
 *     - checking preferences;
 *     - checking hints;
 *     - checking satisfiability;
 *     - checking contradictory relationships;
 *     - checking topology/placement interaction;
 *     - checking topology/routing interaction;
 *     - checking topology/network interaction;
 *     - checking topology/hardware interaction;
 *     - checking quantum-topology compatibility.
 *
 * The parser performs none of these checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Topology syntax lowers through the domain-neutral AST and semantic model.
 *
 * Distributed semantic information may subsequently participate in distributed
 * compiler/runtime representations.
 *
 * Quantum-related information continues through:
 *
 *     quantum::ir
 *
 * There must be no:
 *
 *     DistributedTopologyIR
 *
 * created merely because the syntax originated here if an existing canonical
 * semantic representation already owns the information.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The parser structure must preserve:
 *
 *     - topology declaration ordering;
 *     - member ordering;
 *     - relationship ordering;
 *     - endpoint ordering;
 *     - expression structure;
 *     - metadata ordering;
 *     - source spans.
 *
 * Semantic normalization occurs after parsing.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no hardware queries;
 *     - no network queries;
 *     - no runtime callbacks;
 *     - no randomness.
 *
 * Identical token streams under the same grammar/version produce the same
 * parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Topology declarations are declarative.
 *
 * Parsing MUST NOT:
 *
 *     - contact a network;
 *     - inspect hardware;
 *     - discover devices;
 *     - allocate resources;
 *     - execute commands;
 *     - access credentials;
 *     - alter deployment state.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * The only public entry point owned by this file is:
 *
 *     distributedTopologyConstruct
 *
 * Higher-level composition should consume this rule.
 *
 * ============================================================================
 */

parser grammar DistributedTopology;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Exactly one public entry point is exposed.
 */
distributedTopologyConstruct
    : distributedTopologyDeclaration
    ;


/* ============================================================================
 * 2. TOPOLOGY DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     topology cluster {
 *         nodes: workers;
 *         edges: links;
 *     }
 *
 * The `TOPOLOGY` token is intentionally owned by the canonical lexer rather
 * than defined locally.
 *
 * If the lexical vocabulary has not yet been promoted to include TOPOLOGY,
 * that promotion must be made centrally in:
 *
 *     grammar/lexer/keywords.g4
 *
 * and propagated through:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar MUST NOT define a private topology token.
 */
distributedTopologyDeclaration
    : topologyVisibility?
      topologyModifier*
      TOPOLOGY
      qualifiedName
      distributedTopologyTypeClause?
      distributedTopologyGenericParameters?
      distributedTopologyExtendsClause?
      distributedTopologyBody
    ;


/* ============================================================================
 * 3. VISIBILITY
 * ============================================================================
 */

topologyVisibility
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/* ============================================================================
 * 4. MODIFIERS
 * ============================================================================
 *
 * Only shared lexical modifiers are accepted.
 *
 * No topology-specific modifier vocabulary is created here.
 */
topologyModifier
    : STATIC
    | ABSTRACT
    | FINAL
    ;


/* ============================================================================
 * 5. OPTIONAL TOPOLOGY TYPE
 * ============================================================================
 *
 * Examples:
 *
 *     topology cluster ...
 *
 *     topology logical_network ...
 *
 *     topology distributed_execution ...
 *
 * The type is a semantic name.
 *
 * No closed topology-family enumeration is created.
 */
distributedTopologyTypeClause
    : COLON
      qualifiedName
    ;


/* ============================================================================
 * 6. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters represent semantic parameters.
 *
 * They are NOT machine-size limits.
 */
distributedTopologyGenericParameters
    : LESS_THAN
      distributedTopologyGenericParameter
      (COMMA distributedTopologyGenericParameter)*
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
 * 7. EXTENSION
 * ============================================================================
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
 * 8. TOPOLOGY BODY
 * ============================================================================
 *
 * A topology body contains declarations and semantic specifications.
 *
 * Repetition is deliberately unbounded.
 */
distributedTopologyBody
    : LBRACE
      distributedTopologyMember*
      RBRACE
    ;


/* ============================================================================
 * 9. TOPOLOGY MEMBER
 * ============================================================================
 *
 * Each member has a distinct structural shape.
 */
distributedTopologyMember
    : distributedTopologyNode
    | distributedTopologyGroup
    | distributedTopologyEdge
    | distributedTopologyLink
    | distributedTopologyProperty
    | distributedTopologyRequirement
    | distributedTopologyConstraint
    | distributedTopologyPreference
    | distributedTopologyHint
    | distributedTopologyPredicate
    | distributedTopologySection
    ;


/* ============================================================================
 * 10. LOGICAL NODE
 * ============================================================================
 *
 * Node is a LOGICAL topology participant.
 *
 * It is not a physical machine/device.
 *
 * Canonical form:
 *
 *     node worker;
 *
 *     node worker {
 *         role: compute;
 *     }
 *
 * Because NODE is a shared lexical concept used by the topology family, it
 * must be supplied by the canonical lexer rather than locally invented.
 */
distributedTopologyNode
    : NODE
      identifier
      distributedTopologyMemberBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 11. LOGICAL GROUP
 * ============================================================================
 *
 * A group contains a logical collection expression.
 *
 * Canonical examples:
 *
 *     group workers;
 *
 *     group workers = worker_set;
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
 * 12. LOGICAL EDGE
 * ============================================================================
 *
 * Edge describes a relationship between logical topology participants.
 *
 * Canonical forms:
 *
 *     edge(a, b);
 *
 *     edge(a, b, relationship);
 *
 *     edge(a, b) {
 *         latency: requirement;
 *     }
 *
 * No physical route is implied.
 */
distributedTopologyEdge
    : EDGE
      LPAREN
      distributedTopologyEndpointList
      RPAREN
      distributedTopologyMemberBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 13. LOGICAL LINK
 * ============================================================================
 *
 * Link is an explicit logical relationship construct.
 *
 * It does not select a network transport.
 */
distributedTopologyLink
    : LINK
      LPAREN
      distributedTopologyEndpointList
      RPAREN
      distributedTopologyMemberBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 14. ENDPOINT LIST
 * ============================================================================
 *
 * Endpoint count is unrestricted at the language level.
 */
distributedTopologyEndpointList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 15. PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     property_name: expression;
 *
 * Property names remain semantic identifiers.
 *
 * This prevents the grammar from growing a new keyword for every future
 * topology characteristic.
 */
distributedTopologyProperty
    : identifier
      COLON
      distributedTopologyValue
      SEMICOLON?
    ;


/* ============================================================================
 * 16. REQUIREMENT
 * ============================================================================
 *
 * Requirement means the condition MUST be satisfiable by the realization.
 *
 * Examples:
 *
 *     requires capability("communication");
 *
 *     requires bandwidth >= required_bandwidth;
 */
distributedTopologyRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 17. CONSTRAINT
 * ============================================================================
 *
 * Constraint restricts legal realization.
 */
distributedTopologyConstraint
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 18. PREFERENCE
 * ============================================================================
 *
 * Preference is advisory optimization intent.
 */
distributedTopologyPreference
    : PREFER
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 19. HINT
 * ============================================================================
 *
 * Hint is advisory metadata.
 */
distributedTopologyHint
    : HINT
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 20. PREDICATE
 * ============================================================================
 *
 * A predicate provides a semantic condition without making it a hardware
 * decision.
 *
 * The predicate name remains an identifier.
 */
distributedTopologyPredicate
    : identifier
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 21. NESTED SECTION
 * ============================================================================
 *
 * Nested sections allow future topology metadata without changing the
 * grammar.
 *
 * Example:
 *
 *     properties {
 *         locality: logical;
 *     }
 */
distributedTopologySection
    : identifier
      distributedTopologyMemberBody
    ;


/* ============================================================================
 * 22. MEMBER BODY
 * ============================================================================
 *
 * Node/group/link metadata uses the same controlled member vocabulary.
 *
 * Arbitrary statements are deliberately excluded.
 */
distributedTopologyMemberBody
    : LBRACE
      distributedTopologyMember*
      RBRACE
    ;


/* ============================================================================
 * 23. VALUE
 * ============================================================================
 *
 * Ordinary computation delegates to the canonical expression grammar.
 *
 * Structured values are supported without introducing another data language.
 */
distributedTopologyValue
    : expression
    | distributedTopologyObject
    | distributedTopologyList
    ;


/* ============================================================================
 * 24. OBJECT
 * ============================================================================
 */

distributedTopologyObject
    : LBRACE
      distributedTopologyObjectMember*
      RBRACE
    ;


distributedTopologyObjectMember
    : identifier
      COLON
      distributedTopologyValue
      SEMICOLON?
    ;


/* ============================================================================
 * 25. LIST
 * ============================================================================
 *
 * No finite cardinality is imposed.
 */
distributedTopologyList
    : LBRACKET
      distributedTopologyListElement*
      RBRACKET
    ;


distributedTopologyListElement
    : expression
    | distributedTopologyObject
    | distributedTopologyList
    ;


/*
 * ============================================================================
 * END OF DISTRIBUTED TOPOLOGY GRAMMAR
 * ============================================================================
 *
 * Integration invariants:
 *
 *     - no hardware selection;
 *     - no physical topology;
 *     - no routing;
 *     - no placement;
 *     - no scheduling;
 *     - no transport;
 *     - no resource allocation;
 *     - no quantum IR;
 *     - no QEC;
 *     - no ZQN;
 *     - no finite resource limits;
 *     - no operation enumeration;
 *     - no vendor enumeration;
 *     - no embedded Rust;
 *     - no unsafe code;
 *     - deterministic parsing;
 *     - source-span preservation;
 *     - open-world topology vocabulary.
 *
 * ============================================================================
 */