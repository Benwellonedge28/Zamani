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
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded target-language actions.
 *     No semantic predicates.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for machine-independent hardware
 * topology descriptions and topology intent.
 *
 * A topology describes relationships among logical computational resources.
 *
 * It may express:
 *
 *     - logical topology declarations;
 *     - node classes;
 *     - endpoint classes;
 *     - links;
 *     - connectivity;
 *     - directionality;
 *     - weights;
 *     - capacities;
 *     - latency;
 *     - bandwidth;
 *     - cost;
 *     - reliability;
 *     - topology constraints;
 *     - topology requirements;
 *     - topology preferences;
 *     - topology annotations;
 *     - topology composition;
 *     - topology properties;
 *     - topology-independent placement relationships;
 *     - topology families;
 *     - topology predicates;
 *     - symbolic topology dimensions;
 *     - implementation-independent connectivity requirements.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - numeric literal recognition;
 *     - physical hardware discovery;
 *     - physical device enumeration;
 *     - device serial numbers;
 *     - physical addresses;
 *     - vendor-specific topology formats;
 *     - runtime topology discovery;
 *     - calibration;
 *     - routing algorithms;
 *     - scheduling;
 *     - placement algorithms;
 *     - resource allocation;
 *     - optimization;
 *     - device drivers;
 *     - backend execution;
 *     - canonical AST implementation;
 *     - semantic type checking;
 *     - canonical quantum IR;
 *     - QEC;
 *     - ZQN/noise semantics.
 *
 * In particular, this file MUST NOT become a second representation of the
 * physical topology discovered by the Hardware HAL.
 *
 * ============================================================================
 * TOPOLOGY VS PHYSICAL MACHINE
 * ============================================================================
 *
 * A topology is a semantic relationship model.
 *
 * For example:
 *
 *     topology ring {
 *         connect compute[*] to compute[*]
 *             where adjacent;
 *     }
 *
 * expresses a topology relationship.
 *
 * It does NOT mean:
 *
 *     - use device 7;
 *     - use physical address 0x1234;
 *     - use exactly 16 nodes;
 *     - use exactly 32 qubits;
 *     - use a particular vendor;
 *     - use a particular processor.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Topology syntax must describe:
 *
 *     WHAT relationships are required or permitted.
 *
 * It must not unnecessarily describe:
 *
 *     WHICH physical machine realizes those relationships.
 *
 * A topology may therefore be resolved differently on:
 *
 *     - a tiny embedded target;
 *     - one CPU;
 *     - many CPUs;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum processor;
 *     - a quantum simulator;
 *     - a heterogeneous accelerator;
 *     - a cluster;
 *     - a supercomputer;
 *     - a distributed system;
 *     - a future architecture.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits for:
 *
 *     - topology nodes;
 *     - node classes;
 *     - links;
 *     - edges;
 *     - ports;
 *     - dimensions;
 *     - topology components;
 *     - connected resources;
 *     - graph size;
 *     - degree;
 *     - path length;
 *     - capacity;
 *     - bandwidth;
 *     - latency;
 *     - distance;
 *     - topology depth.
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * rather than fixed-size.
 *
 * Quantities are expressions.
 *
 * The semantic/resource/target layers determine whether a requested topology
 * can be realized by the selected execution environment.
 *
 * ============================================================================
 * OPEN-WORLD TOPOLOGY MODEL
 * ============================================================================
 *
 * The grammar deliberately does NOT hard-code topology names such as:
 *
 *     ring
 *     mesh
 *     torus
 *     grid
 *     star
 *     tree
 *     hypercube
 *     heavy_hex
 *
 * Those may be represented as qualified semantic names:
 *
 *     topology.quantum.heavy_hex
 *     topology.network.mesh
 *     topology.custom.application
 *
 * or through user-defined topology declarations.
 *
 * This keeps the grammar extensible as new architectures are introduced.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum topology is represented as hardware/resource topology intent.
 *
 * Examples may describe:
 *
 *     quantum.connectivity
 *     quantum.neighbor
 *     quantum.interaction
 *     quantum.routing
 *
 * but this grammar does NOT define:
 *
 *     - quantum gates;
 *     - quantum operations;
 *     - quantum states;
 *     - quantum circuits;
 *     - QEC algorithms;
 *     - noise models;
 *     - ZQN;
 *     - quantum IR.
 *
 * Quantum source semantics ultimately lower through:
 *
 *     quantum::ir
 *
 * before routing/scheduling/backend realization.
 *
 * ============================================================================
 * ARCHITECTURAL FLOW
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     lexer/tokens.g4
 *       |
 *       v
 *     topology.g4
 *       |
 *       v
 *     syntax AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *     topology model      resource model
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *        target/capability resolution
 *                 |
 *                 v
 *             placement
 *                 |
 *                 v
 *              routing
 *                 |
 *                 v
 *             scheduling
 *                 |
 *                 v
 *            Hardware HAL
 *                 |
 *                 v
 *              runtime
 *
 * Grammar never directly depends on runtime discovery.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Required lexical vocabulary belongs in:
 *
 *     grammar/lexer/tokens.g4
 *
 * Required topology tokens:
 *
 *     K_TOPOLOGY
 *     K_NODE
 *     K_EDGE
 *     K_LINK
 *     K_CONNECT
 *     K_FROM
 *     K_TO
 *     K_BIDIRECTIONAL
 *     K_DIRECTED
 *     K_UNDIRECTED
 *     K_WEIGHT
 *     K_CAPACITY
 *     K_LATENCY
 *     K_BANDWIDTH
 *     K_DISTANCE
 *     K_COST
 *     K_RELIABILITY
 *     K_NEIGHBOR
 *     K_ADJACENT
 *     K_WHERE
 *     K_REQUIRES
 *     K_PREFER
 *     K_CONSTRAINT
 *     K_PROPERTY
 *     K_GROUP
 *     K_EXTENDS
 *
 * If any of these are not yet present in tokens.g4, they MUST be added to
 * the shared lexical vocabulary. They must NOT be implemented as private
 * lexer rules here.
 *
 * Existing generic tokens required:
 *
 *     IDENTIFIER
 *     STRING_LITERAL
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT
 *     DOUBLE_COLON
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     LESS
 *     GREATER
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *
 * The exact shared token names remain controlled by tokens.g4.
 *
 * ============================================================================
 * ROOT-PARSER INTEGRATION
 * ============================================================================
 *
 * The root/hardware parser must expose:
 *
 *     topologyDeclaration
 *
 * as a hardware declaration alternative.
 *
 * A topology declaration may also be accepted as a member of:
 *
 *     hardwareDeclaration
 *     deviceDeclaration
 *     targetDeclaration
 *
 * where those parent grammars explicitly permit topology intent.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax information only.
 *
 * The AST layer should provide semantic nodes equivalent to:
 *
 *     HardwareTopologyDecl
 *     TopologyNodeDecl
 *     TopologyEdgeDecl
 *     TopologyGroupDecl
 *     TopologyProperty
 *     TopologyConstraint
 *     TopologyRequirement
 *     TopologyPreference
 *     TopologyPredicate
 *     TopologyEndpoint
 *     TopologyMetric
 *
 * Each node should retain:
 *
 *     - source span;
 *     - source order where semantically relevant;
 *     - qualified names;
 *     - expressions;
 *     - annotations;
 *     - modifiers.
 *
 * The grammar itself does not construct those AST objects.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving topology names;
 *     - resolving node classes;
 *     - resolving endpoint references;
 *     - validating graph relationships;
 *     - validating metric types;
 *     - validating units;
 *     - detecting impossible constraints;
 *     - distinguishing requirement/constraint/preference;
 *     - checking topology compatibility;
 *     - determining whether topology is static, symbolic, dynamic, or
 *       target-dependent;
 *     - producing diagnostics.
 *
 * The grammar does not decide whether a topology is realizable.
 *
 * ============================================================================
 * HARDWARE HAL CONTRACT
 * ============================================================================
 *
 * The Hardware HAL may provide runtime topology information.
 *
 * The direction is:
 *
 *     grammar -> AST -> semantic model -> compiler/runtime context
 *
 * NOT:
 *
 *     grammar -> Hardware HAL
 *
 * The HAL may report:
 *
 *     - available nodes;
 *     - actual connectivity;
 *     - supported links;
 *     - latency;
 *     - bandwidth;
 *     - reliability;
 *     - capabilities.
 *
 * The source grammar does not enumerate those physical resources.
 *
 * ============================================================================
 * ROUTING / PLACEMENT / SCHEDULING
 * ============================================================================
 *
 * Topology describes connectivity requirements.
 *
 * Placement maps logical resources to realizable resources.
 *
 * Routing realizes communication/interaction paths.
 *
 * Scheduling determines execution order and timing.
 *
 * Therefore:
 *
 *     topology.g4
 *         != placement
 *         != routing
 *         != scheduling
 *
 * This separation is mandatory.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareTopologyParser;

options {
    tokenVocab = ZamaniTokens;
}


// ============================================================================
// 1. PUBLIC ENTRY POINT
// ============================================================================

topologyDeclaration
    : topologyAnnotation*
      topologyVisibility?
      topologyModifier*
      K_TOPOLOGY
      topologyName
      topologyExtendsClause?
      topologyBody
    ;


// ============================================================================
// 2. VISIBILITY / MODIFIERS
// ============================================================================

topologyVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;

topologyModifier
    : K_STATIC
    | K_CONST
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_PARTIAL
    ;


// ============================================================================
// 3. ANNOTATIONS
// ============================================================================

topologyAnnotation
    : AT topologyQualifiedName
      (LPAREN topologyArgumentList? RPAREN)?
    ;

topologyArgumentList
    : topologyArgument
      (COMMA topologyArgument)*
    ;

topologyArgument
    : topologyExpression
    | topologyQualifiedName
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    ;


// ============================================================================
// 4. TOPOLOGY NAMES
// ============================================================================

topologyName
    : topologyQualifiedName
    ;

topologyQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON IDENTIFIER
        | DOT IDENTIFIER
      )*
    ;


// ============================================================================
// 5. TOPOLOGY INHERITANCE / EXTENSION
// ============================================================================
//
// Extension is semantic composition, not physical inheritance.
//
// A topology may extend a logical topology definition while remaining
// implementation-independent.
//

topologyExtendsClause
    : K_EXTENDS topologyQualifiedName
      (COMMA topologyQualifiedName)*
    ;


// ============================================================================
// 6. TOPOLOGY BODY
// ============================================================================

topologyBody
    : LBRACE topologyMember* RBRACE
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
    | topologyAnnotation* topologyPredicateDeclaration
    | topologyAnnotation* topologyUsingDeclaration
    ;


// ============================================================================
// 7. NODE DECLARATIONS
// ============================================================================
//
// A node is a LOGICAL topology participant.
//
// It is not a physical device ID.
//
// Examples:
//
//     node compute;
//     node quantum.logical;
//     node accelerator;
//

topologyNodeDeclaration
    : K_NODE
      topologyNodeName
      topologyNodeTypeClause?
      topologyNodeMultiplicityClause?
      topologyNodeAttributeBlock?
      SEMICOLON
    ;

topologyNodeName
    : topologyQualifiedName
    ;

topologyNodeTypeClause
    : COLON topologyQualifiedName
    ;

topologyNodeMultiplicityClause
    : LBRACKET topologyExpression RBRACKET
    ;


// ============================================================================
// 8. NODE GROUPS
// ============================================================================
//
// Groups allow scalable topology descriptions without enumerating members.
//
// The number of members is a semantic/resource concern.
//
// No fixed number is imposed.
//

topologyGroupDeclaration
    : K_GROUP
      topologyQualifiedName
      topologyGroupTypeClause?
      topologyGroupSelectorClause?
      topologyGroupMultiplicityClause?
      topologyGroupBody?
      SEMICOLON?
    ;

topologyGroupTypeClause
    : COLON topologyQualifiedName
    ;

topologyGroupSelectorClause
    : K_WHERE
      topologyPredicateExpression
    ;

topologyGroupMultiplicityClause
    : LBRACKET topologyExpression RBRACKET
    ;

topologyGroupBody
    : LBRACE
      topologyGroupMember*
      RBRACE
    ;

topologyGroupMember
    : topologyQualifiedName
      (COMMA topologyQualifiedName)*
      SEMICOLON
    ;


// ============================================================================
// 9. EDGE DECLARATIONS
// ============================================================================
//
// Edges describe logical relationships between topology endpoints.
//
// They do not imply a physical cable, wire, optical path, network interface,
// quantum coupling channel, or particular machine implementation unless the
// semantic layer explicitly establishes that meaning.
//

topologyEdgeDeclaration
    : K_EDGE
      topologyEdgeName?
      topologyEdgeEndpointClause
      topologyEdgeDirectionClause?
      topologyEdgeAttributeBlock?
      SEMICOLON
    ;

topologyEdgeName
    : topologyQualifiedName
    ;

topologyEdgeEndpointClause
    : topologyEndpoint
      K_TO
      topologyEndpoint
    ;

topologyEdgeDirectionClause
    : K_BIDIRECTIONAL
    | K_DIRECTED
    | K_UNDIRECTED
    ;


// ============================================================================
// 10. LINK DECLARATIONS
// ============================================================================
//
// "link" is an optional semantic vocabulary for communication/connectivity
// relationships. It remains abstract.
//

topologyLinkDeclaration
    : K_LINK
      topologyLinkName?
      topologyEndpoint
      K_TO
      topologyEndpoint
      topologyEdgeDirectionClause?
      topologyLinkAttributeBlock?
      SEMICOLON
    ;

topologyLinkName
    : topologyQualifiedName
    ;


// ============================================================================
// 11. CONNECTION DECLARATIONS
// ============================================================================
//
// Connections are the most direct source-level topology intent.
//
// Examples:
//
//     connect q to r;
//     connect logical.qubit[*] to logical.qubit[*];
//     connect a to b bidirectional;
//

topologyConnectionDeclaration
    : K_CONNECT
      topologyEndpoint
      K_TO
      topologyEndpoint
      topologyEdgeDirectionClause?
      topologyConnectionConditionClause?
      topologyConnectionAttributeBlock?
      SEMICOLON
    ;


// ============================================================================
// 12. ENDPOINTS
// ============================================================================
//
// Endpoints may contain symbolic selectors.
//
// No physical address syntax is defined here.
//

topologyEndpoint
    : topologyEndpointBase
      topologyEndpointSelector*
    ;

topologyEndpointBase
    : topologyQualifiedName
    ;

topologyEndpointSelector
    : LBRACKET
      topologySelectorExpression
      RBRACKET
    ;


// ============================================================================
// 13. SELECTORS
// ============================================================================
//
// Selectors allow a topology to describe classes or symbolic sets without
// enumerating physical resources.
//

topologySelectorExpression
    : topologyExpression
    | topologyPredicateExpression
    ;


// ============================================================================
// 14. CONNECTION CONDITIONS
// ============================================================================

topologyConnectionConditionClause
    : K_WHERE
      topologyPredicateExpression
    ;


// ============================================================================
// 15. NODE ATTRIBUTES
// ============================================================================

topologyNodeAttributeBlock
    : LBRACE
      topologyAttribute*
      RBRACE
    ;

topologyAttribute
    : topologyQualifiedName
      (ASSIGN topologyExpression)?
      SEMICOLON
    ;


// ============================================================================
// 16. EDGE ATTRIBUTES
// ============================================================================

topologyEdgeAttributeBlock
    : LBRACE
      topologyEdgeAttribute*
      RBRACE
    ;

topologyEdgeAttribute
    : topologyMetricAttribute
    | topologyAttribute
    ;


// ============================================================================
// 17. LINK ATTRIBUTES
// ============================================================================

topologyLinkAttributeBlock
    : LBRACE
      topologyEdgeAttribute*
      RBRACE
    ;


// ============================================================================
// 18. CONNECTION ATTRIBUTES
// ============================================================================

topologyConnectionAttributeBlock
    : LBRACE
      topologyEdgeAttribute*
      RBRACE
    ;


// ============================================================================
// 19. METRICS
// ============================================================================
//
// Metrics remain generic and symbolic.
//
// The grammar does not assume:
//     - nanoseconds;
//     - GHz;
//     - GB/s;
//     - meters;
//     - percentage ranges;
//     - any particular unit system.
//
// Unit semantics belong to the shared expression/resource/type system.
//

topologyMetricAttribute
    : K_WEIGHT
      ASSIGN
      topologyExpression
      SEMICOLON

    | K_CAPACITY
      ASSIGN
      topologyExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      topologyExpression
      SEMICOLON

    | K_BANDWIDTH
      ASSIGN
      topologyExpression
      SEMICOLON

    | K_DISTANCE
      ASSIGN
      topologyExpression
      SEMICOLON

    | K_COST
      ASSIGN
      topologyExpression
      SEMICOLON

    | K_RELIABILITY
      ASSIGN
      topologyExpression
      SEMICOLON
    ;


// ============================================================================
// 20. PROPERTY DECLARATIONS
// ============================================================================
//
// Properties are open-world.
//
// Do not hard-code every possible future topology property.
//

topologyPropertyDeclaration
    : K_PROPERTY
      topologyQualifiedName
      (COLON topologyQualifiedName)?
      (ASSIGN topologyExpression)?
      SEMICOLON
    ;


// ============================================================================
// 21. REQUIREMENTS
// ============================================================================
//
// Requirements describe semantic minimums or mandatory relationships.
//
// They are stronger than preferences.
//

topologyRequirementDeclaration
    : K_REQUIRES
      topologyRequirementExpression
      SEMICOLON
    ;

topologyRequirementExpression
    : topologyConnectivityRequirement
    | topologyMetricRequirement
    | topologyPredicateExpression
    | topologyQualifiedName
    ;

topologyConnectivityRequirement
    : K_CONNECT
      topologyEndpoint
      K_TO
      topologyEndpoint
      topologyEdgeDirectionClause?
    ;

topologyMetricRequirement
    : topologyMetricName
      topologyComparisonOperator
      topologyExpression
    ;

topologyMetricName
    : K_WEIGHT
    | K_CAPACITY
    | K_LATENCY
    | K_BANDWIDTH
    | K_DISTANCE
    | K_COST
    | K_RELIABILITY
    | topologyQualifiedName
    ;


// ============================================================================
// 22. CONSTRAINTS
// ============================================================================
//
// Constraints define conditions that a valid realization must satisfy.
//
// They do not select a particular physical implementation.
//

topologyConstraintDeclaration
    : K_CONSTRAINT
      topologyConstraintExpression
      SEMICOLON
    ;

topologyConstraintExpression
    : topologyComparisonExpression
    | topologyLogicalExpression
    | topologyConnectivityConstraint
    | topologyQualifiedName
    ;

topologyConnectivityConstraint
    : topologyEndpoint
      topologyConnectivityOperator
      topologyEndpoint
    ;

topologyConnectivityOperator
    : K_CONNECTS
    | K_ADJACENT
    | K_NEIGHBOR
    ;


// ============================================================================
// 23. PREFERENCES
// ============================================================================
//
// Preferences are optimization hints.
//
// A compiler may choose another realization when necessary.
//

topologyPreferenceDeclaration
    : K_PREFER
      topologyPreferenceExpression
      SEMICOLON
    ;

topologyPreferenceExpression
    : topologyQualifiedName
    | topologyComparisonExpression
    | topologyConnectivityPreference
    ;

topologyConnectivityPreference
    : topologyEndpoint
      topologyConnectivityOperator
      topologyEndpoint
    ;


// ============================================================================
// 24. PREDICATES
// ============================================================================
//
// Predicates allow topology conditions to remain symbolic.
//
// The semantic layer determines whether a predicate is statically decidable,
// target-dependent, or runtime-dependent.
//

topologyPredicateDeclaration
    : K_WHERE
      topologyPredicateExpression
      SEMICOLON
    ;

topologyPredicateExpression
    : topologyLogicalExpression
    | topologyComparisonExpression
    | topologyConnectivityPredicate
    | topologyQualifiedName
    | LPAREN topologyPredicateExpression RPAREN
    ;

topologyConnectivityPredicate
    : topologyEndpoint
      topologyConnectivityOperator
      topologyEndpoint
    ;


// ============================================================================
// 25. COMPARISON EXPRESSIONS
// ============================================================================

topologyComparisonExpression
    : topologyExpression
      topologyComparisonOperator
      topologyExpression
    ;

topologyComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


// ============================================================================
// 26. LOGICAL EXPRESSIONS
// ============================================================================

topologyLogicalExpression
    : topologyLogicalExpression
      K_OR
      topologyLogicalTerm
    | topologyLogicalTerm
    ;

topologyLogicalTerm
    : topologyLogicalTerm
      K_AND
      topologyLogicalFactor
    | topologyLogicalFactor
    ;

topologyLogicalFactor
    : K_NOT topologyLogicalFactor
    | LPAREN topologyLogicalExpression RPAREN
    | topologyComparisonExpression
    | topologyConnectivityPredicate
    | topologyQualifiedName
    ;


// ============================================================================
// 27. EXPRESSIONS
// ============================================================================
//
// This is intentionally a topology-local syntactic expression boundary.
//
// When the universal expression grammar is available through an imported
// parser grammar, this rule should be replaced by the shared expression
// contract rather than creating a second semantic expression system.
//
// Until parser composition is established repository-wide, the local boundary
// permits symbolic topology quantities without introducing machine limits.
//

topologyExpression
    : topologyAdditiveExpression
    ;

topologyAdditiveExpression
    : topologyMultiplicativeExpression
      (
          PLUS topologyMultiplicativeExpression
        | MINUS topologyMultiplicativeExpression
      )*
    ;

topologyMultiplicativeExpression
    : topologyUnaryExpression
      (
          STAR topologyUnaryExpression
        | SLASH topologyUnaryExpression
      )*
    ;

topologyUnaryExpression
    : PLUS topologyUnaryExpression
    | MINUS topologyUnaryExpression
    | topologyPrimaryExpression
    ;

topologyPrimaryExpression
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | topologyQualifiedName
    | LPAREN topologyExpression RPAREN
    ;


// ============================================================================
// 28. USING / IMPORT-STYLE TOPOLOGY NAMES
// ============================================================================
//
// This rule only records source-level name usage.
//
// Package/module dependency semantics belong to the module system.
//

topologyUsingDeclaration
    : K_USING
      topologyQualifiedName
      (K_AS IDENTIFIER)?
      SEMICOLON
    ;


// ============================================================================
// 29. GENERIC TOPOLOGY QUANTITIES
// ============================================================================
//
// Symbolic dimensions remain legal.
//
// Examples:
//
//     [workers]
//     [available_nodes]
//     [required_degree]
//
// No parser-level integer maximum exists.
//

topologyMultiplicity
    : LBRACKET topologyExpression RBRACKET
    ;


// ============================================================================
// 30. OPTIONAL TOPOLOGY FAMILIES
// ============================================================================
//
// Topology families are represented by names rather than a closed enumeration.
//
// Examples:
//
//     topology.graph
//     topology.mesh
//     topology.network
//     topology.quantum
//     topology.application
//
// No built-in list is imposed.
//

topologyFamilyReference
    : topologyQualifiedName
    ;


// ============================================================================
// 31. TOPOLOGY SCHEMA REFERENCE
// ============================================================================
//
// Allows a declaration to reference a separately defined topology schema.
//

topologySchemaReference
    : K_TYPE
      topologyQualifiedName
    ;


// ============================================================================
// 32. TOPOLOGY RELATIONSHIP REFERENCE
// ============================================================================

topologyRelationshipReference
    : topologyEndpoint
      topologyConnectivityOperator
      topologyEndpoint
    ;


// ============================================================================
// 33. TOPOLOGY CONSTRAINT SET
// ============================================================================
//
// A constraint set groups constraints without introducing a fixed number of
// constraints.
//

topologyConstraintSet
    : LBRACE
      topologyConstraintDeclaration*
      RBRACE
    ;


// ============================================================================
// 34. TOPOLOGY REQUIREMENT SET
// ============================================================================

topologyRequirementSet
    : LBRACE
      topologyRequirementDeclaration*
      RBRACE
    ;


// ============================================================================
// 35. TOPOLOGY PREFERENCE SET
// ============================================================================

topologyPreferenceSet
    : LBRACE
      topologyPreferenceDeclaration*
      RBRACE
    ;


// ============================================================================
// 36. TOPOLOGY VALIDATION BOUNDARY
// ============================================================================
//
// Syntax alone cannot determine:
//
//     - whether every endpoint exists;
//     - whether a graph is connected;
//     - whether cycles are allowed;
//     - whether an edge is realizable;
//     - whether bandwidth is sufficient;
//     - whether latency is achievable;
//     - whether topology matches hardware;
//     - whether a quantum interaction is supported.
//
// Those are semantic/target/resource validations.
//
// This grammar therefore intentionally stops at syntactic validity.
// ============================================================================


// ============================================================================
// 37. NON-OWNERSHIP OF PHYSICAL TOPOLOGY
// ============================================================================
//
// The following MUST NOT appear as required topology syntax:
//
//     physical_device_id
//     serial_number
//     pci_address
//     mac_address
//     hardware_address
//     vendor_id
//     fixed_qubit_index
//     fixed_core_index
//     fixed_gpu_index
//
// Such values may be represented by other target/deployment languages when
// they genuinely belong to deployment semantics, but they are not required
// by this topology grammar.
//
// ============================================================================


// ============================================================================
// 38. SCALABILITY INVARIANTS
// ============================================================================
//
// These invariants are architectural requirements:
//
//     topologyNodeDeclaration*
//     topologyGroupDeclaration*
//     topologyEdgeDeclaration*
//     topologyLinkDeclaration*
//     topologyConnectionDeclaration*
//
// MUST remain unbounded by source grammar.
//
// A topology may contain any number of logical participants.
//
// Multiplicity is always represented symbolically where applicable.
//
// Examples:
//
//     node compute[workers];
//
//     connect compute[*] to storage[*];
//
//     connect logical.qubit[*] to logical.qubit[*];
//
// The semantic/resource layer determines the actual realizable cardinality.
//
// ============================================================================


// ============================================================================
// 39. DETERMINISM
// ============================================================================
//
// Parsing is deterministic:
//
//     source
//       -> lexer
//       -> parser
//
// produces the same syntax tree for the same source and grammar version.
//
// There are:
//
//     - no runtime callbacks;
//     - no hardware discovery;
//     - no network access;
//     - no random decisions;
//     - no calibration queries;
//     - no backend-dependent parser branches.
//
// ============================================================================


// ============================================================================
// 40. VERSIONING
// ============================================================================
//
// Changes to:
//
//     - topology keywords;
//     - topology syntax;
//     - operator precedence;
//     - declaration forms;
//     - endpoint syntax;
//
// are language compatibility changes.
//
// Semantic topology vocabulary should preferably be extended through qualified
// names rather than repeatedly adding reserved keywords.
//
// ============================================================================


// ============================================================================
// 41. SECURITY
// ============================================================================
//
// Hardware topology capability is NOT an authorization capability.
//
// This grammar does not grant:
//
//     - access to hardware;
//     - permission to execute;
//     - permission to allocate resources;
//     - permission to connect to a network;
//     - permission to access another user's device.
//
// Security/authorization semantics belong to grammar/security and the
// corresponding semantic/runtime security layers.
//
// ============================================================================


// ============================================================================
// 42. COMPLETION CRITERIA
// ============================================================================
//
// This file is complete only when:
//
//     [ ] tokens.g4 provides every required topology keyword/operator token;
//
//     [ ] the root parser exposes topologyDeclaration;
//
//     [ ] AST has topology declaration/endpoint/edge/node/group models;
//
//     [ ] semantic analysis resolves topology names and expressions;
//
//     [ ] resource semantics validate topology metrics;
//
//     [ ] target selection consumes topology requirements;
//
//     [ ] placement consumes topology constraints;
//
//     [ ] routing consumes topology relationships;
//
//     [ ] scheduling remains independent of topology syntax;
//
//     [ ] Hardware HAL remains the source of discovered physical topology;
//
//     [ ] no physical device enumeration exists in this grammar;
//
//     [ ] no machine-size limit exists;
//
//     [ ] no MAX_* topology constant exists;
//
//     [ ] quantum topology lowers through the canonical quantum::ir boundary
//         where quantum computation is involved;
//
//     [ ] positive, negative, boundary, scalability, determinism, and
//         cross-domain tests exist;
//
//     [ ] parser generation succeeds for Rust 1.97 / Rust 1.97.1;
//
//     [ ] generated Rust contains no prohibited unsafe usage;
//
//     [ ] documentation agrees with the grammar.
//
// ============================================================================