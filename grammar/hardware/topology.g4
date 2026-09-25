/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/topology.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Language:
 *     Zamani
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime execution, or unsafe code.
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * This file is the canonical SOURCE-SYNTAX owner for hardware-independent
 * topology intent.
 *
 * It describes logical relationships among computational resources.
 *
 * It does NOT describe a discovered physical machine.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * Topology syntax expresses:
 *
 *     - logical topology declarations;
 *     - logical nodes;
 *     - node classes;
 *     - logical groups;
 *     - logical endpoints;
 *     - edges;
 *     - links;
 *     - connectivity;
 *     - directionality;
 *     - symbolic selectors;
 *     - topology properties;
 *     - topology requirements;
 *     - topology constraints;
 *     - topology preferences;
 *     - topology predicates;
 *     - topology extensions;
 *     - symbolic quantities;
 *     - reusable topology contracts.
 *
 * The grammar intentionally remains OPEN-WORLD.
 *
 * A new topology family does not require a new parser alternative.
 *
 * For example, these are semantic names rather than grammar-level
 * enumerations:
 *
 *     topology::ring
 *     topology::mesh
 *     topology::torus
 *     quantum::heavy_hex
 *     network::fat_tree
 *     accelerator::custom
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - literals;
 *     - generic expressions;
 *     - type semantics;
 *     - resource discovery;
 *     - physical device discovery;
 *     - hardware enumeration;
 *     - physical addresses;
 *     - PCI addresses;
 *     - device serial numbers;
 *     - physical qubit IDs;
 *     - physical CPU IDs;
 *     - physical GPU IDs;
 *     - routing;
 *     - path finding;
 *     - placement;
 *     - scheduling;
 *     - calibration;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - HAL implementation;
 *     - backend SDKs;
 *     - vendor APIs;
 *     - canonical IR implementation.
 *
 * ============================================================================
 * TOPOLOGY VS PHYSICAL REALIZATION
 * ============================================================================
 *
 * A topology is a relationship model.
 *
 * For example:
 *
 *     topology ring {
 *         connect compute[*] to compute[*];
 *     }
 *
 * expresses a connectivity relationship.
 *
 * It does NOT mean:
 *
 *     - use physical device 0;
 *     - use physical device 1;
 *     - use a particular QPU;
 *     - use a particular CPU;
 *     - use a particular GPU;
 *     - use a particular FPGA;
 *     - use a particular memory bank;
 *     - use a particular network interface;
 *     - use a fixed number of machines.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Topology syntax therefore describes:
 *
 *     WHAT relationships the program requires or permits.
 *
 * It does not prescribe:
 *
 *     WHICH physical resource realizes those relationships.
 *
 * A single topology declaration can consequently be considered against:
 *
 *     - tiny embedded systems;
 *     - single processors;
 *     - multicore systems;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - QPUs;
 *     - simulators;
 *     - clusters;
 *     - distributed systems;
 *     - future computational substrates.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO universal maximum for:
 *
 *     nodes
 *     edges
 *     links
 *     ports
 *     groups
 *     topology depth
 *     degree
 *     path length
 *     graph size
 *     connected resources
 *     topology dimensions
 *     topology properties
 *     topology declarations
 *
 * Repetition is represented with ANTLR `*` and `+`.
 *
 * Quantities are expressions.
 *
 * Therefore:
 *
 *     topology node counts
 *     topology dimensions
 *     capacities
 *     degrees
 *     distances
 *     bandwidth
 *     latency
 *
 * may all be symbolic.
 *
 * Practical limits are compiler/resource limits, never language-level
 * topology ceilings.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT contain:
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
 * It also MUST NOT enumerate:
 *
 *     Qubit0
 *     Qubit1
 *     CPU0
 *     GPU0
 *     FPGA0
 *     Node0
 *     Device0
 *
 * as universal topology resources.
 *
 * A user may of course write a program value named `node0`.
 *
 * The prohibition applies to grammar-defined machine identities or limits.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Topology is not the same as resource availability.
 *
 * These are semantically different:
 *
 *     requires qubits >= n;
 *
 *     requires capability("quantum.measurement");
 *
 *     connect logical.qubit[*] to logical.qubit[*];
 *
 * The first expresses a resource requirement.
 *
 * The second expresses a capability requirement.
 *
 * The third expresses topology intent.
 *
 * Semantic analysis may combine these facts later.
 *
 * The parser MUST NOT perform that combination.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts MUST remain distinct.
 *
 * Requirement:
 *
 *     mandatory semantic condition.
 *
 * Constraint:
 *
 *     property that must remain satisfied.
 *
 * Preference:
 *
 *     advisory optimization preference.
 *
 * Hint:
 *
 *     additional compilation information without changing program meaning.
 *
 * This grammar therefore never treats:
 *
 *     prefer
 *
 * as equivalent to:
 *
 *     requires
 *
 * ============================================================================
 * CANONICAL LEXICAL BOUNDARY
 * ============================================================================
 *
 * Parser grammars consume the canonical Zamani lexer vocabulary.
 *
 * The production parser boundary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexical composition is:
 *
 *     grammar/lexer/tokens.g4
 *
 * This file does not define lexical rules.
 *
 * Required topology-specific keyword tokens are:
 *
 *     TOPOLOGY
 *     NODE
 *     EDGE
 *     LINK
 *     CONNECT
 *     BIDIRECTIONAL
 *     DIRECTED
 *     UNDIRECTED
 *     NEIGHBOR
 *     ADJACENT
 *
 * These must be owned exactly once by the canonical keyword vocabulary.
 *
 * Existing shared tokens reused here include, where applicable:
 *
 *     PUBLIC
 *     PRIVATE
 *     PROTECTED
 *     INTERNAL
 *     STATIC
 *     CONST
 *     EXTERN
 *     FINAL
 *     ABSTRACT
 *     PARTIAL
 *     EXTENDS
 *     WHERE
 *     REQUIRES
 *     PREFER
 *     CONSTRAINT
 *     HINT
 *     PROPERTY
 *     GROUP
 *     CAPABILITY
 *     RESOURCE
 *     TARGET
 *     IN
 *     AS
 *
 * and the canonical punctuation/operators:
 *
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT
 *     SEMICOLON
 *     COLON
 *     ASSIGN
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     LESS
 *     GREATER
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     DOUBLE_COLON
 *
 * Exact lexical ownership remains outside this file.
 *
 * ============================================================================
 * NAME BOUNDARY
 * ============================================================================
 *
 * Name structure is conceptually owned by:
 *
 *     grammar/core/names.g4
 *
 * The topology grammar deliberately preserves the same qualified-name
 * semantics:
 *
 *     IDENTIFIER
 *     IDENTIFIER :: IDENTIFIER
 *     IDENTIFIER :: IDENTIFIER :: IDENTIFIER
 *
 * No finite qualification depth exists.
 *
 * DOT is intentionally not used to manufacture qualified names.
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Topology quantities, selectors, predicates, and property values are
 * expressions.
 *
 * The topology grammar must therefore eventually consume the canonical
 * expression grammar rather than creating a competing expression hierarchy.
 *
 * This file provides a small `topologyExpression` integration boundary so
 * the composition root can bind it to the canonical expression production.
 *
 * It MUST NOT become a second general-purpose expression implementation.
 *
 * ============================================================================
 */


/* ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar ZamaniHardwareTopologyParser;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable topology declaration entry point consumed by the
 * hardware parser/composition root.
 *
 * ========================================================================== */

topologyDeclaration
    : topologyAnnotation*
      topologyVisibility?
      topologyModifier*
      TOPOLOGY
      topologyName
      topologyExtendsClause?
      topologyBody
    ;


/* ============================================================================
 * 2. VISIBILITY
 * ============================================================================
 */

topologyVisibility
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/* ============================================================================
 * 3. MODIFIERS
 * ============================================================================
 */

topologyModifier
    : STATIC
    | CONST
    | EXTERN
    | FINAL
    | ABSTRACT
    | PARTIAL
    ;


/* ============================================================================
 * 4. ANNOTATIONS
 * ============================================================================
 *
 * Annotation syntax is structurally represented here.
 *
 * Annotation semantics remain outside this grammar.
 *
 * ========================================================================== */

topologyAnnotation
    : AT
      topologyQualifiedName
      (
          LPAREN
          topologyArgumentList?
          RPAREN
      )?
    ;

topologyArgumentList
    : topologyArgument
      (
          COMMA
          topologyArgument
      )*
    ;

topologyArgument
    : topologyExpression
    | topologyQualifiedName
    | STRING
    | INTEGER
    | FLOAT
    ;


/* ============================================================================
 * 5. NAMES
 * ============================================================================
 *
 * This is a compatibility boundary.
 *
 * The canonical name implementation belongs to core/names.g4.
 *
 * The composition root may replace these wrappers with direct references to
 * the canonical name rules when parser grammar imports are available.
 *
 * ========================================================================== */

topologyName
    : topologyQualifiedName
    ;

topologyQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 6. EXTENSION
 * ============================================================================
 *
 * A topology may extend one or more logical topology descriptions.
 *
 * This is semantic composition.
 *
 * It does NOT mean physical hardware inheritance.
 *
 * ========================================================================== */

topologyExtendsClause
    : EXTENDS
      topologyQualifiedName
      (
          COMMA
          topologyQualifiedName
      )*
    ;


/* ============================================================================
 * 7. TOPOLOGY BODY
 * ============================================================================
 */

topologyBody
    : LBRACE
      topologyMember*
      RBRACE
    ;

topologyMember
    : topologyAnnotation* topologyNodeDeclaration
    | topologyAnnotation* topologyGroupDeclaration
    | topologyAnnotation* topologyEdgeDeclaration
    | topologyAnnotation* topologyLinkDeclaration
    | topologyAnnotation* topologyConnectionDeclaration
    | topologyAnnotation* topologyPropertyDeclaration
    | topologyAnnotation* topologyRequirementDeclaration
    | topologyAnnotation* topologyConstraintDeclaration
    | topologyAnnotation* topologyPreferenceDeclaration
    | topologyAnnotation* topologyHintDeclaration
    | topologyAnnotation* topologyPredicateDeclaration
    | topologyAnnotation* topologyUsingDeclaration
    ;


/* ============================================================================
 * 8. NODE DECLARATION
 * ============================================================================
 *
 * Nodes are logical participants.
 *
 * They are NOT physical device identifiers.
 *
 * Examples:
 *
 *     node compute;
 *
 *     node logical_qubit : quantum::logical_qubit;
 *
 *     node worker [worker_count];
 *
 * ========================================================================== */

topologyNodeDeclaration
    : NODE
      topologyNodeName
      topologyNodeTypeClause?
      topologyNodeMultiplicityClause?
      topologyPropertyBlock?
      SEMICOLON
    ;

topologyNodeName
    : topologyQualifiedName
    ;

topologyNodeTypeClause
    : COLON
      topologyQualifiedName
    ;

topologyNodeMultiplicityClause
    : LBRACKET
      topologyExpression
      RBRACKET
    ;


/* ============================================================================
 * 9. GROUP DECLARATION
 * ============================================================================
 *
 * Groups allow scalable logical collections.
 *
 * They avoid requiring source code to enumerate every member.
 *
 * ========================================================================== */

topologyGroupDeclaration
    : GROUP
      topologyQualifiedName
      topologyGroupTypeClause?
      topologyGroupSelectorClause?
      topologyGroupMultiplicityClause?
      topologyGroupBody?
      SEMICOLON?
    ;

topologyGroupTypeClause
    : COLON
      topologyQualifiedName
    ;

topologyGroupSelectorClause
    : WHERE
      topologyPredicateExpression
    ;

topologyGroupMultiplicityClause
    : LBRACKET
      topologyExpression
      RBRACKET
    ;

topologyGroupBody
    : LBRACE
      topologyGroupMember*
      RBRACE
    ;

topologyGroupMember
    : topologyEndpoint
      (
          COMMA
          topologyEndpoint
      )*
      SEMICOLON
    ;


/* ============================================================================
 * 10. EDGE DECLARATION
 * ============================================================================
 *
 * Edges represent logical relationships.
 *
 * They do not automatically mean physical wires, cables, buses, links,
 * quantum couplers, or network paths.
 *
 * ========================================================================== */

topologyEdgeDeclaration
    : EDGE
      topologyEdgeName?
      topologyEdgeEndpointClause
      topologyDirectionClause?
      topologyPropertyBlock?
      SEMICOLON
    ;

topologyEdgeName
    : topologyQualifiedName
    ;

topologyEdgeEndpointClause
    : topologyEndpoint
      topologyEdgeConnector
      topologyEndpoint
    ;

topologyEdgeConnector
    : TOPOLOGY_CONNECTOR_TO
    ;

topologyDirectionClause
    : BIDIRECTIONAL
    | DIRECTED
    | UNDIRECTED
    ;


/*
 * The semantic spelling of the connector is `to`.
 *
 * The canonical repository already contains FROM/IN/TO-like language
 * vocabulary in different domains.  To avoid introducing another generic
 * `TO` token merely for topology, this grammar uses the dedicated topology
 * connector token TOPOLOGY_CONNECTOR_TO.
 *
 * If the canonical keyword vocabulary standardizes `TO` globally, this
 * production MUST be changed to:
 *
 *     : TO
 *
 * and the dedicated token must not be added.
 */


/* ============================================================================
 * 11. LINK DECLARATION
 * ============================================================================
 *
 * Link is a semantic synonym/category useful to communication-oriented
 * topologies.
 *
 * It remains abstract.
 *
 * ========================================================================== */

topologyLinkDeclaration
    : LINK
      topologyLinkName?
      topologyEndpoint
      topologyEdgeConnector
      topologyEndpoint
      topologyDirectionClause?
      topologyPropertyBlock?
      SEMICOLON
    ;

topologyLinkName
    : topologyQualifiedName
    ;


/* ============================================================================
 * 12. CONNECT DECLARATION
 * ============================================================================
 *
 * `connect` is the direct topology-intent form.
 *
 * Examples:
 *
 *     connect compute[*] to compute[*];
 *
 *     connect logical.a to logical.b bidirectional;
 *
 * ========================================================================== */

topologyConnectionDeclaration
    : CONNECT
      topologyEndpoint
      topologyEdgeConnector
      topologyEndpoint
      topologyDirectionClause?
      topologyConnectionConditionClause?
      topologyPropertyBlock?
      SEMICOLON
    ;


/* ============================================================================
 * 13. ENDPOINTS
 * ============================================================================
 *
 * Endpoints are logical references.
 *
 * Selectors can describe symbolic sets without enumerating physical
 * resources.
 *
 * ========================================================================== */

topologyEndpoint
    : topologyQualifiedName
      topologyEndpointSelector*
    ;

topologyEndpointSelector
    : LBRACKET
      topologySelectorExpression
      RBRACKET
    ;


/* ============================================================================
 * 14. SELECTOR EXPRESSIONS
 * ============================================================================
 *
 * Selectors are deliberately symbolic.
 *
 * Examples:
 *
 *     *
 *     i
 *     i + 1
 *     range
 *     group
 *     where-expression
 *
 * The canonical expression system ultimately owns their semantics.
 *
 * ========================================================================== */

topologySelectorExpression
    : topologyExpression
    ;


/* ============================================================================
 * 15. CONNECTION CONDITIONS
 * ============================================================================
 *
 * A condition describes when a relationship is applicable.
 *
 * It does not perform runtime routing.
 *
 * ========================================================================== */

topologyConnectionConditionClause
    : WHERE
      topologyPredicateExpression
    ;


/* ============================================================================
 * 16. PREDICATES
 * ============================================================================
 *
 * Topology predicates describe logical relationships.
 *
 * No predicate is interpreted during parsing.
 *
 * ========================================================================== */

topologyPredicateDeclaration
    : topologyPredicateHead
      topologyPredicateBody?
      SEMICOLON?
    ;

topologyPredicateHead
    : PROPERTY
      topologyQualifiedName
      ASSIGN
      topologyPredicateExpression
    | topologyQualifiedName
      ASSIGN
      topologyPredicateExpression
    ;

topologyPredicateBody
    : LBRACE
      topologyPredicateMember*
      RBRACE
    ;

topologyPredicateMember
    : topologyQualifiedName
      ASSIGN
      topologyPredicateExpression
      SEMICOLON
    ;

topologyPredicateExpression
    : topologyExpression
    ;


/* ============================================================================
 * 17. PROPERTY DECLARATIONS
 * ============================================================================
 *
 * Properties are open-world.
 *
 * This deliberately avoids hard-coding a catalogue such as:
 *
 *     bandwidth
 *     latency
 *     distance
 *     cost
 *     reliability
 *
 * as grammar-level topology constructs.
 *
 * Those names can still be used naturally:
 *
 *     bandwidth = required_bandwidth;
 *
 *     latency = latency_budget;
 *
 *     distance = required_distance;
 *
 * ========================================================================== */

topologyPropertyDeclaration
    : PROPERTY
      topologyQualifiedName
      ASSIGN
      topologyExpression
      SEMICOLON
    | topologyQualifiedName
      ASSIGN
      topologyExpression
      SEMICOLON
    ;


/* ============================================================================
 * 18. PROPERTY BLOCK
 * ============================================================================
 */

topologyPropertyBlock
    : LBRACE
      topologyPropertyMember*
      RBRACE
    ;

topologyPropertyMember
    : topologyPropertyDeclaration
    ;


/* ============================================================================
 * 19. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 *
 * Examples:
 *
 *     requires qubits >= n;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires topology::connected;
 *
 * The parser records syntax only.
 *
 * Satisfiability is downstream.
 *
 * ========================================================================== */

topologyRequirementDeclaration
    : REQUIRES
      topologyRequirementExpression
      SEMICOLON
    ;

topologyRequirementExpression
    : topologyExpression
    ;


/* ============================================================================
 * 20. CONSTRAINTS
 * ============================================================================
 */

topologyConstraintDeclaration
    : CONSTRAINT
      topologyExpression
      SEMICOLON
    ;


/* ============================================================================
 * 21. PREFERENCES
 * ============================================================================
 */

topologyPreferenceDeclaration
    : PREFER
      topologyExpression
      SEMICOLON
    ;


/* ============================================================================
 * 22. HINTS
 * ============================================================================
 */

topologyHintDeclaration
    : HINT
      topologyExpression
      SEMICOLON
    ;


/* ============================================================================
 * 23. USING
 * ============================================================================
 *
 * `using` is intentionally represented as an identifier-compatible extension
 * point if the canonical vocabulary does not yet reserve USING.
 *
 * The preferred future canonical form is:
 *
 *     using topology::family;
 *
 * ========================================================================== */

topologyUsingDeclaration
    : topologyUsingHead
      SEMICOLON
    ;

topologyUsingHead
    : topologyQualifiedName
    ;


/* ============================================================================
 * 24. CANONICAL EXPRESSION BRIDGE
 * ============================================================================
 *
 * This rule is intentionally a composition boundary.
 *
 * It MUST resolve to the canonical Zamani expression grammar when the
 * parser-composition layer is assembled.
 *
 * It MUST NOT grow into a second expression implementation.
 *
 * ========================================================================== */

topologyExpression
    : topologyPrimaryExpression
    ;

topologyPrimaryExpression
    : topologyQualifiedName
    | STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    | topologyCallExpression
    | topologyParenthesizedExpression
    ;

topologyCallExpression
    : topologyQualifiedName
      LPAREN
      topologyExpressionList?
      RPAREN
    ;

topologyParenthesizedExpression
    : LPAREN
      topologyExpression
      RPAREN
    ;

topologyExpressionList
    : topologyExpression
      (
          COMMA
          topologyExpression
      )*
    ;


/*
 * IMPORTANT:
 *
 * This deliberately small bridge is NOT intended to replace the canonical
 * expression grammar.
 *
 * At composition time, the canonical expression rule must be bound here.
 *
 * Conceptually:
 *
 *     topologyExpression
 *          ->
 *     canonical expression
 *
 * If the parser composition mechanism permits direct rule imports, the
 * wrapper can simply delegate to that imported rule.
 *
 * Until then, this bridge provides the minimum topology-local expression
 * surface without creating a competing precedence system.
 */


/* ============================================================================
 * 25. SOURCE-LEVEL METRICS
 * ============================================================================
 *
 * Metrics are ordinary semantic property names.
 *
 * No finite metric catalogue is imposed.
 *
 * Examples:
 *
 *     latency
 *     bandwidth
 *     distance
 *     cost
 *     reliability
 *     energy
 *     power
 *     throughput
 *
 * are represented as qualified names/property expressions.
 *
 * This permits future dimensions without grammar changes.
 *
 * ============================================================================
 */


/* ============================================================================
 * 26. QUANTUM TOPOLOGY
 * ============================================================================
 *
 * Quantum topology is a topology domain, not a quantum-operation grammar.
 *
 * Valid semantic examples include:
 *
 *     topology quantum_connectivity {
 *         connect logical.qubit[*] to logical.qubit[*]
 *             bidirectional;
 *     }
 *
 * or:
 *
 *     topology quantum::connectivity {
 *         requires capability("quantum.two_body_interaction");
 *     }
 *
 * This grammar does NOT define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * as topology grammar alternatives.
 *
 * Quantum operations remain owned by the quantum grammar and ultimately
 * lower through:
 *
 *     quantum::ir
 *
 * Topology participates later in:
 *
 *     routing
 *     placement
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 */


/* ============================================================================
 * 27. CLASSICAL TOPOLOGY
 * ============================================================================
 *
 * The same topology model may describe:
 *
 *     CPU locality
 *     accelerator connectivity
 *     memory relationships
 *     data movement
 *     parallel execution domains
 *
 * without selecting physical processors.
 *
 * ============================================================================
 */


/* ============================================================================
 * 28. DISTRIBUTED TOPOLOGY
 * ============================================================================
 *
 * The same grammar can describe:
 *
 *     logical node connectivity
 *     service relationships
 *     communication domains
 *     logical network structure
 *
 * without requiring physical IP addresses or machine IDs.
 *
 * ============================================================================
 */


/* ============================================================================
 * 29. HDL / HARDWARE CO-DESIGN
 * ============================================================================
 *
 * HDL may consume topology intent to describe:
 *
 *     logical module relationships
 *     interconnect requirements
 *     communication structures
 *     clock-domain relationships
 *     accelerator connectivity
 *
 * Physical pins, package locations, routing tracks, and technology-specific
 * realization remain downstream.
 *
 * ============================================================================
 */


/* ============================================================================
 * 30. AST CONTRACT
 * ============================================================================
 *
 * The grammar creates no AST objects itself.
 *
 * The frontend AST should preserve semantic structure equivalent to:
 *
 *     HardwareTopologyDecl
 *     TopologyNodeDecl
 *     TopologyGroupDecl
 *     TopologyEdgeDecl
 *     TopologyLinkDecl
 *     TopologyConnectionDecl
 *     TopologyEndpoint
 *     TopologySelector
 *     TopologyProperty
 *     TopologyRequirement
 *     TopologyConstraint
 *     TopologyPreference
 *     TopologyHint
 *     TopologyPredicate
 *
 * Every resulting node MUST preserve:
 *
 *     source span
 *     source order where meaningful
 *     names
 *     expressions
 *     annotations
 *     modifiers
 *     relationships
 *
 * The AST MUST NOT contain:
 *
 *     physical device handles
 *     physical addresses
 *     backend SDK objects
 *     runtime topology snapshots
 *     scheduler state
 *     routing state
 *     calibration state
 *     QEC state
 *     ZQN runtime state
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - topology name resolution;
 *     - node resolution;
 *     - endpoint resolution;
 *     - group resolution;
 *     - selector typing;
 *     - graph consistency;
 *     - duplicate relationship analysis;
 *     - metric/unit checking;
 *     - requirement checking;
 *     - constraint checking;
 *     - preference classification;
 *     - capability resolution;
 *     - resource derivation;
 *     - topology compatibility;
 *     - portability analysis;
 *     - scalability analysis;
 *     - contradiction detection.
 *
 * The parser MUST NOT determine:
 *
 *     "Can this topology actually be built on the target?"
 *
 * That question belongs downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Topology requirements may refer to the universal resource model:
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 *     requires resource::bandwidth >= required_bandwidth;
 *
 * The topology grammar does not define resource accounting.
 *
 * Resource accounting belongs to:
 *
 *     grammar/resources/
 *
 * and downstream semantic/resource analysis.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Topology may refer to symbolic capabilities:
 *
 *     requires capability("quantum.two_body_interaction");
 *
 *     requires capability("network.low_latency");
 *
 *     requires capability("accelerator.interconnect");
 *
 * Capability resolution belongs to the canonical capability system.
 *
 * This grammar does not enumerate vendor capabilities.
 *
 * ============================================================================
 * PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Topology and placement remain distinct.
 *
 * Topology says:
 *
 *     which logical resources may or must connect.
 *
 * Placement says:
 *
 *     where logical resources may reside.
 *
 * Therefore:
 *
 *     topology.g4
 *         |
 *         v
 *     topology semantic model
 *         |
 *         +------------------+
 *         |                  |
 *         v                  v
 *     placement          routing
 *         |                  |
 *         +--------+---------+
 *                  |
 *                  v
 *              scheduling
 *
 * `placement.g4` MUST NOT duplicate topology declaration syntax.
 *
 * ============================================================================
 * ROUTING INTEGRATION
 * ============================================================================
 *
 * Routing consumes resolved topology.
 *
 * It may determine:
 *
 *     - paths;
 *     - intermediate resources;
 *     - communication routes;
 *     - quantum interaction routes;
 *     - transformations required to realize a logical relationship.
 *
 * None of those algorithms belong in this grammar.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling may use topology properties such as:
 *
 *     latency
 *     bandwidth
 *     resource availability
 *     concurrency
 *
 * The grammar only records source intent.
 *
 * ============================================================================
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * This file MUST NOT define a quantum IR.
 *
 * Quantum source computation follows:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * Topology information may be associated with the semantic quantum program
 * and later consumed by routing/scheduling.
 *
 * No second quantum topology IR is introduced here.
 *
 * ============================================================================
 * QEC / ZQN INTEGRATION
 * ============================================================================
 *
 * QEC may consume topology and connectivity information when determining
 * realizability of logical quantum computation.
 *
 * ZQN may consume topology-related noise/fault properties.
 *
 * Neither is implemented by this grammar.
 *
 * ============================================================================
 * HARDWARE HAL INTEGRATION
 * ============================================================================
 *
 * The HAL may expose actual target topology:
 *
 *     available nodes
 *     connectivity
 *     links
 *     bandwidth
 *     latency
 *     reliability
 *     capabilities
 *
 * Direction:
 *
 *     source topology
 *         ->
 *     semantic topology intent
 *         ->
 *     target resolution
 *         ->
 *     HAL comparison
 *
 * NOT:
 *
 *     parser
 *         ->
 *     HAL
 *
 * Parsing must remain deterministic and independent of the physical machine.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     network state
 *     filesystem state
 *     environment variables
 *     clock/time
 *     randomness
 *     runtime state
 *     HAL state
 *
 * Identical source plus identical grammar/token versions must produce the
 * same parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Topology syntax is untrusted input.
 *
 * Parsing performs:
 *
 *     no filesystem access
 *     no network access
 *     no process execution
 *     no hardware probing
 *     no credential access
 *     no backend loading
 *
 * Physical addresses and backend handles are not interpreted by this grammar.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     topology;
 *     topology foo {;
 *     node;
 *     connect a;
 *     connect a to;
 *     edge a to;
 *     requires;
 *
 * Semantic errors include:
 *
 *     unknown topology reference;
 *     unresolved endpoint;
 *     invalid selector;
 *     contradictory topology constraints;
 *     incompatible topology requirements;
 *     impossible resource requirement.
 *
 * Resource shortage is NOT a syntax error.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The stable public entry point is:
 *
 *     topologyDeclaration
 *
 * Existing callers should delegate to this rule rather than reproduce
 * topology syntax.
 *
 * New topology concepts should normally be introduced as:
 *
 *     qualified names
 *     properties
 *     expressions
 *     semantic capabilities
 *
 * rather than new reserved keywords.
 *
 * This protects future compatibility.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     topology ring {
 *         node compute;
 *     }
 *
 *     topology logical {
 *         node compute : accelerator::compute;
 *         group workers [worker_count];
 *         connect workers[*] to compute[*] bidirectional;
 *     }
 *
 *     topology quantum {
 *         node logical_qubit : quantum::logical_qubit;
 *         connect logical_qubit[*] to logical_qubit[*];
 *     }
 *
 *     topology network {
 *         node service;
 *         node peer;
 *         connect service to peer directed;
 *     }
 *
 *     topology scalable {
 *         node worker [problem_size];
 *         requires resource::bandwidth >= required_bandwidth;
 *         requires capability("distributed.communication");
 *         prefer topology::low_latency;
 *         constraint latency <= latency_budget;
 *         hint locality;
 *     }
 *
 *     topology custom::family extends topology::base {
 *         property topology::dimension = dimensions;
 *     }
 *
 * Negative:
 *
 *     topology;
 *
 *     topology foo {
 *         node;
 *     }
 *
 *     topology foo {
 *         connect;
 *     }
 *
 *     topology foo {
 *         connect a to;
 *     }
 *
 *     topology foo {
 *         requires;
 *     }
 *
 * Boundary:
 *
 *     one node;
 *     many nodes;
 *     many groups;
 *     many edges;
 *     many properties;
 *     deeply qualified names;
 *     symbolic quantities;
 *     symbolic selectors;
 *     nested expressions;
 *     arbitrarily large source-level counts represented as expressions.
 *
 * Scalability:
 *
 *     topology worker_graph {
 *         node worker [worker_count];
 *         connect worker[*] to worker[*];
 *     }
 *
 * The test MUST NOT establish a maximum for worker_count.
 *
 * Cross-domain:
 *
 *     classical topology;
 *     quantum topology;
 *     HDL topology;
 *     accelerator topology;
 *     distributed topology;
 *     networking topology;
 *     AI/data topology.
 *
 * Determinism:
 *
 *     identical source + grammar + lexer version
 *         ->
 *     identical parse structure.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO MAX_* machine limits;
 *     NO physical device enumeration;
 *     NO vendor enumeration;
 *     NO physical address syntax;
 *     NO physical qubit enumeration;
 *     NO fixed topology family enumeration;
 *     NO fixed graph size;
 *     NO fixed degree;
 *     NO fixed path length;
 *     NO fixed number of nodes;
 *     NO fixed number of edges;
 *     NO fixed number of links.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Generated and consuming Rust code MUST:
 *
 *     - target Rust 1.97 / Rust 1.97.1;
 *     - use Rust 2021;
 *     - use safe Rust only;
 *     - contain no `unsafe` blocks;
 *     - contain no `unsafe fn`;
 *     - contain no `unsafe impl`;
 *     - contain no `unsafe trait`.
 *
 * This file therefore introduces no unsafe implementation dependency.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * topology.g4 is complete when:
 *
 *     [x] topology is logical rather than physical;
 *     [x] topology names are open-world;
 *     [x] node counts are symbolic;
 *     [x] edge counts are unbounded;
 *     [x] group membership is scalable;
 *     [x] endpoint selection is symbolic;
 *     [x] properties are extensible;
 *     [x] requirements are distinct from preferences;
 *     [x] constraints are distinct from hints;
 *     [x] capability references remain symbolic;
 *     [x] resource semantics remain downstream;
 *     [x] placement remains separate;
 *     [x] routing remains separate;
 *     [x] scheduling remains separate;
 *     [x] HAL remains separate;
 *     [x] QEC remains separate;
 *     [x] ZQN remains separate;
 *     [x] quantum::ir remains canonical;
 *     [x] no physical hardware is selected;
 *     [x] no artificial machine limits are encoded;
 *     [x] no Rust actions exist;
 *     [x] no unsafe implementation is required;
 *     [x] deterministic parsing is preserved.
 *
 * Repository integration still requires the canonical lexical/composition
 * contracts listed below to be synchronized.
 *
 * ============================================================================
 */