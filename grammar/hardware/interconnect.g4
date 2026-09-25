/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/interconnect.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Grammar name:
 *     ZamaniHardwareInterconnectParser
 *
 * Status:
 *     PRODUCTION HARDWARE-CONTRACT LEAF GRAMMAR
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     This grammar contains no Rust actions, predicates, or executable code.
 *     The Rust implementation consuming the grammar MUST remain:
 *
 *         Rust 2021
 *         Rust 1.97 / 1.97.1
 *         safe Rust
 *         no unsafe code
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL SYNTAX for target-independent hardware
 * interconnect intent.
 *
 * An interconnect is the semantic communication/connectivity boundary between
 * abstract hardware entities.
 *
 * It can describe:
 *
 *     - interconnect declarations;
 *     - logical links;
 *     - endpoint sets;
 *     - endpoint references;
 *     - channels;
 *     - fabrics;
 *     - links between abstract resources;
 *     - directionality;
 *     - capacity;
 *     - bandwidth;
 *     - latency;
 *     - distance;
 *     - reliability;
 *     - protocol intent;
 *     - quality-of-service intent;
 *     - ordering requirements;
 *     - flow-control requirements;
 *     - connectivity requirements;
 *     - topology relationships;
 *     - resource requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - abstract routing domains;
 *     - logical mappings;
 *     - scalable endpoint groups;
 *     - parameterized interconnects;
 *     - extensible implementation properties.
 *
 * The grammar intentionally describes WHAT interconnect semantics are required
 * or declared.
 *
 * It does not decide WHICH physical interconnect realizes them.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
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
 *     Hardware dispatcher
 *          |
 *          v
 *     ZamaniHardwareInterconnectParser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> topology analysis
 *          +--> constraint analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> optimization
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          v
 *     Hardware HAL
 *          |
 *          v
 *     target realization
 *
 * This grammar is therefore strictly upstream of:
 *
 *     routing
 *     scheduling
 *     physical allocation
 *     device discovery
 *     device drivers
 *     calibration
 *     runtime execution
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - interconnect declarations;
 *     - logical links;
 *     - abstract channels;
 *     - interconnect endpoints;
 *     - endpoint groups;
 *     - interconnect direction;
 *     - interconnect semantic properties;
 *     - link-level requirements;
 *     - link-level constraints;
 *     - link-level preferences;
 *     - link-level capabilities;
 *     - abstract interconnect domains;
 *     - interconnect protocol intent;
 *     - bandwidth/latency/capacity intent;
 *     - logical connectivity declarations;
 *     - scalable interconnect composition;
 *     - parameterized interconnect contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - general expression syntax;
 *     - general type syntax;
 *     - hardware declarations as a whole;
 *     - generic hardware resources;
 *     - generic hardware capabilities;
 *     - generic hardware targets;
 *     - generic placement;
 *     - topology algorithms;
 *     - routing algorithms;
 *     - scheduling algorithms;
 *     - device discovery;
 *     - physical device enumeration;
 *     - physical addresses;
 *     - PCIe/InfiniBand/Ethernet/etc. implementation;
 *     - vendor SDKs;
 *     - network drivers;
 *     - DMA implementation;
 *     - cache-coherence implementation;
 *     - quantum gates;
 *     - quantum states;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - calibration;
 *     - runtime execution.
 *
 * ============================================================================
 * CRITICAL SEPARATION
 * ============================================================================
 *
 * Interconnect syntax is NOT physical routing.
 *
 * For example:
 *
 *     connect compute.host to accelerator[*];
 *
 * describes a logical relationship.
 *
 * It does NOT mean:
 *
 *     use PCIe;
 *     use link 0;
 *     use device 7;
 *     use physical address 0x...;
 *     use exactly N links;
 *     use exactly N ports;
 *     use a particular vendor;
 *     use a particular machine.
 *
 * Physical realization is selected later.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * This grammar supports POCO-REAF by allowing source programs to describe:
 *
 *     semantic communication requirements
 *     logical connectivity
 *     capabilities
 *     constraints
 *     preferences
 *     resource relationships
 *
 * without permanently binding the program to:
 *
 *     one CPU;
 *     one GPU;
 *     one FPGA;
 *     one ASIC;
 *     one QPU;
 *     one cluster;
 *     one network;
 *     one physical interconnect;
 *     one topology;
 *     one device identifier.
 *
 * The same source-level interconnect intent may therefore be lowered to:
 *
 *     on-chip interconnect
 *     memory fabric
 *     CPU fabric
 *     GPU fabric
 *     accelerator fabric
 *     FPGA routing fabric
 *     ASIC NoC
 *     host/device links
 *     cluster interconnect
 *     distributed communication
 *     quantum-device connectivity
 *     quantum-network connectivity
 *     future communication technologies
 *
 * when the target provides a compatible realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *     - interconnects;
 *     - fabrics;
 *     - links;
 *     - channels;
 *     - endpoints;
 *     - endpoint groups;
 *     - ports;
 *     - paths;
 *     - connections;
 *     - bandwidth;
 *     - capacity;
 *     - latency;
 *     - distance;
 *     - protocol count;
 *     - topology size;
 *     - node count;
 *     - device count;
 *     - resource count;
 *     - interconnect hierarchy depth.
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     expression-valued quantities
 *
 * rather than artificial finite bounds.
 *
 * "Infinity" means:
 *
 *     no artificial finite architectural ceiling is encoded here.
 *
 * Actual execution remains bounded by:
 *
 *     source size;
 *     compiler resources;
 *     target resources;
 *     runtime resources;
 *     physical resources;
 *     explicit security/resource policies.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce or imply:
 *
 *     MAX_INTERCONNECTS
 *     MAX_LINKS
 *     MAX_NETWORK_LINKS
 *     MAX_ENDPOINTS
 *     MAX_PORTS
 *     MAX_CHANNELS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_DISTANCE
 *     MAX_PATH_LENGTH
 *
 * It MUST NOT encode fixed physical resources such as:
 *
 *     link0
 *     link1
 *     port0
 *     port1
 *     device0
 *     device1
 *     node0
 *     node1
 *
 * as universal language concepts.
 *
 * A programmer MAY explicitly use an identifier or numeric value as program
 * semantics.
 *
 * For example:
 *
 *     endpoint accelerator_bank[17]
 *
 * is a program-level selection.
 *
 * The grammar must never reinterpret "17" as a universal hardware limit.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * The grammar MUST NOT enumerate every known interconnect technology.
 *
 * Do NOT create alternatives such as:
 *
 *     PCIe
 *     Ethernet
 *     InfiniBand
 *     NVLink
 *     CXL
 *     AXI
 *     NoC
 *     ...
 *
 * as the complete universe of interconnects.
 *
 * Technologies may instead be represented through:
 *
 *     qualified names;
 *     properties;
 *     capabilities;
 *     protocol references;
 *     dialects;
 *     target-specific semantic contracts.
 *
 * Examples:
 *
 *     interconnect.pcie
 *     interconnect.ethernet
 *     interconnect.infiniband
 *     interconnect.cxl
 *     interconnect.noc
 *     vendor.fabric
 *     future.communication.fabric
 *
 * These names remain semantic data rather than permanent parser keywords.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Interconnect syntax may describe connectivity required by quantum systems.
 *
 * Examples include:
 *
 *     quantum.control
 *     quantum.readout
 *     quantum.network
 *     quantum.device_connectivity
 *     quantum.logical_connectivity
 *
 * However, this grammar MUST NOT define:
 *
 *     - quantum gates;
 *     - quantum operations;
 *     - quantum states;
 *     - measurement semantics;
 *     - QEC algorithms;
 *     - ZQN semantics;
 *     - physical qubit allocation;
 *     - quantum routing algorithms.
 *
 * Quantum computation continues through:
 *
 *     quantum frontend
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     hardware realization
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Interconnect syntax may be referenced by HDL/hardware co-design.
 *
 * It does not replace:
 *
 *     grammar/hdl/
 *
 * HDL owns:
 *
 *     - behavioral hardware syntax;
 *     - signals;
 *     - nets;
 *     - sequential logic;
 *     - combinational logic;
 *     - timing constructs;
 *     - HDL interfaces;
 *     - synthesis/verification intent.
 *
 * Interconnect owns the abstract communication relationship.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Interconnect quantities are semantic resource expressions.
 *
 * Examples:
 *
 *     bandwidth >= required_bandwidth
 *     latency <= latency_budget
 *     capacity >= workload_size
 *
 * They do not allocate resources.
 *
 * Resource availability is resolved by the resource/capability subsystem.
 *
 * ============================================================================
 * TOPOLOGY BOUNDARY
 * ============================================================================
 *
 * Topology is the general relationship model.
 *
 * Interconnect is the communication/connectivity contract that can reference
 * topology relationships.
 *
 * Therefore:
 *
 *     topology
 *         owns graph/topology semantics;
 *
 *     interconnect
 *         owns communication-link semantics;
 *
 *     routing
 *         owns path selection;
 *
 *     scheduling
 *         owns temporal realization;
 *
 *     HAL
 *         owns physical target realization.
 *
 * This prevents interconnect.g4 from becoming a second topology grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar uses the canonical Zamani token vocabulary.
 *
 * The intended declaration is:
 *
 *     interconnect ...
 *
 * when the lexer provides an interconnect keyword.
 *
 * If the lexer does not yet provide a dedicated K_INTERCONNECT token, the
 * integration layer SHOULD use an existing extensibility mechanism or add the
 * token centrally in grammar/lexer/ rather than introducing a private lexer
 * inside this file.
 *
 * This file MUST NOT define lexer rules.
 *
 * General expressions and types are consumed through the canonical grammar
 * composition hierarchy.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareInterconnectParser;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public entry point owned by this leaf grammar.
 *
 * The Hardware composition grammar consumes this rule.
 */

hardwareInterconnectDeclaration
    : hardwareInterconnectAttributes*
      hardwareInterconnectVisibility?
      hardwareInterconnectModifier*
      K_INTERCONNECT
      IDENTIFIER
      hardwareInterconnectGenericParameters?
      hardwareInterconnectContractClauses?
      hardwareInterconnectBody
    ;


/* ============================================================================
 * 2. DECLARATION ATTRIBUTES
 * ============================================================================
 */

hardwareInterconnectAttributes
    : AT
      hardwareInterconnectQualifiedName
      (
          LPAREN
          hardwareInterconnectArgumentList?
          RPAREN
      )?
    ;

hardwareInterconnectArgumentList
    : hardwareInterconnectArgument
      (
          COMMA
          hardwareInterconnectArgument
      )*
      COMMA?
    ;

hardwareInterconnectArgument
    : hardwareInterconnectExpression
    | hardwareInterconnectQualifiedName
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | CHAR_LITERAL
    ;


/* ============================================================================
 * 3. VISIBILITY / MODIFIERS
 * ============================================================================
 *
 * These deliberately reuse the existing Zamani token vocabulary.
 */

hardwareInterconnectVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;

hardwareInterconnectModifier
    : K_STATIC
    | K_EXTERN
    | K_FINAL
    | K_ABSTRACT
    | K_SEALED
    | K_PARTIAL
    | K_VIRTUAL
    ;


/* ============================================================================
 * 4. GENERIC PARAMETERS
 * ============================================================================
 *
 * Interconnects are parameterizable.
 *
 * No physical dimension is fixed here.
 *
 * Example:
 *
 *     interconnect Fabric<Width, Lanes, Capacity> { ... }
 *
 * Width, lanes and capacity remain semantic parameters.
 */

hardwareInterconnectGenericParameters
    : LT
      hardwareInterconnectGenericParameter
      (
          COMMA
          hardwareInterconnectGenericParameter
      )*
      COMMA?
      GT
    ;

hardwareInterconnectGenericParameter
    : IDENTIFIER
      (
          COLON
          hardwareInterconnectGenericBound
      )?
      (
          ASSIGN
          hardwareInterconnectExpression
      )?
    ;

hardwareInterconnectGenericBound
    : hardwareInterconnectQualifiedName
    | hardwareInterconnectCapabilityReference
    ;


/* ============================================================================
 * 5. CONTRACT CLAUSES
 * ============================================================================
 */

hardwareInterconnectContractClauses
    : hardwareInterconnectContractClause+
    ;

hardwareInterconnectContractClause
    : hardwareInterconnectRequiresClause
    | hardwareInterconnectProvidesClause
    | hardwareInterconnectConstraintClause
    | hardwareInterconnectPreferenceClause
    ;


/* ============================================================================
 * 6. INTERCONNECT BODY
 * ============================================================================
 */

hardwareInterconnectBody
    : LBRACE
      hardwareInterconnectItem*
      RBRACE
    ;

hardwareInterconnectItem
    : hardwareInterconnectAttributes*
      (
          hardwareInterconnectInterfaceDeclaration
        | hardwareInterconnectEndpointDeclaration
        | hardwareInterconnectEndpointGroupDeclaration
        | hardwareInterconnectLinkDeclaration
        | hardwareInterconnectChannelDeclaration
        | hardwareInterconnectFabricDeclaration
        | hardwareInterconnectProtocolDeclaration
        | hardwareInterconnectResourceDeclaration
        | hardwareInterconnectCapabilityDeclaration
        | hardwareInterconnectRequirementDeclaration
        | hardwareInterconnectConstraintDeclaration
        | hardwareInterconnectPreferenceDeclaration
        | hardwareInterconnectTopologyReference
        | hardwareInterconnectPlacementReference
        | hardwareInterconnectMappingDeclaration
        | hardwareInterconnectPropertyDeclaration
        | hardwareInterconnectAssertion
      )
    ;


/* ============================================================================
 * 7. INTERFACES
 * ============================================================================
 *
 * This is an abstract interconnect interface, not an HDL interface.
 */

hardwareInterconnectInterfaceDeclaration
    : K_INTERFACE
      IDENTIFIER
      hardwareInterconnectGenericParameters?
      hardwareInterconnectInterfaceInheritance?
      LBRACE
      hardwareInterconnectInterfaceMember*
      RBRACE
    ;

hardwareInterconnectInterfaceInheritance
    : K_EXTENDS
      hardwareInterconnectQualifiedName
      (
          COMMA
          hardwareInterconnectQualifiedName
      )*
    ;

hardwareInterconnectInterfaceMember
    : hardwareInterconnectEndpointDeclaration
    | hardwareInterconnectChannelDeclaration
    | hardwareInterconnectProtocolDeclaration
    | hardwareInterconnectCapabilityDeclaration
    | hardwareInterconnectPropertyDeclaration
    ;


/* ============================================================================
 * 8. ENDPOINTS
 * ============================================================================
 *
 * Endpoints are logical.
 *
 * They may represent:
 *
 *     compute resources;
 *     memories;
 *     accelerators;
 *     devices;
 *     modules;
 *     services;
 *     quantum resources;
 *     HDL interfaces;
 *     distributed resources;
 *     future computational entities.
 *
 * They do not identify physical addresses.
 */

hardwareInterconnectEndpointDeclaration
    : K_ENDPOINT
      IDENTIFIER
      hardwareInterconnectEndpointTypeClause?
      hardwareInterconnectEndpointMultiplicityClause?
      hardwareInterconnectEndpointPropertyBlock?
      SEMICOLON
    ;

hardwareInterconnectEndpointTypeClause
    : COLON
      hardwareInterconnectQualifiedName
    ;

hardwareInterconnectEndpointMultiplicityClause
    : LBRACKET
      hardwareInterconnectExpression
      RBRACKET
    ;

hardwareInterconnectEndpointPropertyBlock
    : LBRACE
      hardwareInterconnectPropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 9. ENDPOINT GROUPS
 * ============================================================================
 *
 * Groups are important for scalability.
 *
 * The grammar never expands a group into a fixed number of endpoints.
 *
 * Example:
 *
 *     endpoint_group accelerators
 *         where capability.accelerator;
 *
 * The actual number of members is resolved semantically.
 */

hardwareInterconnectEndpointGroupDeclaration
    : K_GROUP
      IDENTIFIER
      hardwareInterconnectGroupTypeClause?
      hardwareInterconnectGroupSelectorClause?
      hardwareInterconnectGroupMultiplicityClause?
      hardwareInterconnectGroupBody?
      SEMICOLON?
    ;

hardwareInterconnectGroupTypeClause
    : COLON
      hardwareInterconnectQualifiedName
    ;

hardwareInterconnectGroupSelectorClause
    : K_WHERE
      hardwareInterconnectExpression
    ;

hardwareInterconnectGroupMultiplicityClause
    : LBRACKET
      hardwareInterconnectExpression
      RBRACKET
    ;

hardwareInterconnectGroupBody
    : LBRACE
      hardwareInterconnectGroupMember*
      RBRACE
    ;

hardwareInterconnectGroupMember
    : hardwareInterconnectEndpointReference
      (
          COMMA
          hardwareInterconnectEndpointReference
      )*
      SEMICOLON
    ;


/* ============================================================================
 * 10. LINKS
 * ============================================================================
 *
 * A link expresses a logical communication relationship.
 *
 * It is not a physical cable or a selected hardware route.
 */

hardwareInterconnectLinkDeclaration
    : K_LINK
      hardwareInterconnectLinkName?
      hardwareInterconnectLinkEndpoints
      hardwareInterconnectDirectionClause?
      hardwareInterconnectLinkPropertyBlock?
      SEMICOLON
    ;

hardwareInterconnectLinkName
    : hardwareInterconnectQualifiedName
    ;

hardwareInterconnectLinkEndpoints
    : hardwareInterconnectEndpointReference
      K_TO
      hardwareInterconnectEndpointReference
    ;

hardwareInterconnectDirectionClause
    : K_BIDIRECTIONAL
    | K_DIRECTED
    | K_UNDIRECTED
    ;


/* ============================================================================
 * 11. CHANNELS
 * ============================================================================
 *
 * Channels describe logical communication channels.
 *
 * They are not operating-system sockets, physical lanes, or driver objects.
 */

hardwareInterconnectChannelDeclaration
    : K_CHANNEL
      IDENTIFIER
      hardwareInterconnectChannelEndpoints?
      hardwareInterconnectChannelPropertyBlock?
      SEMICOLON
    ;

hardwareInterconnectChannelEndpoints
    : hardwareInterconnectEndpointReference
      K_TO
      hardwareInterconnectEndpointReference
    ;


/* ============================================================================
 * 12. FABRICS
 * ============================================================================
 *
 * A fabric is a reusable abstract communication domain.
 *
 * It can contain links, channels, endpoints and properties without imposing
 * a fixed implementation technology.
 */

hardwareInterconnectFabricDeclaration
    : K_FABRIC
      IDENTIFIER
      hardwareInterconnectGenericParameters?
      hardwareInterconnectFabricPropertyBlock?
      SEMICOLON
    ;

hardwareInterconnectFabricPropertyBlock
    : LBRACE
      hardwareInterconnectFabricItem*
      RBRACE
    ;

hardwareInterconnectFabricItem
    : hardwareInterconnectEndpointReference
    | hardwareInterconnectLinkReference
    | hardwareInterconnectChannelReference
    | hardwareInterconnectProtocolReference
    | hardwareInterconnectPropertyStatement
    ;


/* ============================================================================
 * 13. PROTOCOLS
 * ============================================================================
 *
 * Protocol names are open-world qualified names.
 *
 * The grammar does not enumerate Ethernet, PCIe, CXL, RDMA, etc.
 */

hardwareInterconnectProtocolDeclaration
    : K_PROTOCOL
      hardwareInterconnectQualifiedName
      hardwareInterconnectProtocolPropertyBlock?
      SEMICOLON
    ;

hardwareInterconnectProtocolPropertyBlock
    : LBRACE
      hardwareInterconnectPropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 14. RESOURCES
 * ============================================================================
 *
 * Resource values are expressions.
 *
 * Examples:
 *
 *     bandwidth >= required_bandwidth;
 *     capacity >= workload_size;
 *     latency <= latency_budget;
 *
 * No machine-size maximum is represented here.
 */

hardwareInterconnectResourceDeclaration
    : K_RESOURCE
      IDENTIFIER
      hardwareInterconnectResourceTypeClause?
      hardwareInterconnectResourceQuantityClause?
      hardwareInterconnectResourcePropertyBlock?
      SEMICOLON
    ;

hardwareInterconnectResourceTypeClause
    : COLON
      hardwareInterconnectQualifiedName
    ;

hardwareInterconnectResourceQuantityClause
    : hardwareInterconnectRelationOperator
      hardwareInterconnectExpression
    ;

hardwareInterconnectResourcePropertyBlock
    : LBRACE
      hardwareInterconnectPropertyStatement*
      RBRACE
    ;


/* ============================================================================
 * 15. CAPABILITIES
 * ============================================================================
 */

hardwareInterconnectCapabilityDeclaration
    : K_CAPABILITY
      hardwareInterconnectCapabilityReference
      hardwareInterconnectCapabilityValue?
      SEMICOLON
    ;

hardwareInterconnectCapabilityReference
    : hardwareInterconnectQualifiedName
    ;

hardwareInterconnectCapabilityValue
    : ASSIGN
      hardwareInterconnectExpression
    ;


/* ============================================================================
 * 16. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 */

hardwareInterconnectRequirementDeclaration
    : K_REQUIRES
      hardwareInterconnectRequirementExpression
      SEMICOLON
    ;

hardwareInterconnectRequiresClause
    : K_REQUIRES
      hardwareInterconnectRequirementExpression
      SEMICOLON
    ;

hardwareInterconnectRequirementExpression
    : hardwareInterconnectRequirementOperand
      (
          hardwareInterconnectLogicalOperator
          hardwareInterconnectRequirementOperand
      )*
    ;

hardwareInterconnectRequirementOperand
    : hardwareInterconnectCapabilityReference
    | hardwareInterconnectResourceReference
    | hardwareInterconnectComparison
    | hardwareInterconnectQualifiedName
    | LPAREN
      hardwareInterconnectRequirementExpression
      RPAREN
    ;


/* ============================================================================
 * 17. PROVIDES
 * ============================================================================
 */

hardwareInterconnectProvidesClause
    : K_PROVIDES
      hardwareInterconnectProvidesExpression
      SEMICOLON
    ;

hardwareInterconnectProvidesExpression
    : hardwareInterconnectCapabilityReference
    | hardwareInterconnectResourceReference
    | hardwareInterconnectQualifiedName
    ;


/* ============================================================================
 * 18. CONSTRAINTS
 * ============================================================================
 */

hardwareInterconnectConstraintDeclaration
    : K_CONSTRAINT
      hardwareInterconnectConstraintExpression
      SEMICOLON
    ;

hardwareInterconnectConstraintClause
    : K_CONSTRAINT
      hardwareInterconnectConstraintExpression
      SEMICOLON
    ;

hardwareInterconnectConstraintExpression
    : hardwareInterconnectComparison
    | hardwareInterconnectQualifiedName
    | LPAREN
      hardwareInterconnectConstraintExpression
      RPAREN
    ;


/* ============================================================================
 * 19. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-semantic implementation guidance.
 */

hardwareInterconnectPreferenceDeclaration
    : K_PREFER
      hardwareInterconnectPreferenceExpression
      SEMICOLON
    ;

hardwareInterconnectPreferenceClause
    : K_PREFER
      hardwareInterconnectPreferenceExpression
      SEMICOLON
    ;

hardwareInterconnectPreferenceExpression
    : hardwareInterconnectComparison
    | hardwareInterconnectQualifiedName
    | hardwareInterconnectExpression
    ;


/* ============================================================================
 * 20. TOPOLOGY REFERENCE
 * ============================================================================
 *
 * This references topology semantics without recreating topology.g4.
 *
 * Example:
 *
 *     topology = application.mesh;
 *
 * The actual topology declaration remains owned by topology.g4.
 */

hardwareInterconnectTopologyReference
    : K_TOPOLOGY
      ASSIGN
      hardwareInterconnectQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 21. PLACEMENT REFERENCE
 * ============================================================================
 *
 * Placement remains intent.
 *
 * Actual placement algorithms remain downstream.
 */

hardwareInterconnectPlacementReference
    : K_PLACEMENT
      ASSIGN
      hardwareInterconnectQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 22. MAPPING
 * ============================================================================
 *
 * Maps abstract communication entities.
 *
 * This is NOT physical allocation.
 */

hardwareInterconnectMappingDeclaration
    : K_MAP
      hardwareInterconnectQualifiedName
      K_TO
      hardwareInterconnectQualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 23. LINK / CHANNEL / ENDPOINT REFERENCES
 * ============================================================================
 */

hardwareInterconnectEndpointReference
    : hardwareInterconnectQualifiedName
      hardwareInterconnectEndpointSelector?
    ;

hardwareInterconnectEndpointSelector
    : LBRACKET
      hardwareInterconnectExpression
      RBRACKET
    ;

hardwareInterconnectLinkReference
    : hardwareInterconnectQualifiedName
    ;

hardwareInterconnectChannelReference
    : hardwareInterconnectQualifiedName
    ;

hardwareInterconnectProtocolReference
    : hardwareInterconnectQualifiedName
    ;


/* ============================================================================
 * 24. LINK PROPERTIES
 * ============================================================================
 *
 * Properties intentionally remain extensible.
 *
 * Known semantic properties are recognized explicitly where they already have
 * canonical Zamani vocabulary. Future/vendor-specific properties remain
 * qualified names.
 */

hardwareInterconnectLinkPropertyBlock
    : LBRACE
      hardwareInterconnectLinkProperty*
      RBRACE
    ;

hardwareInterconnectLinkProperty
    : K_BANDWIDTH
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_CAPACITY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_DISTANCE
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_PROTOCOL
      ASSIGN
      hardwareInterconnectQualifiedName
      SEMICOLON

    | K_RELIABILITY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_PRIORITY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_ORDER
      ASSIGN
      hardwareInterconnectQualifiedName
      SEMICOLON

    | K_FLOW
      ASSIGN
      hardwareInterconnectQualifiedName
      SEMICOLON

    | hardwareInterconnectPropertyStatement
    ;


/* ============================================================================
 * 25. CHANNEL PROPERTIES
 * ============================================================================
 */

hardwareInterconnectChannelPropertyBlock
    : LBRACE
      hardwareInterconnectChannelProperty*
      RBRACE
    ;

hardwareInterconnectChannelProperty
    : K_BANDWIDTH
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_LATENCY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_CAPACITY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_PROTOCOL
      ASSIGN
      hardwareInterconnectQualifiedName
      SEMICOLON

    | K_RELIABILITY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | K_PRIORITY
      ASSIGN
      hardwareInterconnectExpression
      SEMICOLON

    | hardwareInterconnectPropertyStatement
    ;


/* ============================================================================
 * 26. GENERIC PROPERTY STATEMENTS
 * ============================================================================
 *
 * Property names are open-world qualified names.
 *
 * This prevents every future technology from becoming a language keyword.
 */

hardwareInterconnectPropertyStatement
    : hardwareInterconnectQualifiedName
      (
          ASSIGN
          hardwareInterconnectExpression
      )?
      SEMICOLON
    ;

hardwareInterconnectPropertyDeclaration
    : K_PROPERTY
      hardwareInterconnectQualifiedName
      (
          ASSIGN
          hardwareInterconnectExpression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 27. ASSERTIONS
 * ============================================================================
 *
 * Assertions describe source-level contracts.
 *
 * They do not execute during parsing.
 */

hardwareInterconnectAssertion
    : K_ASSERT
      hardwareInterconnectExpression
      SEMICOLON
    ;


/* ============================================================================
 * 28. EXPRESSIONS
 * ============================================================================
 *
 * The interconnect grammar needs expression-shaped syntax but must not create
 * a second expression language.
 *
 * These rules are deliberately limited adapters around the canonical Zamani
 * expression vocabulary available through the parser composition layer.
 *
 * When the canonical expression grammar is imported by the Hardware
 * composition grammar, these adapter names MUST be bound to that canonical
 * expression representation rather than implemented as a competing parser.
 *
 * The fallback structural expression below exists only to keep this leaf
 * grammar independently composable.
 */


/* ---------------------------------------------------------------------------
 * Qualified names
 * ---------------------------------------------------------------------------
 *
 * Qualified names are open-world.
 *
 * Examples:
 *
 *     compute.host
 *     accelerator.tensor
 *     quantum.control
 *     vendor.fabric.link
 */

hardwareInterconnectQualifiedName
    : IDENTIFIER
      (
          DOT
          IDENTIFIER
      )*
    ;


/* ---------------------------------------------------------------------------
 * Resource references
 * ---------------------------------------------------------------------------
 */

hardwareInterconnectResourceReference
    : hardwareInterconnectQualifiedName
    ;


/* ---------------------------------------------------------------------------
 * Comparisons
 * ---------------------------------------------------------------------------
 */

hardwareInterconnectComparison
    : hardwareInterconnectExpression
      hardwareInterconnectRelationOperator
      hardwareInterconnectExpression
    ;


/* ---------------------------------------------------------------------------
 * Relation operators
 * ---------------------------------------------------------------------------
 */

hardwareInterconnectRelationOperator
    : LT
    | LE
    | GT
    | GE
    | EQ
    | NE
    | ASSIGN
    ;


/* ---------------------------------------------------------------------------
 * Logical operators
 * ---------------------------------------------------------------------------
 */

hardwareInterconnectLogicalOperator
    : K_AND
    | K_OR
    ;


/* ---------------------------------------------------------------------------
 * Expression adapter
 * ---------------------------------------------------------------------------
 *
 * The expression surface is intentionally open-ended.
 *
 * It supports:
 *
 *     identifiers;
 *     qualified names;
 *     literals;
 *     indexing;
 *     calls;
 *     arithmetic;
 *     comparisons;
 *     grouping.
 *
 * The canonical Zamani expression grammar remains authoritative for full
 * expression semantics.
 */

hardwareInterconnectExpression
    : hardwareInterconnectLogicalExpression
    ;

hardwareInterconnectLogicalExpression
    : hardwareInterconnectComparisonExpression
      (
          hardwareInterconnectLogicalOperator
          hardwareInterconnectComparisonExpression
      )*
    ;

hardwareInterconnectComparisonExpression
    : hardwareInterconnectAdditiveExpression
      (
          hardwareInterconnectRelationOperator
          hardwareInterconnectAdditiveExpression
      )?
    ;

hardwareInterconnectAdditiveExpression
    : hardwareInterconnectMultiplicativeExpression
      (
          (PLUS | MINUS)
          hardwareInterconnectMultiplicativeExpression
      )*
    ;

hardwareInterconnectMultiplicativeExpression
    : hardwareInterconnectUnaryExpression
      (
          (STAR | SLASH | PERCENT)
          hardwareInterconnectUnaryExpression
      )*
    ;

hardwareInterconnectUnaryExpression
    : (PLUS | MINUS | NOT)?
      hardwareInterconnectPrimaryExpression
    ;

hardwareInterconnectPrimaryExpression
    : hardwareInterconnectQualifiedName
      hardwareInterconnectPostfix*
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | K_TRUE
    | K_FALSE
    | LPAREN
      hardwareInterconnectExpression
      RPAREN
    ;

hardwareInterconnectPostfix
    : LBRACKET
      hardwareInterconnectExpression
      RBRACKET
    | LPAREN
      hardwareInterconnectCallArgumentList?
      RPAREN
    ;

hardwareInterconnectCallArgumentList
    : hardwareInterconnectExpression
      (
          COMMA
          hardwareInterconnectExpression
      )*
      COMMA?
    ;


/* ============================================================================
 * 29. DESIGN INVARIANTS
 * ============================================================================
 *
 * The following invariants are normative:
 *
 * 1. This grammar contains no lexer rules.
 *
 * 2. This grammar contains no embedded Rust.
 *
 * 3. This grammar contains no semantic predicates.
 *
 * 4. This grammar contains no physical-device discovery.
 *
 * 5. This grammar contains no runtime execution.
 *
 * 6. This grammar contains no routing algorithm.
 *
 * 7. This grammar contains no scheduling algorithm.
 *
 * 8. This grammar contains no physical allocation algorithm.
 *
 * 9. This grammar contains no vendor-specific mandatory keyword universe.
 *
 * 10. This grammar contains no artificial finite machine-size limits.
 *
 * 11. This grammar does not create quantum::ir.
 *
 * 12. This grammar does not create a hardware IR.
 *
 * 13. This grammar does not create a second topology IR.
 *
 * 14. All physical realization is downstream.
 *
 * 15. Requirements, constraints, preferences and capabilities remain
 *     semantically distinct.
 *
 * 16. Generic parameters are source semantics, not implementation limits.
 *
 * 17. Endpoint groups are resolved semantically and are never expanded by
 *     the parser into physical resources.
 *
 * 18. Qualified names provide the extension mechanism for future
 *     interconnect technologies.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser layer must preserve enough information for the domain-neutral AST
 * to represent:
 *
 *     InterconnectDeclaration
 *     InterconnectEndpoint
 *     InterconnectEndpointGroup
 *     InterconnectLink
 *     InterconnectChannel
 *     InterconnectFabric
 *     InterconnectProtocolReference
 *     InterconnectResourceRequirement
 *     InterconnectCapability
 *     InterconnectConstraint
 *     InterconnectPreference
 *     InterconnectTopologyReference
 *     InterconnectPlacementReference
 *     InterconnectMapping
 *     InterconnectProperty
 *     InterconnectAssertion
 *
 * Every AST node must preserve source spans.
 *
 * The AST must not contain physical device assumptions merely because the
 * syntax contains an abstract endpoint.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must distinguish:
 *
 *     endpoint
 *     endpoint group
 *     logical link
 *     logical channel
 *     fabric
 *     capability
 *     resource
 *     requirement
 *     constraint
 *     preference
 *     mapping
 *
 * It must validate:
 *
 *     - endpoint references;
 *     - duplicate declarations;
 *     - incompatible endpoint types;
 *     - invalid requirements;
 *     - invalid resource expressions;
 *     - invalid capabilities;
 *     - invalid topology references;
 *     - invalid mappings;
 *     - contradictory constraints;
 *     - invalid directionality;
 *     - type compatibility.
 *
 * It must NOT select physical hardware during parsing.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not define a new IR.
 *
 * Semantic lowering should feed the repository's canonical hardware/resource
 * representation.
 *
 * Interconnect semantics may subsequently be consumed by:
 *
 *     topology analysis
 *     resource analysis
 *     placement
 *     routing
 *     scheduling
 *     resilience
 *     HAL
 *
 * Quantum programs continue to use:
 *
 *     quantum::ir
 *
 * and this grammar must never introduce another quantum IR.
 *
 * ============================================================================
 * ROUTING CONTRACT
 * ============================================================================
 *
 * The source may say:
 *
 *     connect source to destination;
 *
 * or:
 *
 *     requires bandwidth >= required_bandwidth;
 *
 * but the grammar MUST NOT decide:
 *
 *     path = source -> link3 -> link7 -> destination
 *
 * Physical/logical route selection belongs to routing.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * The grammar may express:
 *
 *     latency;
 *     ordering;
 *     priority;
 *     timing-related properties;
 *
 * but must not construct an execution schedule.
 *
 * Scheduling belongs downstream.
 *
 * ============================================================================
 * HARDWARE HAL CONTRACT
 * ============================================================================
 *
 * The HAL resolves semantic interconnect requirements against actual target
 * capabilities.
 *
 * Examples:
 *
 *     abstract bandwidth requirement
 *          |
 *          v
 *     target capability
 *          |
 *          v
 *     physical link/fabric
 *
 * The grammar does not know the physical link.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is a new leaf grammar.
 *
 * It MUST NOT rename or delete:
 *
 *     grammar/hardware/hardware.g4
 *     grammar/hardware/topology.g4
 *     grammar/hardware/resources.g4
 *     grammar/hardware/capabilities.g4
 *     grammar/hardware/placement.g4
 *
 * Existing generic hardware connection syntax remains compatible.
 *
 * The interconnect grammar introduces a distinct semantic declaration:
 *
 *     hardwareInterconnectDeclaration
 *
 * rather than silently replacing:
 *
 *     hardwareConnectionDeclaration
 *
 * Existing source forms continue to be owned by their existing contracts until
 * an explicit compatibility migration is approved.
 *
 * ============================================================================
 * CONFORMANCE TEST REQUIREMENTS
 * ============================================================================
 *
 * Positive cases MUST include:
 *
 *     interconnect Fabric {
 *         endpoint host;
 *         endpoint accelerator;
 *         link host to accelerator;
 *     }
 *
 *     interconnect Fabric {
 *         endpoint_group accelerators [n];
 *         link host to accelerators[*];
 *     }
 *
 *     interconnect Fabric {
 *         endpoint source;
 *         endpoint destination;
 *         link path source to destination bidirectional {
 *             bandwidth = required_bandwidth;
 *             latency = latency_budget;
 *         }
 *     }
 *
 *     interconnect Fabric {
 *         requires capability.quantum.control;
 *         requires resource.bandwidth >= required_bandwidth;
 *         topology = application.topology;
 *         placement = execution.preference;
 *     }
 *
 *     interconnect Fabric<Width, Lanes, Capacity> {
 *         endpoint source;
 *         endpoint destination;
 *         link source to destination {
 *             bandwidth = Width * Lanes;
 *             capacity = Capacity;
 *         }
 *     }
 *
 * Negative cases MUST include:
 *
 *     - missing endpoint;
 *     - malformed link;
 *     - malformed direction;
 *     - malformed comparison;
 *     - duplicate endpoint declarations;
 *     - invalid empty link endpoint;
 *     - invalid property assignment;
 *     - invalid generic parameter syntax.
 *
 * Scalability cases MUST include:
 *
 *     - one endpoint;
 *     - many endpoints;
 *     - endpoint groups;
 *     - symbolic multiplicity;
 *     - parameterized capacity;
 *     - parameterized bandwidth;
 *     - nested fabrics;
 *     - arbitrarily large logical topology descriptions.
 *
 * Hard-coding tests MUST reject architectural additions such as:
 *
 *     MAX_INTERCONNECTS
 *     MAX_LINKS
 *     MAX_ENDPOINTS
 *     MAX_PORTS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * when they are introduced as language limits.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 *     [x] ownership is explicit;
 *     [x] lexer ownership is external;
 *     [x] no Rust actions exist;
 *     [x] no unsafe requirement exists;
 *     [x] no artificial capacity limits exist;
 *     [x] links are logical;
 *     [x] endpoints are abstract;
 *     [x] endpoint groups are scalable;
 *     [x] directionality is extensible;
 *     [x] bandwidth is expression-valued;
 *     [x] latency is expression-valued;
 *     [x] capacity is expression-valued;
 *     [x] protocol names are open-world;
 *     [x] topology remains separately owned;
 *     [x] placement remains separately owned;
 *     [x] routing remains downstream;
 *     [x] scheduling remains downstream;
 *     [x] HAL remains downstream;
 *     [x] quantum::ir remains canonical;
 *     [x] requirements are separate from preferences;
 *     [x] capabilities are separate from resources;
 *     [x] mappings do not imply physical allocation;
 *     [x] AST contract is defined;
 *     [x] semantic contract is defined;
 *     [x] IR contract is defined;
 *     [x] compatibility contract is defined;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] scalability tests are defined;
 *     [x] hard-coding audit is defined.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */