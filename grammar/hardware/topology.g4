/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* File:
* grammar/hardware/topology.g4
* 
* Grammar kind:
* ANTLR4 parser grammar
* 
* Grammar name:
* ZamaniHardwareTopologyParser
* 
* Target:
* Rust 1.97 / Rust 1.97.1
* 
* Safety:
* Action-free grammar.
* No embedded Rust.
* No semantic predicates.
* No target-language actions.
* No unsafe Rust.
* 
* ============================================================================
* STATUS
* ============================================================================
* 
* PRODUCTION TOPOLOGY CONTRACT
* 
* This file defines the source-level syntax for abstract hardware topology
* intent.
* 
* The grammar describes relationships among logical computational resources.
* 
* It does NOT describe a particular physical machine.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - topology declarations;
* - logical topology nodes;
* - logical topology groups;
* - abstract endpoints;
* - logical connectivity relationships;
* - topology relationship direction;
* - topology relationship attributes;
* - topology predicates;
* - topology requirements;
* - topology constraints;
* - topology preferences;
* - topology properties;
* - topology composition/extension;
* - symbolic topology cardinality;
* - topology-level semantic metadata.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical token definitions;
* - identifiers;
* - general expressions;
* - general types;
* - resources;
* - capabilities;
* - hardware devices;
* - targets;
* - physical device discovery;
* - physical device identifiers;
* - physical addresses;
* - placement algorithms;
* - routing algorithms;
* - scheduling algorithms;
* - optimization;
* - calibration;
* - runtime execution;
* - device drivers;
* - QEC;
* - ZQN;
* - quantum operations;
* - quantum states;
* - quantum circuits;
* - canonical quantum IR.
* 
* ============================================================================
* ARCHITECTURAL PRINCIPLE
* ============================================================================
* 
* Topology is a declarative relationship contract.
* 
* It describes:
* 
* WHAT relationships a computation requires or prefers.
* 
* It does not prescribe:
* 
* WHICH physical device realizes those relationships.
* 
* Therefore topology syntax MUST remain valid across:
* 
* - tiny embedded systems;
* - single CPUs;
* - multicore CPUs;
* - GPUs;
* - FPGAs;
* - ASICs;
* - accelerators;
* - quantum processors;
* - quantum simulators;
* - heterogeneous systems;
* - clusters;
* - distributed systems;
* - HPC systems;
* - future architectures.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* The topology grammar MUST NOT establish language-level limits for:
* 
* - nodes;
* - edges;
* - links;
* - groups;
* - ports;
* - degree;
* - graph size;
* - topology depth;
* - path length;
* - connected resources;
* - CPUs;
* - cores;
* - threads;
* - GPUs;
* - FPGAs;
* - ASICs;
* - accelerators;
* - QPUs;
* - qubits;
* - memory;
* - devices;
* - network nodes;
* - timelines.
* 
* No constants such as:
* 
* MAX_NODES
* MAX_EDGES
* MAX_PORTS
* MAX_DEVICES
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_MEMORY
* 
* may be introduced by this grammar.
* 
* A finite repetition operator in a grammar is not a hardware limit.
* 
* ============================================================================
* REQUIREMENT / CONSTRAINT / PREFERENCE
* ============================================================================
* 
* These concepts are deliberately distinct.
* 
* REQUIREMENT:
* A semantic condition that must be satisfied.
* 
* CONSTRAINT:
* A restriction on acceptable realization.
* 
* PREFERENCE:
* A desired property that may be relaxed when necessary.
* 
* PROPERTY:
* Descriptive metadata that does not itself imply a requirement.
* 
* HINT:
* Optional implementation guidance where the surrounding language permits
* it.
* 
* This grammar does not decide whether any of these conditions are satisfiable.
* 
* ============================================================================
* PHYSICAL REALIZATION BOUNDARY
* ============================================================================
* 
* This grammar MUST NOT require:
* 
* physical_device_id
* serial_number
* PCI address
* MAC address
* physical_qubit_index
* physical_core_index
* physical_gpu_index
* physical_memory_bank
* vendor-specific device identifier
* 
* Physical realization belongs downstream.
* 
* ============================================================================
* DOMAIN BOUNDARY
* ============================================================================
* 
* Topology is domain-neutral.
* 
* The same topology model may describe relationships among:
* 
* CPU resources
* GPU resources
* FPGA resources
* ASIC resources
* accelerator resources
* quantum resources
* memory resources
* communication resources
* distributed resources
* heterogeneous resources
* 
* Domain-specific semantics belong to the relevant semantic and IR layers.
* 
* Quantum topology does NOT define quantum operations here.
* 
* Quantum computation follows:
* 
* source
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic analysis
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* routing
*   |
*   v
* scheduling
*   |
*   v
* QEC / resilience
*   |
*   v
* ZQN
*   |
*   v
* HAL
*   |
*   v
* target realization
* 
* This grammar does not create or replace quantum::ir.
* 
* ============================================================================
* LEXICAL INTEGRATION
* ============================================================================
* 
* This parser consumes the canonical Zamani lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* through:
* 
* tokenVocab = ZamaniLexer;
* 
* No private lexer rules are permitted here.
* 
* IMPORTANT:
* 
* The repository's actual lexer vocabulary uses canonical token names rather
* than the K_* names used by the previous topology grammar.
* 
* Therefore this file intentionally uses the canonical lexical names such as:
* 
* TOPOLOGY
* NODE
* EDGE
* LINK
* CONNECT
* FROM
* TO
* GROUP
* EXTENDS
* REQUIRES
* CONSTRAINT
* PREFER
* PROPERTY
* WHERE
* TYPE
* STATIC
* ABSTRACT
* PUBLIC
* PRIVATE
* PROTECTED
* INTERNAL
* 
* The lexical vocabulary must contain these concepts exactly once.
* 
* If a topology-specific keyword is not yet present in the canonical lexer,
* it MUST be added to the canonical lexical vocabulary before this parser is
* generated. It MUST NOT be defined locally in this file.
* 
* ============================================================================
* SHARED EXPRESSION INTEGRATION
* ============================================================================
* 
* This file intentionally DOES NOT define a second arithmetic/logical
* expression grammar.
* 
* The previous topology grammar contained:
* 
* topologyExpression
* topologyAdditiveExpression
* topologyMultiplicativeExpression
* topologyUnaryExpression
* topologyPrimaryExpression
* 
* That duplicated the language-wide expression system.
* 
* Production topology syntax must consume the repository's canonical
* expression rule.
* 
* The composition parser therefore supplies:
* 
* expression
* 
* or the repository's final canonical expression entry rule.
* 
* The exact exported rule name must remain aligned with
* grammar/antlr/ZamaniParser.g4.
* 
* ============================================================================
* SHARED NAME INTEGRATION
* ============================================================================
* 
* Topology must reuse the canonical name/path representation.
* 
* It must not create a second incompatible qualified-name system.
* 
* The composition parser supplies:
* 
* qualifiedName
* 
* or the repository's canonical name rule.
* 
* ============================================================================
* SHARED TYPE INTEGRATION
* ============================================================================
* 
* Topology does not define a second type system.
* 
* Type annotations should consume the canonical type rule supplied by the
* parser composition layer.
* 
* ============================================================================
* GRAMMAR COMPOSITION
* ============================================================================
* 
* This file is a parser component.
* 
* It is consumed by:
* 
* grammar/antlr/ZamaniParser.g4
* 
* and ultimately:
* 
* grammar/Zamani.g4
* 
* This file MUST NOT be imported directly by:
* 
* grammar/Zamani.g4
* 
* The canonical composition chain is:
* 
* Zamani.g4
*     |
*     +--> ZamaniParser.g4
*             |
*             +--> hardware grammar
*                     |
*                     +--> topology.g4
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* Every accepted topology construct must map to domain-neutral AST data.
* 
* Recommended semantic AST entities are:
* 
* HardwareTopologyDecl
* TopologyNodeDecl
* TopologyGroupDecl
* TopologyEndpoint
* TopologyRelationshipDecl
* TopologyProperty
* TopologyRequirement
* TopologyConstraint
* TopologyPreference
* TopologyPredicate
* TopologyExtension
* 
* Each AST entity must preserve:
* 
* - source span;
* - source order where meaningful;
* - identifier/path;
* - expressions;
* - modifiers;
* - annotations;
* - relationship direction;
* - topology attributes.
* 
* This grammar does not construct those AST objects.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis is responsible for:
* 
* - resolving topology names;
* - resolving node/group references;
* - validating endpoint compatibility;
* - validating cardinality expressions;
* - validating relationship direction;
* - validating metric types;
* - validating units;
* - validating topology predicates;
* - distinguishing requirements/constraints/preferences;
* - checking satisfiability;
* - detecting contradictory declarations;
* - checking whether a topology is static or symbolic;
* - determining target-dependent realizability.
* 
* The parser MUST NOT perform these operations.
* 
* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Topology may refer to resources and capabilities semantically.
* 
* However:
* 
* topology.g4
* 
* does not redefine:
* 
* resources.g4
* capabilities.g4
* 
* Resource availability belongs to the resource subsystem.
* 
* Capability availability belongs to the capability subsystem.
* 
* Example semantic intent:
* 
* requires capability("communication")
* 
* is source-level intent.
* 
* Whether a target provides the capability is determined downstream.
* 
* ============================================================================
* PLACEMENT / ROUTING / SCHEDULING
* ============================================================================
* 
* Topology is NOT placement.
* 
* Topology is NOT routing.
* 
* Topology is NOT scheduling.
* 
* The separation is:
* 
* topology
*     =
* abstract relationship requirements
* 
* placement
*     =
* mapping logical resources to realizable resources
* 
* routing
*     =
* finding realizable paths
* 
* scheduling
*     =
* ordering resource use in time
* 
* Therefore topology.g4 MUST NOT contain:
* 
* route(...)
* schedule(...)
* place(...)
* 
* implementations or physical allocation semantics.
* 
* ============================================================================
* INTERCONNECT BOUNDARY
* ============================================================================
* 
* The repository contains hardware interconnect concepts.
* 
* topology.g4 owns the abstract graph relationship.
* 
* hardware/interconnect.g4 owns reusable interconnect/interface contracts.
* 
* topology.g4 must not duplicate physical or protocol-level interconnect
* declarations.
* 
* ============================================================================
* OPEN-WORLD TOPOLOGY MODEL
* ============================================================================
* 
* Topology families are intentionally open.
* 
* The grammar does not enumerate:
* 
* ring
* mesh
* torus
* tree
* star
* grid
* hypercube
* heavy_hex
* 
* These are represented by:
* 
* identifiers
* qualified names
* properties
* relationships
* predicates
* 
* This permits future architectures without changing the core grammar merely
* because a new topology family is invented.
* 
* ============================================================================
* DECLARATION MODEL
* ============================================================================
* 
* A topology declaration has the conceptual form:
* 
* topology Name {
*     ...
* }
* 
* A topology may contain:
* 
* node declarations
* group declarations
* relationship declarations
* property declarations
* requirement declarations
* constraint declarations
* preference declarations
* predicate declarations
* extension declarations
* 
* ============================================================================
  */

parser grammar ZamaniHardwareTopologyParser;

options {
tokenVocab = ZamaniLexer;
}

// ============================================================================
// 1. PUBLIC TOPOLOGY ENTRY POINT
// ============================================================================

topologyDeclaration
: topologyAnnotation*
topologyVisibility?
topologyModifier*
TOPOLOGY
qualifiedName
topologyTypeParameters?
topologyExtendsClause?
topologyBody
;

// ============================================================================
// 2. VISIBILITY
// ============================================================================

topologyVisibility
: PUBLIC
| PRIVATE
| PROTECTED
| INTERNAL
;

// ============================================================================
// 3. MODIFIERS
// ============================================================================
//
// Only modifiers that have already been established by the shared language
// vocabulary are permitted here.
//

topologyModifier
: STATIC
| ABSTRACT
;

// ============================================================================
// 4. ANNOTATIONS
// ============================================================================
//
// Annotation syntax is owned lexically by the canonical annotation grammar.
// The topology grammar only consumes it.
//

topologyAnnotation
: AT
qualifiedName
topologyAnnotationArguments?
;

topologyAnnotationArguments
: LPAREN
topologyArgumentList?
RPAREN
;

topologyArgumentList
: expression
(COMMA expression)*
COMMA?
;

// ============================================================================
// 5. GENERIC / PARAMETERIZED TOPOLOGIES
// ============================================================================
//
// Parameters are semantic values, not hardware limits.
//
// Example:
//
//     topology Network<N> { ... }
//
// N may represent any valid semantic quantity.
//

topologyTypeParameters
: LT
topologyTypeParameter
(COMMA topologyTypeParameter)*
COMMA?
GT
;

topologyTypeParameter
: IDENTIFIER
topologyTypeParameterBound?
topologyTypeParameterDefault?
;

topologyTypeParameterBound
: COLON
type
;

topologyTypeParameterDefault
: ASSIGN
expression
;

// ============================================================================
// 6. EXTENSION / COMPOSITION
// ============================================================================
//
// Extending a topology describes logical composition.
//
// It does not mean physical inheritance.
//

topologyExtendsClause
: EXTENDS
qualifiedName
(COMMA qualifiedName)*
;

// ============================================================================
// 7. TOPOLOGY BODY
// ============================================================================

topologyBody
: LBRACE
topologyMember*
RBRACE
;

// ============================================================================
// 8. TOPOLOGY MEMBER DISPATCH
// ============================================================================
//
// Keep the alternatives structurally distinguishable.
//
// Domain-specific semantics remain downstream.
//

topologyMember
: topologyAnnotation*
(
topologyNodeDeclaration
| topologyGroupDeclaration
| topologyRelationshipDeclaration
| topologyPropertyDeclaration
| topologyRequirementDeclaration
| topologyConstraintDeclaration
| topologyPreferenceDeclaration
| topologyPredicateDeclaration
| topologyUsingDeclaration
)
;

// ============================================================================
// 9. NODE DECLARATIONS
// ============================================================================
//
// Nodes are logical topology participants.
//
// A node declaration never means a physical device identifier.
//

topologyNodeDeclaration
: NODE
qualifiedName
topologyNodeTypeClause?
topologyCardinalityClause?
topologyNodePropertyBlock?
SEMICOLON
;

topologyNodeTypeClause
: COLON
qualifiedName
;

// ============================================================================
// 10. NODE CARDINALITY
// ============================================================================
//
// Cardinality is an expression.
//
// No fixed maximum is encoded.
//

topologyCardinalityClause
: LBRACKET
expression
RBRACKET
;

// ============================================================================
// 11. NODE PROPERTY BLOCK
// ============================================================================
//
// Property names remain open and qualified.
//
// This prevents topology.g4 from becoming a closed dictionary of today's
// hardware attributes.
//

topologyNodePropertyBlock
: LBRACE
topologyPropertyStatement*
RBRACE
;

// ============================================================================
// 12. GROUP DECLARATIONS
// ============================================================================
//
// Groups describe sets/classes of logical topology participants.
//
// They may be explicitly named or selected symbolically.
//

topologyGroupDeclaration
: GROUP
qualifiedName
topologyGroupTypeClause?
topologyGroupCardinalityClause?
topologyGroupSelectorClause?
topologyGroupBody?
SEMICOLON?
;

topologyGroupTypeClause
: COLON
qualifiedName
;

topologyGroupCardinalityClause
: LBRACKET
expression
RBRACKET
;

topologyGroupSelectorClause
: WHERE
topologyPredicateExpression
;

topologyGroupBody
: LBRACE
topologyGroupMember*
RBRACE
;

topologyGroupMember
: qualifiedName
(COMMA qualifiedName)*
SEMICOLON
;

// ============================================================================
// 13. TOPOLOGY RELATIONSHIPS
// ============================================================================
//
// A single relationship rule replaces the old overlapping:
//
//     edge
//     link
//     connect
//
// model.
//
// The relationship kind remains open through an optional named kind/property
// rather than forcing multiple semantically overlapping grammar constructs.
//

topologyRelationshipDeclaration
: topologyRelationshipKeyword
topologyRelationshipName?
topologyEndpoint
topologyRelationshipOperator
topologyEndpoint
topologyRelationshipDirection?
topologyRelationshipCondition?
topologyRelationshipPropertyBlock?
SEMICOLON
;

// ============================================================================
// 14. RELATIONSHIP KEYWORD
// ============================================================================
//
// CONNECT is the preferred source-level relationship spelling.
//
// EDGE/LINK may be retained by the lexical compatibility layer if they are
// already stable language vocabulary. They have identical syntactic ownership
// here and are not separate semantic models.
//

topologyRelationshipKeyword
: CONNECT
| EDGE
| LINK
;

// ============================================================================
// 15. RELATIONSHIP NAME
// ============================================================================

topologyRelationshipName
: qualifiedName
;

// ============================================================================
// 16. RELATIONSHIP OPERATOR
// ============================================================================
//
// TO expresses the ordered source relationship:
//
//     A to B
//
// Direction is separately represented so that a relationship can be:
//
//     directed
//     undirected
//     bidirectional
//
// without creating separate AST concepts.
//

topologyRelationshipOperator
: TO
;

// ============================================================================
// 17. RELATIONSHIP DIRECTION
// ============================================================================

topologyRelationshipDirection
: BIDIRECTIONAL
| DIRECTED
| UNDIRECTED
;

// ============================================================================
// 18. RELATIONSHIP CONDITION
// ============================================================================
//
// Conditions constrain which members of symbolic endpoint sets participate.
//

topologyRelationshipCondition
: WHERE
topologyPredicateExpression
;

// ============================================================================
// 19. ENDPOINTS
// ============================================================================
//
// Endpoints identify logical resources/groups/classes.
//
// They may be parameterized by symbolic selectors.
//
// Physical resource IDs are intentionally not represented.
//

topologyEndpoint
: qualifiedName
topologyEndpointSelector*
;

// ============================================================================
// 20. ENDPOINT SELECTORS
// ============================================================================
//
// Selectors are expressions, not fixed-size indexes.
//
// Examples:
//
//     compute[*]
//     compute[i]
//     compute[worker]
//     compute[condition]
//
// The semantic layer determines whether a selector is valid.
//

topologyEndpointSelector
: LBRACKET
topologySelectorExpression
RBRACKET
;

topologySelectorExpression
: expression
;

// ============================================================================
// 21. RELATIONSHIP PROPERTIES
// ============================================================================
//
// Properties are open-world.
//
// Metrics such as latency, bandwidth, capacity, reliability, cost and weight
// may be represented without forcing a closed topology vocabulary.
//

topologyRelationshipPropertyBlock
: LBRACE
topologyPropertyStatement*
RBRACE
;

// ============================================================================
// 22. PROPERTY STATEMENTS
// ============================================================================

topologyPropertyStatement
: qualifiedName
topologyPropertyTypeClause?
topologyPropertyValueClause?
SEMICOLON
;

topologyPropertyTypeClause
: COLON
type
;

topologyPropertyValueClause
: ASSIGN
expression
;

// ============================================================================
// 23. TOPOLOGY PROPERTY DECLARATIONS
// ============================================================================
//
// A property is metadata.
//
// It does not itself impose a target requirement.
//

topologyPropertyDeclaration
: PROPERTY
qualifiedName
topologyPropertyTypeClause?
topologyPropertyValueClause?
SEMICOLON
;

// ============================================================================
// 24. REQUIREMENTS
// ============================================================================
//
// Requirements are mandatory semantic conditions.
//
// They remain separate from preferences.
//

topologyRequirementDeclaration
: REQUIRES
topologyRequirementExpression
SEMICOLON
;

topologyRequirementExpression
: topologyRequirementOperand
(
topologyLogicalOperator
topologyRequirementOperand
)*
;

topologyRequirementOperand
: topologyConnectivityRequirement
| topologyComparisonExpression
| qualifiedName
| LPAREN
topologyRequirementExpression
RPAREN
;

// ============================================================================
// 25. CONNECTIVITY REQUIREMENTS
// ============================================================================

topologyConnectivityRequirement
: topologyEndpoint
topologyConnectivityOperator
topologyEndpoint
;

// ============================================================================
// 26. CONNECTIVITY OPERATORS
// ============================================================================
//
// These express semantic relationships.
//
// They do not invoke routing.
//

topologyConnectivityOperator
: CONNECTS
| ADJACENT
| NEIGHBOR
;

// ============================================================================
// 27. CONSTRAINTS
// ============================================================================

topologyConstraintDeclaration
: CONSTRAINT
topologyConstraintExpression
SEMICOLON
;

topologyConstraintExpression
: topologyLogicalExpression
;

// ============================================================================
// 28. PREFERENCES
// ============================================================================
//
// Preferences are optimization guidance rather than mandatory semantic
// requirements.
//

topologyPreferenceDeclaration
: PREFER
topologyPreferenceExpression
SEMICOLON
;

topologyPreferenceExpression
: topologyLogicalExpression
;

// ============================================================================
// 29. PREDICATE DECLARATIONS
// ============================================================================
//
// A predicate declaration provides reusable symbolic topology conditions.
//

topologyPredicateDeclaration
: WHERE
topologyPredicateExpression
SEMICOLON
;

// ============================================================================
// 30. PREDICATE EXPRESSIONS
// ============================================================================
//
// Logical precedence is represented structurally.
//
// The expression grammar remains owned by the universal expression system for
// arithmetic/value expressions.
//

topologyPredicateExpression
: topologyLogicalExpression
;

topologyLogicalExpression
: topologyLogicalTerm
(
OR
topologyLogicalTerm
)*
;

topologyLogicalTerm
: topologyLogicalFactor
(
AND
topologyLogicalFactor
)*
;

topologyLogicalFactor
: NOT topologyLogicalFactor
| LPAREN topologyPredicateExpression RPAREN
| topologyConnectivityPredicate
| topologyComparisonExpression
| qualifiedName
;

// ============================================================================
// 31. CONNECTIVITY PREDICATES
// ============================================================================

topologyConnectivityPredicate
: topologyEndpoint
topologyConnectivityOperator
topologyEndpoint
;

// ============================================================================
// 32. COMPARISON EXPRESSIONS
// ============================================================================
//
// Value expressions are delegated to the universal expression grammar.
//
// Only the relationship operator is owned here.
//

topologyComparisonExpression
: expression
topologyComparisonOperator
expression
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
// 33. LOGICAL OPERATORS
// ============================================================================
//
// The canonical lexical vocabulary owns AND/OR/NOT.
//

topologyLogicalOperator
: AND
| OR
;

// ============================================================================
// 34. TOPOLOGY USING DECLARATIONS
// ============================================================================
//
// Name import/alias semantics remain owned by the module/name-resolution
// subsystem. This rule only represents topology-local source syntax where the
// parent parser permits it.
//

topologyUsingDeclaration
: USE
qualifiedName
topologyUsingAlias?
SEMICOLON
;

topologyUsingAlias
: AS
IDENTIFIER
;

// ============================================================================
// 35. TOPOLOGY FAMILY REFERENCE
// ============================================================================
//
// There is intentionally no closed enumeration.
//

topologyFamilyReference
: qualifiedName
;

// ============================================================================
// 36. TOPOLOGY SCHEMA REFERENCE
// ============================================================================

topologySchemaReference
: TYPE
qualifiedName
;

// ============================================================================
// 37. RELATIONSHIP REFERENCE
// ============================================================================
//
// This is source-level relationship syntax only.
//

topologyRelationshipReference
: topologyEndpoint
topologyConnectivityOperator
topologyEndpoint
;

// ============================================================================
// 38. REQUIREMENT / CONSTRAINT / PREFERENCE SETS
// ============================================================================
//
// These rules are reusable parser components. They do not introduce an
// additional semantic representation.
//

topologyRequirementSet
: LBRACE
topologyRequirementDeclaration*
RBRACE
;

topologyConstraintSet
: LBRACE
topologyConstraintDeclaration*
RBRACE
;

topologyPreferenceSet
: LBRACE
topologyPreferenceDeclaration*
RBRACE
;

// ============================================================================
// 39. TOPOLOGY CARDINALITY
// ============================================================================
//
// Generic cardinality remains an expression.
//
// There is no grammar-level upper bound.
//

topologyMultiplicity
: LBRACKET
expression
RBRACKET
;

// ============================================================================
// 40. TOPOLOGY METADATA CONVENTION
// ============================================================================
//
// The following names are intentionally NOT lexer keywords:
//
//     latency
//     bandwidth
//     capacity
//     distance
//     cost
//     weight
//     reliability
//     degree
//     diameter
//     throughput
//
// They remain names/properties unless the repository's canonical semantic
// specification explicitly makes one a language keyword.
//
// This prevents topology.g4 from becoming a permanently closed dictionary.
//
// Example:
//
//     property latency = required_latency;
//
// or:
//
//     connect compute[] to memory[] {
//         latency = required_latency;
//         bandwidth = required_bandwidth;
//     }
//
// Unit interpretation belongs to the shared type/resource system.
//
//
// ============================================================================
// 41. SYMBOLIC SCALABILITY
// ============================================================================
//
// These forms are syntactically valid:
//
//     node compute;
//     node compute[workers];
//     node compute[count];
//     group workers[available_workers];
//     connect compute[] to storage[];
//     connect compute[i] to storage[i];
//     requires compute connects storage;
//     requires latency <= required_latency;
//
// The actual cardinality is resolved downstream.
//
// No source grammar limit is implied.
//
//
// ============================================================================
// 42. TOPOLOGY SEMANTIC STATES
// ============================================================================
//
// The semantic layer may classify a topology as:
//
//     static
//     symbolic
//     dynamic
//     target-dependent
//     runtime-dependent
//
// The parser does not encode those states as separate topology ASTs.
//
//
// ============================================================================
// 43. PHYSICAL TOPOLOGY PROHIBITION
// ============================================================================
//
// The following are NOT topology-language requirements:
//
//     device gpu0;
//     physical_qubit 17;
//     pci 0000:01:00.0;
//     memory_bank 3;
//     cpu_core 7;
//
// Such physical realization belongs to target/deployment/HAL layers.
//
// The topology grammar must remain valid without those concepts.
//
//
// ============================================================================
// 44. HARD-CODING PROHIBITION
// ============================================================================
//
// This file MUST NOT introduce:
//
//     MAX_NODES
//     MAX_EDGES
//     MAX_LINKS
//     MAX_PORTS
//     MAX_DEVICES
//     MAX_QUBITS
//     MAX_CPUS
//     MAX_CORES
//     MAX_THREADS
//     MAX_GPUS
//     MAX_FPGAS
//     MAX_MEMORY
//     MAX_ACCELERATORS
//     MAX_TIMELINES
//
// It must also not encode implicit fixed sizes such as:
//
//     node[32]
//     topology<64>
//     qpu[128]
//
// as language-wide restrictions.
//
// Numeric values appearing in source remain ordinary program semantics.
//
//
// ============================================================================
// 45. DETERMINISM
// ============================================================================
//
// Parsing MUST depend only on:
//
//     source text
//     grammar version
//     lexical vocabulary
//     parser configuration
//     explicitly selected language/dialect configuration
//
// Parsing MUST NOT depend on:
//
//     hardware availability
//     target discovery
//     filesystem state
//     network state
//     wall-clock time
//     randomness
//     environment state
//     runtime state
//     device state
//
// Identical source under identical grammar/configuration must produce identical
// parse structure and diagnostics.
//
//
// ============================================================================
// 46. ERROR BOUNDARY
// ============================================================================
//
// Parser errors include only syntactic invalidity.
//
// Examples:
//
//     missing topology name
//     missing topology body
//     malformed endpoint
//     malformed relationship
//     malformed property
//     malformed requirement
//
// Semantic errors belong downstream:
//
//     unknown topology name
//     unknown node
//     duplicate node
//     impossible cardinality
//     incompatible endpoints
//     unsatisfied capability
//     insufficient resource
//     impossible latency requirement
//     unsupported topology
//
// Resource exhaustion of the compiler/parser itself is not a language semantic
// error.
//
//
// ============================================================================
// 47. AST / SEMANTIC / IR BOUNDARY
// ============================================================================
//
// The topology grammar lowers through:
//
//     topology syntax
//         |
//         v
//     domain-neutral AST
//         |
//         v
//     topology semantic model
//         |
//         +--> resource/capability analysis
//         |
//         +--> hardware target analysis
//         |
//         +--> placement
//         |
//         +--> routing
//         |
//         +--> scheduling
//         |
//         v
//     target realization
//
// Quantum programs continue through the canonical:
//
//     quantum::ir
//
// boundary.
//
// topology.g4 does not define another IR.
//
//
// ============================================================================
// 48. HARDWARE HAL BOUNDARY
// ============================================================================
//
// The HAL may discover:
//
//     actual devices
//     actual links
//     actual capabilities
//     actual topology
//     availability
//     measured latency
//     measured bandwidth
//     reliability
//
// Those are implementation/target facts.
//
// They must never be silently inserted into parsing.
//
//
// ============================================================================
// 49. SECURITY BOUNDARY
// ============================================================================
//
// A topology declaration does not grant permission to:
//
//     access hardware
//     allocate hardware
//     access a network
//     access another process
//     access another user's device
//     bypass authorization
//
// Authorization belongs to the security/capability system.
//
//
// ============================================================================
// 50. COMPATIBILITY
// ============================================================================
//
// The following are compatibility-sensitive:
//
//     topology declaration spelling
//     topology member syntax
//     relationship syntax
//     direction keywords
//     endpoint selector syntax
//     requirement syntax
//     constraint syntax
//     preference syntax
//     property syntax
//     token names
//
// Changes must be recorded through:
//
//     grammar/compatibility/
//     grammar/spec/compatibility.md
//
// Historical/experimental syntax in:
//
//     grammar/Zamani-Grammar.md
//
// does not automatically become canonical syntax.
//
//
// ============================================================================
// 51. TEST CONTRACT
// ============================================================================
//
// Production validation must include:
//
// POSITIVE
//
//     minimal topology
//     one node
//     multiple nodes
//     symbolic node cardinality
//     groups
//     symbolic groups
//     directed relationship
//     undirected relationship
//     bidirectional relationship
//     relationship properties
//     requirements
//     constraints
//     preferences
//     predicates
//     topology extension
//     qualified names
//     parameterized topology
//     classical hardware topology
//     accelerator topology
//     quantum hardware topology
//     heterogeneous topology
//     distributed topology
//
// NEGATIVE
//
//     missing topology name
//     missing body
//     malformed node
//     malformed group
//     malformed endpoint
//     missing relationship target
//     malformed selector
//     malformed property
//     malformed requirement
//     malformed constraint
//     malformed predicate
//
// BOUNDARY
//
//     empty topology
//     one logical participant
//     large logical participant set
//     symbolic cardinality
//     deeply nested predicates
//     large property sets
//     long qualified names
//
// SCALABILITY
//
//     increasing node declarations
//     increasing relationship declarations
//     increasing group declarations
//     symbolic cardinality
//     large expressions
//     large source files
//
// DETERMINISM
//
//     same source -> same parse result
//     same source -> same diagnostics
//
// CROSS-DOMAIN
//
//     classical + topology
//     quantum + topology
//     HDL + topology
//     hybrid + topology
//     distributed + topology
//     AI + topology
//     heterogeneous + topology
//
//
// ============================================================================
// 52. REPOSITORY INTEGRATION CONTRACT
// ============================================================================
//
// Upstream:
//
//     grammar/lexer/*
//     grammar/core/*
//     grammar/types/*
//     grammar/expressions/*
//
// Composition:
//
//     grammar/antlr/ZamaniParser.g4
//
// Root:
//
//     grammar/Zamani.g4
//
// Sibling hardware grammars:
//
//     hardware.g4
//     devices.g4
//     resources.g4
//     capabilities.g4
//     targets.g4
//     interconnect.g4
//     placement.g4
//
// Related semantic systems:
//
//     resource analysis
//     capability analysis
//     hardware semantic model
//     target resolution
//     placement
//     routing
//     scheduling
//     HAL
//
// Quantum boundary:
//
//     quantum::ir
//
// Quantum downstream systems:
//
//     QEC
//     ZQN
//     resilience
//     routing
//     scheduling
//
// This grammar must not duplicate those systems.
//
//
// ============================================================================
// 53. RUST 1.97 / 1.97.1 CONTRACT
// ============================================================================
//
// This file contains no Rust implementation.
//
// Generated parser integration must remain compatible with:
//
//     Rust 1.97
//     Rust 1.97.1
//     Rust 2021
//
// The repository's compiler/frontend implementation must remain safe Rust.
//
// No "unsafe" code is required by this grammar.
//
//
// ============================================================================
// 54. COMPLETION CRITERIA
// ============================================================================
//
// This file is complete only when:
//
//     [ ] canonical lexer provides all referenced tokens;
//     [ ] canonical parser composition imports this grammar;
//     [ ] canonical expression rule is available;
//     [ ] canonical type rule is available;
//     [ ] canonical qualified-name rule is available;
//     [ ] topology AST mapping exists;
//     [ ] topology semantic model exists;
//     [ ] topology resource/capability integration exists;
//     [ ] topology target-resolution integration exists;
//     [ ] topology placement integration exists;
//     [ ] topology routing integration exists;
//     [ ] topology scheduling remains downstream;
//     [ ] physical discovery remains HAL-owned;
//     [ ] quantum semantics remain quantum::ir-owned;
//     [ ] QEC remains outside this grammar;
//     [ ] ZQN remains outside this grammar;
//     [ ] no physical device enumeration exists;
//     [ ] no machine-size limit exists;
//     [ ] no MAX_* topology constant exists;
//     [ ] no vendor topology enumeration exists;
//     [ ] no duplicate expression grammar exists;
//     [ ] no duplicate type grammar exists;
//     [ ] no duplicate name grammar exists;
//     [ ] positive tests exist;
//     [ ] negative tests exist;
//     [ ] boundary tests exist;
//     [ ] scalability tests exist;
//     [ ] determinism tests exist;
//     [ ] cross-domain tests exist;
//     [ ] compatibility tests exist;
//     [ ] ANTLR generation succeeds;
//     [ ] generated Rust contains no unsafe requirement;
//     [ ] repository parser conformance succeeds;
//     [ ] documentation matches implementation.
//
//
// ============================================================================
// 55. FINAL ARCHITECTURAL INVARIANT
// ============================================================================
//
// topology.g4 describes:
//
//     abstract topology intent.
//
// It does NOT describe:
//
//     physical topology realization.
//
// Therefore:
//
//     topology
//         !=
//     placement
//
//     topology
//         !=
//     routing
//
//     topology
//         !=
//     scheduling
//
//     topology
//         !=
//     calibration
//
//     topology
//         !=
//     QEC
//
//     topology
//         !=
//     ZQN
//
//     topology
//         !=
//     HAL
//
//     topology
//         !=
//     runtime
//
// The complete architectural path remains:
//
//     Zamani source
//          |
//          v
//     canonical lexer
//          |
//          v
//     canonical parser
//          |
//          v
//     domain-neutral AST
//          |
//          v
//     semantic analysis
//          |
//          v
//     canonical IR / semantic models
//          |
//          v
//     optimization
//          |
//          v
//     placement / routing / scheduling
//          |
//          v
//     resilience / QEC / ZQN where applicable
//          |
//          v
//     Hardware HAL
//          |
//          v
//     target realization
//
// The source program therefore remains independent of today's hardware scale.
//
//     tiny
//       |
//     CPU
//       |
//     GPU
//       |
//     FPGA
//       |
//     ASIC
//       |
//     QPU
//       |
//     heterogeneous
//       |
//     distributed
//       |
//     future
//
// The available resources determine realization.
//
// The source-level topology contract does not impose the resource ceiling.
//
// ============================================================================
// END OF grammar/hardware/topology.g4
// ============================================================================
*/
parser grammar ZamaniHardwareTopologyParser;

options {
tokenVocab = ZamaniLexer;
}

topologyDeclaration
: topologyAnnotation*
topologyVisibility?
topologyModifier*
TOPOLOGY
qualifiedName
topologyTypeParameters?
topologyExtendsClause?
topologyBody
;

topologyVisibility
: PUBLIC
| PRIVATE
| PROTECTED
| INTERNAL
;

topologyModifier
: STATIC
| ABSTRACT
;

topologyAnnotation
: AT
qualifiedName
topologyAnnotationArguments?
;

topologyAnnotationArguments
: LPAREN
topologyArgumentList?
RPAREN
;

topologyArgumentList
: expression
(COMMA expression)*
COMMA?
;

topologyTypeParameters
: LT
topologyTypeParameter
(COMMA topologyTypeParameter)*
COMMA?
GT
;

topologyTypeParameter
: IDENTIFIER
topologyTypeParameterBound?
topologyTypeParameterDefault?
;

topologyTypeParameterBound
: COLON
type
;

topologyTypeParameterDefault
: ASSIGN
expression
;

topologyExtendsClause
: EXTENDS
qualifiedName
(COMMA qualifiedName)*
;

topologyBody
: LBRACE
topologyMember*
RBRACE
;

topologyMember
: topologyAnnotation*
(
topologyNodeDeclaration
| topologyGroupDeclaration
| topologyRelationshipDeclaration
| topologyPropertyDeclaration
| topologyRequirementDeclaration
| topologyConstraintDeclaration
| topologyPreferenceDeclaration
| topologyPredicateDeclaration
| topologyUsingDeclaration
)
;

topologyNodeDeclaration
: NODE
qualifiedName
topologyNodeTypeClause?
topologyCardinalityClause?
topologyNodePropertyBlock?
SEMICOLON
;

topologyNodeTypeClause
: COLON
qualifiedName
;

topologyCardinalityClause
: LBRACKET
expression
RBRACKET
;

topologyNodePropertyBlock
: LBRACE
topologyPropertyStatement*
RBRACE
;

topologyGroupDeclaration
: GROUP
qualifiedName
topologyGroupTypeClause?
topologyGroupCardinalityClause?
topologyGroupSelectorClause?
topologyGroupBody?
SEMICOLON?
;

topologyGroupTypeClause
: COLON
qualifiedName
;

topologyGroupCardinalityClause
: LBRACKET
expression
RBRACKET
;

topologyGroupSelectorClause
: WHERE
topologyPredicateExpression
;

topologyGroupBody
: LBRACE
topologyGroupMember*
RBRACE
;

topologyGroupMember
: qualifiedName
(COMMA qualifiedName)*
SEMICOLON
;

topologyRelationshipDeclaration
: topologyRelationshipKeyword
topologyRelationshipName?
topologyEndpoint
topologyRelationshipOperator
topologyEndpoint
topologyRelationshipDirection?
topologyRelationshipCondition?
topologyRelationshipPropertyBlock?
SEMICOLON
;

topologyRelationshipKeyword
: CONNECT
| EDGE
| LINK
;

topologyRelationshipName
: qualifiedName
;

topologyRelationshipOperator
: TO
;

topologyRelationshipDirection
: BIDIRECTIONAL
| DIRECTED
| UNDIRECTED
;

topologyRelationshipCondition
: WHERE
topologyPredicateExpression
;

topologyEndpoint
: qualifiedName
topologyEndpointSelector*
;

topologyEndpointSelector
: LBRACKET
topologySelectorExpression
RBRACKET
;

topologySelectorExpression
: expression
;

topologyRelationshipPropertyBlock
: LBRACE
topologyPropertyStatement*
RBRACE
;

topologyPropertyStatement
: qualifiedName
topologyPropertyTypeClause?
topologyPropertyValueClause?
SEMICOLON
;

topologyPropertyTypeClause
: COLON
type
;

topologyPropertyValueClause
: ASSIGN
expression
;

topologyPropertyDeclaration
: PROPERTY
qualifiedName
topologyPropertyTypeClause?
topologyPropertyValueClause?
SEMICOLON
;

topologyRequirementDeclaration
: REQUIRES
topologyRequirementExpression
SEMICOLON
;

topologyRequirementExpression
: topologyRequirementOperand
(
topologyLogicalOperator
topologyRequirementOperand
)*
;

topologyRequirementOperand
: topologyConnectivityRequirement
| topologyComparisonExpression
| qualifiedName
| LPAREN
topologyRequirementExpression
RPAREN
;

topologyConnectivityRequirement
: topologyEndpoint
topologyConnectivityOperator
topologyEndpoint
;

topologyConnectivityOperator
: CONNECTS
| ADJACENT
| NEIGHBOR
;

topologyConstraintDeclaration
: CONSTRAINT
topologyConstraintExpression
SEMICOLON
;

topologyConstraintExpression
: topologyLogicalExpression
;

topologyPreferenceDeclaration
: PREFER
topologyPreferenceExpression
SEMICOLON
;

topologyPreferenceExpression
: topologyLogicalExpression
;

topologyPredicateDeclaration
: WHERE
topologyPredicateExpression
SEMICOLON
;

topologyPredicateExpression
: topologyLogicalExpression
;

topologyLogicalExpression
: topologyLogicalTerm
(
OR
topologyLogicalTerm
)*
;

topologyLogicalTerm
: topologyLogicalFactor
(
AND
topologyLogicalFactor
)*
;

topologyLogicalFactor
: NOT topologyLogicalFactor
| LPAREN topologyPredicateExpression RPAREN
| topologyConnectivityPredicate
| topologyComparisonExpression
| qualifiedName
;

topologyConnectivityPredicate
: topologyEndpoint
topologyConnectivityOperator
topologyEndpoint
;

topologyComparisonExpression
: expression
topologyComparisonOperator
expression
;

topologyComparisonOperator
: EQUAL_EQUAL
| NOT_EQUAL
| LESS
| LESS_EQUAL
| GREATER
| GREATER_EQUAL
;

topologyLogicalOperator
: AND
| OR
;

topologyUsingDeclaration
: USE
qualifiedName
topologyUsingAlias?
SEMICOLON
;

topologyUsingAlias
: AS
IDENTIFIER
;

topologyFamilyReference
: qualifiedName
;

topologySchemaReference
: TYPE
qualifiedName
;

topologyRelationshipReference
: topologyEndpoint
topologyConnectivityOperator
topologyEndpoint
;

topologyRequirementSet
: LBRACE
topologyRequirementDeclaration*
RBRACE
;

topologyConstraintSet
: LBRACE
topologyConstraintDeclaration*
RBRACE
;

topologyPreferenceSet
: LBRACE
topologyPreferenceDeclaration*
RBRACE
;

topologyMultiplicity
: LBRACKET
expression
RBRACKET
;