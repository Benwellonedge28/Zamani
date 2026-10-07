/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/topology.g4
 *
 * GRAMMAR
 * -------
 * ZamaniHardwareTopologyParser
 *
 * STATUS
 * ------
 * PRODUCTION HARDWARE-TOPOLOGY LEAF GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ANTLR
 * -----
 * ANTLR4 parser grammar
 * Action-free
 * Predicate-free
 * Target-independent
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical SOURCE-LEVEL syntax owner for abstract hardware
 * topology intent.
 *
 * A topology describes relationships among LOGICAL computational resources,
 * resource groups, endpoints, and other logical topology participants.
 *
 * It describes WHAT relationship is required, allowed, constrained, or
 * preferred.
 *
 * It does NOT describe HOW a physical machine realizes that relationship.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *     Hardware
 *          |
 *          v
 *     topology.g4
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> names
 *          +--> types
 *          +--> resources
 *          +--> capabilities
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     topology semantic model
 *          |
 *          +--> resource negotiation
 *          +--> capability resolution
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     classical representation   quantum::ir
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              target realization
 *                     |
 *                     v
 *                    HAL
 *
 * This grammar does NOT create an IR.
 *
 * It MUST NOT create or replace:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     topologyDeclaration
 *     topologyBody
 *     topologyMember
 *
 *     topologyNodeDeclaration
 *     topologyNodeTypeClause
 *     topologyNodeCardinality
 *     topologyNodePropertyBlock
 *
 *     topologyGroupDeclaration
 *     topologyGroupTypeClause
 *     topologyGroupCardinality
 *     topologyGroupSelector
 *     topologyGroupBody
 *     topologyGroupMember
 *
 *     topologyRelationshipDeclaration
 *     topologyRelationshipDirection
 *     topologyEndpoint
 *     topologyEndpointSelector
 *     topologyRelationshipCondition
 *     topologyRelationshipPropertyBlock
 *
 *     topologyPropertyDeclaration
 *     topologyPropertyStatement
 *
 *     topologyRequirementDeclaration
 *     topologyConstraintDeclaration
 *     topologyPreferenceDeclaration
 *
 *     topologyConnectivityPredicate
 *
 *     topologyExtendsClause
 *
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     lexer definitions
 *     token spellings
 *     identifiers
 *     qualified names
 *     general expressions
 *     general types
 *     universal requirements
 *     resource expressions
 *     resource negotiation
 *     capability semantics
 *     policies
 *     effects
 *     provenance
 *     placement algorithms
 *     routing algorithms
 *     scheduling algorithms
 *     optimization
 *     hardware discovery
 *     physical device enumeration
 *     physical addresses
 *     physical qubit identities
 *     physical CPU identities
 *     physical GPU identities
 *     physical FPGA identities
 *     physical memory-bank identities
 *     calibration
 *     device drivers
 *     runtime execution
 *     quantum operations
 *     quantum states
 *     quantum circuits
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * LEXICAL AUTHORITY
 * -----------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * SHARED NAME AUTHORITY
 * ---------------------
 *
 *     grammar/core/names.g4
 *
 * SHARED EXPRESSION AUTHORITY
 * ---------------------------
 *
 *     grammar/expressions/expressions.g4
 *
 * SHARED REQUIREMENT AUTHORITY
 * ----------------------------
 *
 *     grammar/core/requirements.g4
 *
 * SHARED CAPABILITY AUTHORITY
 * ---------------------------
 *
 *     grammar/core/capabilities.g4
 *
 * RESOURCE AUTHORITY
 * ------------------
 *
 *     grammar/resources/
 *
 * HARDWARE COMPOSITION ROOT
 * -------------------------
 *
 *     grammar/hardware/hardware.g4
 *
 * UNIVERSAL PARSER ROOT
 * ---------------------
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ROOT GRAMMAR
 * ------------
 *
 *     grammar/Zamani.g4
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This leaf grammar explicitly imports the shared parser authorities that it
 * consumes.
 *
 * ANTLR parser grammar imports are composition boundaries. The resulting
 * composed grammar contains the imported parser rules.
 *
 * The Hardware composition root may also import these grammars through its
 * broader composition tree. Duplicate composition must not create competing
 * rule definitions.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareTopologyParser;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    Requirements
;


/**
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Hardware.g4 exposes topologyDeclaration through:
 *
 *     hardwareDeclaration
 *
 * ZamaniParser.g4 exposes Hardware through the universal parser.
 *
 * There is intentionally NO EOF here because this is a leaf declaration
 * grammar, not the complete program root.
 * ============================================================================
 */

topologyDeclaration
    : topologyVisibility?
      topologyModifier*
      TOPOLOGY
      qualifiedName
      topologyExtendsClause?
      topologyBody
    ;


/**
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 */

topologyVisibility
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


/**
 * ============================================================================
 * MODIFIERS
 * ============================================================================
 *
 * Only modifiers already belonging to the canonical lexical vocabulary are
 * accepted.
 *
 * Semantic validation determines whether a particular modifier combination
 * is meaningful.
 * ============================================================================
 */

topologyModifier
    : STATIC
    | ABSTRACT
    ;


/**
 * ============================================================================
 * TOPOLOGY EXTENSION
 * ============================================================================
 *
 * Extension means logical composition/reuse.
 *
 * It does NOT mean physical hardware inheritance.
 * ============================================================================
 */

topologyExtendsClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/**
 * ============================================================================
 * TOPOLOGY BODY
 * ============================================================================
 */

topologyBody
    : LBRACE
      topologyMember*
      RBRACE
    ;


/**
 * ============================================================================
 * TOPOLOGY MEMBER DISPATCH
 * ============================================================================
 *
 * Each alternative has a distinct leading keyword.
 *
 * This is intentional.
 *
 * It keeps the grammar predictable and avoids semantic predicates.
 * ============================================================================
 */

topologyMember
    : topologyNodeDeclaration
    | topologyGroupDeclaration
    | topologyRelationshipDeclaration
    | topologyPropertyDeclaration
    | topologyRequirementDeclaration
    | topologyConstraintDeclaration
    | topologyPreferenceDeclaration
    ;


/**
 * ============================================================================
 * NODE DECLARATION
 * ============================================================================
 *
 * A node is a LOGICAL participant.
 *
 * It is never a physical device identifier.
 *
 * Examples:
 *
 *     node compute;
 *
 *     node compute[workers];
 *
 *     node compute: compute.resource;
 *
 *     node compute[worker_count] {
 *         role = "compute";
 *     };
 *
 * The cardinality expression is symbolic and has no language-level maximum.
 * ============================================================================
 */

topologyNodeDeclaration
    : NODE
      qualifiedName
      topologyNodeTypeClause?
      topologyNodeCardinality?
      topologyNodePropertyBlock?
      SEMICOLON
    ;


topologyNodeTypeClause
    : COLON
      qualifiedName
    ;


topologyNodeCardinality
    : LBRACKET
      expression
      RBRACKET
    ;


topologyNodePropertyBlock
    : LBRACE
      topologyPropertyStatement*
      RBRACE
    ;


/**
 * ============================================================================
 * GROUP DECLARATION
 * ============================================================================
 *
 * Groups represent logical sets/classes of topology participants.
 *
 * The group itself does not imply a fixed number of members.
 *
 * Examples:
 *
 *     group workers;
 *
 *     group workers[worker_count];
 *
 *     group workers where worker_available;
 *
 *     group workers {
 *         worker0;
 *         worker1;
 *     };
 *
 * Explicit membership is useful for small static logical topologies.
 *
 * Symbolic selection/cardinality is required for scalable topologies.
 * ============================================================================
 */

topologyGroupDeclaration
    : GROUP
      qualifiedName
      topologyGroupTypeClause?
      topologyGroupCardinality?
      topologyGroupSelector?
      topologyGroupBody?
      SEMICOLON
    ;


topologyGroupTypeClause
    : COLON
      qualifiedName
    ;


topologyGroupCardinality
    : LBRACKET
      expression
      RBRACKET
    ;


topologyGroupSelector
    : WHERE
      expression
    ;


topologyGroupBody
    : LBRACE
      topologyGroupMember*
      RBRACE
    ;


topologyGroupMember
    : qualifiedName
      SEMICOLON
    ;


/**
 * ============================================================================
 * RELATIONSHIP DECLARATION
 * ============================================================================
 *
 * There is ONE canonical topology relationship declaration:
 *
 *     connect A to B;
 *
 * Optional direction:
 *
 *     connect A to B directed;
 *     connect A to B undirected;
 *     connect A to B bidirectional;
 *
 * Optional condition:
 *
 *     connect A to B where condition;
 *
 * Optional properties:
 *
 *     connect A to B {
 *         latency = required_latency;
 *         bandwidth = required_bandwidth;
 *     };
 *
 * The grammar intentionally does NOT create separate semantic declarations
 * for:
 *
 *     edge
 *     link
 *     connection
 *
 * Those concepts can be represented through relationship properties or
 * semantic classification.
 *
 * ============================================================================
 */

topologyRelationshipDeclaration
    : CONNECT
      topologyEndpoint
      TO
      topologyEndpoint
      topologyRelationshipDirection?
      topologyRelationshipCondition?
      topologyRelationshipPropertyBlock?
      SEMICOLON
    ;


topologyRelationshipDirection
    : DIRECTED
    | UNDIRECTED
    | BIDIRECTIONAL
    ;


topologyRelationshipCondition
    : WHERE
      expression
    ;


/**
 * ============================================================================
 * ENDPOINT
 * ============================================================================
 *
 * An endpoint is a logical topology reference.
 *
 * Selectors are expressions.
 *
 * Examples:
 *
 *     compute
 *     compute[i]
 *     compute[worker]
 *     compute[workers]
 *
 * No physical index semantics are introduced here.
 * ============================================================================
 */

topologyEndpoint
    : qualifiedName
      topologyEndpointSelector*
    ;


topologyEndpointSelector
    : LBRACKET
      expression
      RBRACKET
    ;


/**
 * ============================================================================
 * RELATIONSHIP PROPERTY BLOCK
 * ============================================================================
 *
 * Properties are intentionally open-world.
 *
 * Examples:
 *
 *     latency
 *     bandwidth
 *     capacity
 *     distance
 *     reliability
 *     throughput
 *     cost
 *     weight
 *     protocol
 *     ordering
 *
 * These remain names unless separately reserved by the canonical language.
 *
 * No topology-specific property catalogue is hard-coded here.
 * ============================================================================
 */

topologyRelationshipPropertyBlock
    : LBRACE
      topologyPropertyStatement*
      RBRACE
    ;


/**
 * ============================================================================
 * PROPERTY DECLARATION
 * ============================================================================
 *
 * A topology property declaration establishes reusable topology metadata.
 *
 * It does not itself impose a physical requirement.
 * ============================================================================
 */

topologyPropertyDeclaration
    : PROPERTY
      qualifiedName
      topologyPropertyValueClause?
      SEMICOLON
    ;


topologyPropertyStatement
    : qualifiedName
      topologyPropertyValueClause
      SEMICOLON
    ;


topologyPropertyValueClause
    : ASSIGN
      expression
    ;


/**
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Generic semantic requirements are delegated to the canonical Requirements
 * grammar.
 *
 * Topology-specific connectivity requirements use the topology predicate:
 *
 *     requires connect(compute, memory);
 *
 * Generic requirements remain open-world:
 *
 *     requires quantum::measurement;
 *
 *     requires execution::deterministic;
 *
 * The parser does not decide satisfiability.
 * ============================================================================
 */

topologyRequirementDeclaration
    : REQUIRES
      (
          topologyConnectivityPredicate
        | requirementExpression
      )
      SEMICOLON
    ;


topologyConnectivityPredicate
    : CONNECT
      LPAREN
      topologyEndpoint
      COMMA
      topologyEndpoint
      RPAREN
    ;


/**
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints use the canonical expression system.
 *
 * Examples:
 *
 *     constraint latency <= latency_budget;
 *
 *     constraint available_bandwidth >= required_bandwidth;
 *
 *     constraint topology_valid;
 *
 * No topology-specific expression precedence is introduced.
 * ============================================================================
 */

topologyConstraintDeclaration
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/**
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * They MUST NOT be interpreted as mandatory requirements.
 *
 * Examples:
 *
 *     prefer locality;
 *
 *     prefer lower_latency;
 *
 *     prefer expression;
 *
 * ============================================================================
 */

topologyPreferenceDeclaration
    : PREFER
      expression
      SEMICOLON
    ;


/**
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser creates ANTLR contexts only.
 *
 * The frontend AST must preserve at minimum:
 *
 *     topology declaration name
 *     visibility
 *     modifiers
 *     extension list
 *     member ordering
 *
 *     node name
 *     node type
 *     node cardinality expression
 *     node properties
 *
 *     group name
 *     group type
 *     group cardinality expression
 *     group selector
 *     explicit members
 *
 *     relationship endpoints
 *     endpoint selectors
 *     relationship direction
 *     relationship condition
 *     relationship properties
 *
 *     topology requirements
 *     topology constraints
 *     topology preferences
 *
 *     source spans
 *
 * No physical resource identity is part of this AST contract.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST perform, as applicable:
 *
 *     name resolution
 *     duplicate declaration detection
 *     group/member validation
 *     endpoint resolution
 *     selector validation
 *     cardinality type validation
 *     direction validation
 *     property type validation
 *     unit validation
 *     requirement normalization
 *     constraint validation
 *     preference validation
 *     topology consistency checking
 *     topology satisfiability analysis
 *     target compatibility analysis
 *     resource feasibility analysis
 *     capability feasibility analysis
 *
 * None of those operations belong in this grammar.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * topology.g4 does not define:
 *
 *     resource
 *     resource quantity
 *     resource capacity
 *     resource budget
 *     resource negotiation
 *
 * Those belong to grammar/resources/.
 *
 * A topology property may refer to a resource-derived expression, but its
 * interpretation belongs to semantic/resource analysis.
 *
 * Example:
 *
 *     connect compute to memory {
 *         bandwidth = required_bandwidth;
 *     };
 *
 * `required_bandwidth` is a semantic value.
 *
 * The grammar does not determine whether any target provides it.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * topology.g4 does not define capability identity or capability semantics.
 *
 * Capability declarations and capability references remain owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Universal requirements remain owned by:
 *
 *     grammar/core/requirements.g4
 *
 * Hardware capability realization remains owned by the hardware semantic
 * layer.
 *
 * ============================================================================
 * PLACEMENT BOUNDARY
 * ============================================================================
 *
 * Topology != placement.
 *
 * Topology:
 *
 *     describes logical relationships.
 *
 * Placement:
 *
 *     maps logical entities toward realizable resources.
 *
 * topology.g4 MUST NOT introduce:
 *
 *     place(...)
 *     placement algorithms
 *     physical allocation
 *     device selection
 *     core selection
 *     GPU selection
 *     QPU selection
 *
 * ============================================================================
 * ROUTING BOUNDARY
 * ============================================================================
 *
 * Topology != routing.
 *
 * A relationship declaration such as:
 *
 *     connect source to destination;
 *
 * does NOT instruct the compiler to select a physical route.
 *
 * Routing occurs downstream.
 *
 * The topology model supplies constraints/relationships that routing MAY
 * consume.
 *
 * ============================================================================
 * SCHEDULING BOUNDARY
 * ============================================================================
 *
 * Topology != scheduling.
 *
 * This grammar contains no:
 *
 *     schedule
 *     timeline
 *     execution order
 *     clock assignment
 *     cycle assignment
 *
 * Scheduling remains downstream.
 *
 * ============================================================================
 * INTERCONNECT BOUNDARY
 * ============================================================================
 *
 * hardware/interconnect.g4 owns interconnect-specific source contracts.
 *
 * topology.g4 owns the abstract relationship graph.
 *
 * Therefore:
 *
 *     topology
 *          |
 *          v
 *     logical relationships
 *          |
 *          v
 *     interconnect semantic analysis
 *          |
 *          v
 *     routing / realization
 *
 * There must not be a second topology graph hidden inside interconnect.g4.
 *
 * ============================================================================
 * TARGET BOUNDARY
 * ============================================================================
 *
 * topology.g4 does not identify:
 *
 *     physical CPU
 *     physical GPU
 *     physical FPGA
 *     physical ASIC
 *     physical QPU
 *     physical accelerator
 *     physical memory bank
 *     PCI address
 *     serial number
 *     MAC address
 *     IP address
 *     physical qubit index
 *
 * Target and device realization belong downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Topology may describe relationships among logical quantum resources.
 *
 * It does NOT define:
 *
 *     quantum operations
 *     quantum gates
 *     qubit state
 *     measurement semantics
 *     physical qubit assignment
 *     SWAP insertion
 *     gate routing
 *     calibration
 *     QEC
 *     ZQN
 *
 * Quantum computation follows:
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
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 * topology.g4 does not replace quantum::ir.
 *
 * ============================================================================
 * HDL CONTRACT
 * ============================================================================
 *
 * Hardware topology may be consumed by HDL/hardware co-design.
 *
 * However this file does not define:
 *
 *     signals
 *     procedural behavior
 *     RTL processes
 *     synthesis
 *     timing implementation
 *     physical cells
 *     placement
 *     routing implementation
 *
 * Those remain owned by grammar/hdl/ and hardware semantic layers.
 *
 * ============================================================================
 * OPEN-WORLD TOPOLOGY CONTRACT
 * ============================================================================
 *
 * The grammar deliberately does NOT enumerate topology families.
 *
 * It does not define closed sets such as:
 *
 *     ring
 *     mesh
 *     torus
 *     tree
 *     star
 *     grid
 *     hypercube
 *     heavy_hex
 *     fat_tree
 *     butterfly
 *     crossbar
 *
 * New topology families are represented through:
 *
 *     qualified names
 *     properties
 *     relationships
 *     groups
 *     expressions
 *     dialects
 *
 * A future topology family therefore does not require modifying this grammar
 * merely because the family is new.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO language-level topology limits.
 *
 * In particular this file contains no:
 *
 *     MAX_NODES
 *     MAX_EDGES
 *     MAX_LINKS
 *     MAX_PORTS
 *     MAX_GROUPS
 *     MAX_ENDPOINTS
 *     MAX_DEVICES
 *     MAX_CONNECTIONS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *     MAX_TOPOLOGY_SIZE
 *
 * It also does not encode implicit universal limits such as:
 *
 *     node[32]
 *     group[64]
 *     topology<128>
 *
 * Numeric source values remain program semantics.
 *
 * A finite compiler/runtime/target capacity is an implementation or
 * realization fact, not a grammar limit.
 *
 * ============================================================================
 * SYMBOLIC SCALABILITY
 * ============================================================================
 *
 * These forms are intentionally representable:
 *
 *     node compute[workload.size];
 *
 *     group workers[available_workers];
 *
 *     connect compute[i] to memory[i];
 *
 *     connect compute[*] to storage[*];
 *
 *     requires connect(compute[*], storage[*]);
 *
 * The semantic layer determines whether selectors and cardinalities are
 * valid and realizable.
 *
 * The grammar does not assign a maximum.
 *
 * ============================================================================
 * ELASTICITY CONTRACT
 * ============================================================================
 *
 * A topology can describe:
 *
 *     fixed logical topology
 *     symbolic topology
 *     dynamically sized topology
 *     target-dependent topology
 *     runtime-dependent topology
 *
 * without changing the grammar.
 *
 * Whether a runtime may expand or contract a topology is a semantic,
 * execution, policy, and resource-management decision.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     token stream
 *     grammar version
 *     explicitly selected parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware discovery
 *     target discovery
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     device state
 *     resource availability
 *
 * Identical source and identical parser configuration must produce the same
 * parse structure and diagnostics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to this grammar.
 *
 * Examples:
 *
 *     missing topology name
 *     missing topology body
 *     malformed node
 *     malformed group
 *     malformed endpoint
 *     missing relationship endpoint
 *     malformed selector
 *     malformed property
 *     malformed requirement
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     duplicate node
 *     unknown endpoint
 *     invalid cardinality
 *     invalid property type
 *     contradictory topology
 *     unsatisfied resource requirement
 *     unavailable capability
 *     impossible realization
 *
 * A target having insufficient resources MUST NOT cause the parser to reject
 * syntactically valid topology source.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no executable code
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no runtime calls
 *     no randomness
 *
 * The Rust implementation consuming the generated parser MUST remain:
 *
 *     Rust 2021
 *     Rust 1.97+
 *     safe Rust
 *     no unsafe code
 *
 * ============================================================================
 * SOURCE PRESERVATION CONTRACT
 * ============================================================================
 *
 * Frontend AST construction must preserve:
 *
 *     source spans
 *     declaration ordering
 *     member ordering
 *     endpoint ordering
 *     selector expressions
 *     cardinality expressions
 *     relationship direction
 *     property ordering
 *     requirement structure
 *     constraint structure
 *     preference structure
 *
 * Semantic normalization occurs downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar owns NO IR.
 *
 * Topology semantic information may be lowered into:
 *
 *     canonical semantic representation
 *     classical representation
 *     quantum::ir metadata
 *     HDL/hardware semantic representation
 *     distributed representation
 *
 * depending on the consuming domain.
 *
 * Topology syntax itself never creates executable machine instructions.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Topology constructs must retain source provenance through the frontend AST.
 *
 * Downstream systems may attach:
 *
 *     derived_from
 *     transformed_by
 *     verified_by
 *     selected_by
 *     routed_by
 *     scheduled_by
 *
 * but those are semantic/compiler records and are not parser constructs.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain topology realization.
 *
 * Examples include:
 *
 *     security policy
 *     locality policy
 *     resource policy
 *     deployment policy
 *     resilience policy
 *     routing policy
 *
 * topology.g4 does not define policy semantics.
 *
 * A topology declaration never grants permission to access hardware.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Stable syntax owned here includes:
 *
 *     topology declaration
 *     topology visibility
 *     topology modifiers
 *     topology extension
 *     node syntax
 *     group syntax
 *     connect syntax
 *     endpoint selector syntax
 *     direction syntax
 *     property syntax
 *     topology requirement syntax
 *     topology constraint syntax
 *     topology preference syntax
 *
 * Any incompatible change MUST be reflected in:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *     grammar/grammar.md
 *
 * Historical material belongs in:
 *
 *     grammar/Zamani-Grammar.md
 *
 * and does not automatically become normative syntax.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * REQUIRED POSITIVE TESTS
 * -----------------------
 *
 *     topology Empty {}
 *
 *     topology Single {
 *         node compute;
 *     }
 *
 *     topology Scaled {
 *         node compute[workload.size];
 *         node memory[partition_count];
 *         connect compute[*] to memory[*];
 *     }
 *
 *     topology Directed {
 *         node source;
 *         node destination;
 *         connect source to destination directed;
 *     }
 *
 *     topology Undirected {
 *         node left;
 *         node right;
 *         connect left to right undirected;
 *     }
 *
 *     topology Bidirectional {
 *         node a;
 *         node b;
 *         connect a to b bidirectional;
 *     }
 *
 *     topology Properties {
 *         node compute;
 *         node memory;
 *
 *         connect compute to memory {
 *             latency = required_latency;
 *             bandwidth = required_bandwidth;
 *         };
 *     }
 *
 *     topology Requirements {
 *         node compute;
 *         node memory;
 *
 *         requires connect(compute, memory);
 *     }
 *
 *     topology GenericRequirements {
 *         requires quantum::measurement;
 *     }
 *
 *     topology Constraints {
 *         constraint latency <= latency_budget;
 *         prefer locality;
 *     }
 *
 * REQUIRED NEGATIVE TESTS
 * ----------------------
 *
 *     topology
 *
 *     topology {}
 *
 *     topology Invalid {
 *         node;
 *     }
 *
 *     topology Invalid {
 *         connect compute to;
 *     }
 *
 *     topology Invalid {
 *         node compute[;
 *     }
 *
 *     topology Invalid {
 *         node compute {
 *         };
 *     }
 *
 * REQUIRED BOUNDARY TESTS
 * ----------------------
 *
 *     symbolic cardinality
 *     symbolic endpoint selectors
 *     empty topology
 *     single-node topology
 *     large logical topology
 *     nested qualified names
 *     large property sets
 *     long expressions
 *     large declaration counts
 *
 * REQUIRED SCALABILITY TESTS
 * --------------------------
 *
 * The test suite must demonstrate that syntax remains valid as logical
 * topology size grows.
 *
 * Tests MUST NOT establish a universal maximum.
 *
 * Required categories:
 *
 *     tiny
 *     small
 *     medium
 *     large
 *     very large
 *     symbolic
 *     dynamic
 *     heterogeneous
 *     distributed
 *     quantum
 *     classical
 *     HDL
 *     hybrid
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Topology must be tested with:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware co-design
 *     distributed computation
 *     AI computation
 *     data computation
 *     networking
 *     accelerators
 *     memory
 *     security/resource policies
 *
 * These tests verify integration without moving domain semantics into this
 * grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE only when:
 *
 *     [ ] canonical lexer tokens exist exactly once;
 *     [ ] grammar composes with Hardware;
 *     [ ] grammar composes with ZamaniParser;
 *     [ ] canonical names are reused;
 *     [ ] canonical expressions are reused;
 *     [ ] canonical requirements are reused;
 *     [ ] no duplicate expression grammar exists;
 *     [ ] no duplicate type system exists;
 *     [ ] no duplicate resource system exists;
 *     [ ] no duplicate capability system exists;
 *     [ ] no physical identifiers exist;
 *     [ ] no fixed machine-size limits exist;
 *     [ ] symbolic cardinality works;
 *     [ ] symbolic selectors work;
 *     [ ] relationship direction is preserved;
 *     [ ] topology properties are open-world;
 *     [ ] requirements remain distinct from constraints;
 *     [ ] preferences remain advisory;
 *     [ ] topology is distinct from placement;
 *     [ ] topology is distinct from routing;
 *     [ ] topology is distinct from scheduling;
 *     [ ] topology is distinct from interconnect realization;
 *     [ ] quantum::ir remains the quantum semantic boundary;
 *     [ ] AST integration exists;
 *     [ ] semantic integration exists;
 *     [ ] provenance integration exists;
 *     [ ] policy integration exists;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] generated ANTLR parser compiles;
 *     [ ] Rust frontend compiles on Rust 1.97+;
 *     [ ] no unsafe Rust is required.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The fundamental topology rule is:
 *
 *     TOPOLOGY INTENT
 *          !=
 *     RESOURCE DISCOVERY
 *          !=
 *     RESOURCE ALLOCATION
 *          !=
 *     PLACEMENT
 *          !=
 *     ROUTING
 *          !=
 *     SCHEDULING
 *          !=
 *     HARDWARE
 *
 * The source describes logical relationships.
 *
 * The compiler and runtime determine how those relationships are realized.
 *
 * Therefore one topology description can remain semantically stable while
 * realization scales from tiny systems through heterogeneous machines,
 * accelerators, quantum systems, clusters, distributed systems, and future
 * computational substrates, subject only to actual resource availability and
 * semantic feasibility.
 *
 * ============================================================================
 */