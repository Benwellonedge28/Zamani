/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/networking/service-discovery.g4
 *
 * Grammar:
 *     NetworkingServiceDiscovery
 *
 * Status:
 *     Production networking service-discovery parser grammar
 *
 * Purpose:
 *     Define target-independent SOURCE-LEVEL SERVICE-DISCOVERY INTENT.
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust.
 *     - No unsafe implementation requirement.
 *     - No parser actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No environment inspection.
 *     - No randomness.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the syntax for declaring logical service-discovery intent.
 *
 * Discovery describes WHAT logical service/resource relationship a program
 * needs. It does not perform discovery and does not select a physical
 * realization.
 *
 * OWNED
 * -----
 *
 * This file owns:
 *
 *     networkServiceDiscoveryConstruct
 *     networkServiceDiscoveryDeclaration
 *     networkServiceDiscoveryBody
 *     networkServiceDiscoveryMember
 *     networkServiceDiscoveryProperty
 *     networkServiceDiscoveryPropertyName
 *     networkServiceDiscoveryPropertyValue
 *     networkServiceDiscoveryNestedBlock
 *     networkServiceDiscoveryNestedBlockName
 *     networkServiceDiscoveryNestedMember
 *     networkServiceDiscoveryReference
 *     networkServiceDiscoveryReferenceList
 *     optionalNetworkServiceDiscoveryReferenceList
 *     networkServiceDiscoveryDeclarationList
 *     networkServiceDiscoveryPropertyList
 *     networkServiceDiscoveryItem
 *
 *     networkServiceDiscoveryServiceReference
 *     networkServiceDiscoveryEndpointReference
 *     networkServiceDiscoveryAddressReference
 *     networkServiceDiscoveryProtocolReference
 *     networkServiceDiscoveryChannelReference
 *     networkServiceDiscoveryRouteReference
 *     networkServiceDiscoveryRequestReference
 *     networkServiceDiscoveryResponseReference
 *     networkServiceDiscoverySocketReference
 *     networkServiceDiscoveryCapabilityReference
 *
 *     networkServiceDiscoverySelector
 *     networkServiceDiscoveryQuery
 *     networkServiceDiscoveryPolicyBlock
 *     networkServiceDiscoveryConstraint
 *     networkServiceDiscoveryRequirement
 *     networkServiceDiscoveryPreference
 *     networkServiceDiscoveryCapability
 *     networkServiceDiscoveryMetadata
 *
 * DOES NOT OWN
 * ------------
 *
 * This file does NOT own:
 *
 *     identifiers
 *     qualified names
 *     attributes
 *     expressions
 *     types
 *     requirements
 *     resource expressions
 *     capability declarations
 *     constraints
 *     preferences
 *     policies
 *     effects
 *     contracts
 *     services
 *     endpoints
 *     addresses
 *     protocols
 *     channels
 *     messages
 *     requests
 *     responses
 *     sockets
 *     streams
 *     routes
 *     network capability declarations
 *     distributed placement
 *     topology
 *     scheduling
 *     routing algorithms
 *     registry implementation
 *     DNS
 *     service-registry protocols
 *     authentication
 *     authorization
 *     cryptography
 *     hardware discovery
 *     target selection
 *     resource allocation
 *     quantum operations
 *     quantum topology
 *     QEC
 *     classical IR
 *     quantum::ir
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/core/names.g4
 *     grammar/expressions/
 *     grammar/core/attributes.g4
 *
 * IMPORTS:
 *
 *     Names
 *     Expressions
 *     Attributes
 *
 * EXPORTS:
 *
 *     networkServiceDiscoveryConstruct
 *     networkServiceDiscoveryDeclaration
 *     networkServiceDiscoveryReference
 *     networkServiceDiscoveryReferenceList
 *     networkServiceDiscoveryServiceReference
 *     networkServiceDiscoveryEndpointReference
 *     networkServiceDiscoveryAddressReference
 *     networkServiceDiscoveryProtocolReference
 *     networkServiceDiscoveryChannelReference
 *     networkServiceDiscoveryRouteReference
 *     networkServiceDiscoveryRequestReference
 *     networkServiceDiscoveryResponseReference
 *     networkServiceDiscoverySocketReference
 *     networkServiceDiscoveryCapabilityReference
 *     networkServiceDiscoverySelector
 *     networkServiceDiscoveryQuery
 *     networkServiceDiscoveryPolicyBlock
 *
 * CONSUMED_BY:
 *
 *     grammar/networking/networking.g4
 *     grammar/antlr/ZamaniParser.g4
 *     networking AST construction
 *     networking semantic analysis
 *     networking validation
 *     networking tooling
 *     networking conformance tests
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST subsystem
 *
 * SEMANTIC_OWNER:
 *
 *     networking service-discovery semantic subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic representation and downstream domain IR
 *
 * TEST_OWNER:
 *
 *     grammar/tests/networking/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/networking.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/spec/contracts.md
 *     grammar/spec/policies.md
 *     grammar/spec/provenance.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniLexer
 *   |
 *   v
 * NetworkingServiceDiscovery
 *   |
 *   v
 * domain-neutral AST
 *   |
 *   +--> name resolution
 *   +--> type analysis
 *   +--> effect analysis
 *   +--> capability analysis
 *   +--> resource analysis
 *   +--> contract validation
 *   +--> policy validation
 *   +--> provenance
 *   |
 *   v
 * networking semantic model
 *   |
 *   v
 * canonical semantic representation
 *   |
 *   +--> routing
 *   +--> placement
 *   +--> scheduling
 *   +--> resilience
 *   +--> deployment
 *   |
 *   v
 * runtime / ZQN / HAL
 *
 * This grammar stops at the source syntax / parse-tree boundary.
 *
 * ============================================================================
 * DISCOVERY MEANING
 * ============================================================================
 *
 * A declaration such as:
 *
 *     discover compute;
 *
 * means:
 *
 *     declare logical discovery intent named `compute`.
 *
 * It does NOT mean:
 *
 *     locate a physical machine;
 *     enumerate devices;
 *     query DNS;
 *     query a registry;
 *     open a socket;
 *     select an endpoint;
 *     select a route;
 *     allocate resources;
 *     contact a service.
 *
 * Those decisions are downstream.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Property names are qualified names and property values are expressions.
 *
 * Therefore new discovery concepts do not require modifying this grammar.
 *
 * Examples:
 *
 *     service: compute;
 *     endpoint: compute::worker;
 *     protocol: network::reliable;
 *     selector: capability("compute.general");
 *     requires: capability("network.discovery");
 *     constraint: latency <= required_latency;
 *     prefers: locality::near;
 *     policy: execution::adaptive;
 *
 * The parser does not assign semantic meaning to the property name.
 *
 * Semantic analysis decides whether a property is:
 *
 *     standard;
 *     dialect-defined;
 *     capability-defined;
 *     policy-defined;
 *     application-defined;
 *     invalid.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Discovery must preserve source portability.
 *
 * A program describes logical requirements rather than permanently binding
 * itself to a particular physical realization.
 *
 * The same discovery intent may therefore participate in realization on:
 *
 *     embedded systems
 *     CPU systems
 *     multicore systems
 *     GPU systems
 *     FPGA systems
 *     ASIC systems
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * provided the semantic requirements and capabilities can be satisfied.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO grammar-level finite limit on:
 *
 *     discovery declarations
 *     properties
 *     nested blocks
 *     references
 *     qualified-name depth
 *     expression size
 *     declaration composition
 *     semantic discovery candidates
 *
 * Lists use `*` or `+`.
 *
 * The grammar MUST NOT introduce artificial capacity constants for:
 *
 *     services
 *     endpoints
 *     nodes
 *     devices
 *     routes
 *     registries
 *     results
 *     connections
 *     network size
 *     memory
 *     bandwidth
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *
 * Physical and implementation limits belong downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Discovery properties may carry requirements such as:
 *
 *     requires: capability("network.discovery");
 *     requires: capability("network.reliable");
 *     requires: capability("quantum.compute");
 *     requires: capability("gpu.compute");
 *     requires: memory >= required_memory;
 *     constraint: latency <= required_latency;
 *
 * These remain expressions at the parser boundary.
 *
 * Resource and capability satisfaction occurs downstream.
 *
 * This grammar never:
 *
 *     discovers resources;
 *     allocates resources;
 *     reserves resources;
 *     selects hardware;
 *     fixes resource capacities.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A discovery declaration may semantically imply effects such as:
 *
 *     network
 *     io
 *     distributed
 *     foreign
 *     native
 *     randomness
 *     simulation
 *
 * The grammar does not create a second effect system.
 *
 * Effect classification belongs to the existing effects subsystem.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Discovery properties may reference:
 *
 *     requirements
 *     constraints
 *     preferences
 *     capabilities
 *     policies
 *     contracts
 *     evidence
 *     provenance
 *
 * The discovery grammar does not redefine those universal systems.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source structure must remain traceable through:
 *
 *     source
 *       ->
 *     parse tree
 *       ->
 *     AST
 *       ->
 *     semantic discovery model
 *       ->
 *     canonical semantic representation
 *
 * Source spans and declaration/member ordering are preserved by the normal
 * ANTLR parse-tree pipeline.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Discovery may identify a logical quantum service:
 *
 *     discover quantum_backend {
 *         service: quantum::backend;
 *         requires: capability("quantum.compute");
 *     }
 *
 * This grammar does NOT define:
 *
 *     qubits
 *     gates
 *     circuits
 *     measurements
 *     pulses
 *     calibration
 *     QPU topology
 *     QEC
 *
 * Quantum semantics remain downstream and, where applicable, lower through:
 *
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * No discovery-specific quantum IR is introduced.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Discovery may identify logical hardware services:
 *
 *     discover accelerator {
 *         service: hardware::accelerator;
 *         requires: capability("accelerator.compute");
 *     }
 *
 * It does not define:
 *
 *     register widths
 *     physical wires
 *     FPGA dimensions
 *     ASIC dimensions
 *     device counts
 *     memory capacities
 *     physical topology
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * NETWORKING COMPONENT BOUNDARIES
 * ============================================================================
 *
 * services.g4
 *     owns service contracts.
 *
 * endpoints.g4
 *     owns endpoint declarations.
 *
 * addresses.g4
 *     owns address syntax.
 *
 * protocols.g4
 *     owns protocol contracts.
 *
 * channels.g4
 *     owns network channel contracts.
 *
 * messages.g4
 *     owns message schemas.
 *
 * requests.g4
 *     owns reusable requests.
 *
 * responses.g4
 *     owns reusable responses.
 *
 * sockets.g4
 *     owns socket contracts.
 *
 * streams.g4 / streaming grammar
 *     owns stream contracts.
 *
 * routing.g4
 *     owns route contracts.
 *
 * network-capabilities.g4
 *     owns network capability declarations.
 *
 * This file references those concepts using qualified names and expressions.
 * It does not duplicate their syntax.
 *
 * ============================================================================
 * PARSER
 * ============================================================================
 */

parser grammar NetworkingServiceDiscovery;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions,
    Attributes
    ;


/*
 * ============================================================================
 * PUBLIC NETWORKING COMPOSITION ENTRY POINT
 * ============================================================================
 *
 * Consumed by:
 *
 *     grammar/networking/networking.g4
 *
 * This rule deliberately accepts declarations only.
 *
 * A bare qualified-name reference is NOT accepted as a top-level networking
 * construct. This prevents discovery references from stealing ordinary
 * networking constructs during aggregate parsing.
 * ============================================================================
 */

networkServiceDiscoveryConstruct
    : networkServiceDiscoveryDeclaration
    ;


/*
 * ============================================================================
 * DISCOVERY DECLARATION
 * ============================================================================
 *
 * Supported forms:
 *
 *     discover compute;
 *
 *     discover compute {
 *         service: compute;
 *     }
 *
 *     discover quantum::backend {
 *         service: quantum::backend;
 *         requires: capability("quantum.compute");
 *     };
 *
 * The declaration name is a logical discovery-contract identity.
 *
 * It is not inherently:
 *
 *     service identity
 *     endpoint identity
 *     process identity
 *     node identity
 *     device identity
 *     hardware identity
 *
 * Semantic analysis determines those relationships.
 * ============================================================================
 */

networkServiceDiscoveryDeclaration
    : attribute*
      DISCOVER
      networkServiceDiscoveryName
      networkServiceDiscoveryBody?
      SEMI?
    ;


/*
 * ============================================================================
 * DISCOVERY NAME
 * ============================================================================
 *
 * Qualified names are used instead of a single identifier so discovery
 * contracts can participate in module/domain namespaces without imposing
 * artificial naming structures.
 * ============================================================================
 */

networkServiceDiscoveryName
    : qualifiedName
    ;


/*
 * ============================================================================
 * DISCOVERY BODY
 * ============================================================================
 */

networkServiceDiscoveryBody
    : LBRACE
      networkServiceDiscoveryMember*
      RBRACE
    ;


/*
 * ============================================================================
 * DISCOVERY MEMBER
 * ============================================================================
 *
 * Every member may have canonical attributes.
 *
 * The member itself is either:
 *
 *     property
 *
 * or:
 *
 *     nested configuration block.
 * ============================================================================
 */

networkServiceDiscoveryMember
    : attribute*
      (
          networkServiceDiscoveryProperty
        | networkServiceDiscoveryNestedBlock
      )
    ;


/*
 * ============================================================================
 * DISCOVERY PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     qualifiedName : expression ;
 *
 * The value is deliberately an expression so the discovery grammar does not
 * need to enumerate:
 *
 *     transports
 *     providers
 *     registries
 *     protocols
 *     service kinds
 *     hardware types
 *     target types
 *     future discovery mechanisms
 * ============================================================================
 */

networkServiceDiscoveryProperty
    : networkServiceDiscoveryPropertyName
      COLON
      networkServiceDiscoveryPropertyValue
      SEMI
    ;


networkServiceDiscoveryPropertyName
    : qualifiedName
    ;


networkServiceDiscoveryPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * NESTED DISCOVERY BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     discover compute {
 *         selector {
 *             capability: compute::general;
 *             locality: locality::near;
 *         }
 *
 *         policy {
 *             strategy: execution::adaptive;
 *         }
 *     }
 *
 * Nested block names are semantic names, not a closed keyword vocabulary.
 * ============================================================================
 */

networkServiceDiscoveryNestedBlock
    : attribute*
      networkServiceDiscoveryNestedBlockName
      LBRACE
      networkServiceDiscoveryNestedMember*
      RBRACE
      SEMI?
    ;


networkServiceDiscoveryNestedBlockName
    : qualifiedName
    ;


networkServiceDiscoveryNestedMember
    : attribute*
      (
          networkServiceDiscoveryProperty
        | networkServiceDiscoveryNestedBlock
      )
    ;


/*
 * ============================================================================
 * LOGICAL DISCOVERY REFERENCE
 * ============================================================================
 *
 * A reference is a source-level logical name.
 *
 * It performs no lookup.
 * ============================================================================
 */

networkServiceDiscoveryReference
    : qualifiedName
    ;


networkServiceDiscoveryReferenceList
    : networkServiceDiscoveryReference
      (
          COMMA
          networkServiceDiscoveryReference
      )*
      COMMA?
    ;


optionalNetworkServiceDiscoveryReferenceList
    : networkServiceDiscoveryReferenceList?
    ;


/*
 * ============================================================================
 * DECLARATION / PROPERTY COLLECTIONS
 * ============================================================================
 */

networkServiceDiscoveryDeclarationList
    : networkServiceDiscoveryDeclaration*
    ;


networkServiceDiscoveryPropertyList
    : networkServiceDiscoveryProperty*
    ;


/*
 * ============================================================================
 * REUSABLE DISCOVERY ITEM
 * ============================================================================
 *
 * This rule is intentionally NOT used by the top-level networking dispatcher.
 * It exists for downstream grammar components that need a typed discovery
 * item boundary.
 * ============================================================================
 */

networkServiceDiscoveryItem
    : networkServiceDiscoveryDeclaration
    | networkServiceDiscoveryReference
    ;


/*
 * ============================================================================
 * TYPED LOGICAL REFERENCES
 * ============================================================================
 *
 * These rules establish semantic boundaries without importing the grammar that
 * owns the referenced declaration.
 *
 * This avoids dependency cycles between networking component grammars.
 * ============================================================================
 */

networkServiceDiscoveryServiceReference
    : qualifiedName
    ;


networkServiceDiscoveryEndpointReference
    : qualifiedName
    ;


networkServiceDiscoveryAddressReference
    : qualifiedName
    ;


networkServiceDiscoveryProtocolReference
    : qualifiedName
    ;


networkServiceDiscoveryChannelReference
    : qualifiedName
    ;


networkServiceDiscoveryRouteReference
    : qualifiedName
    ;


networkServiceDiscoveryRequestReference
    : qualifiedName
    ;


networkServiceDiscoveryResponseReference
    : qualifiedName
    ;


networkServiceDiscoverySocketReference
    : qualifiedName
    ;


networkServiceDiscoveryCapabilityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SELECTOR / QUERY
 * ============================================================================
 *
 * Both remain canonical expression boundaries.
 *
 * The grammar deliberately does not introduce a registry-specific query
 * language.
 * ============================================================================
 */

networkServiceDiscoverySelector
    : expression
    ;


networkServiceDiscoveryQuery
    : expression
    ;


/*
 * ============================================================================
 * POLICY / REQUIREMENT / CONSTRAINT / PREFERENCE / CAPABILITY / METADATA
 * ============================================================================
 *
 * These are structural adapters.
 *
 * They do not create competing policy, requirement, resource, capability,
 * contract, or metadata systems.
 * ============================================================================
 */

networkServiceDiscoveryPolicyBlock
    : networkServiceDiscoveryNestedBlock
    ;


networkServiceDiscoveryConstraint
    : networkServiceDiscoveryProperty
    ;


networkServiceDiscoveryRequirement
    : networkServiceDiscoveryProperty
    ;


networkServiceDiscoveryPreference
    : networkServiceDiscoveryProperty
    ;


networkServiceDiscoveryCapability
    : networkServiceDiscoveryProperty
    ;


networkServiceDiscoveryMetadata
    : networkServiceDiscoveryProperty
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral AST must preserve:
 *
 *     declaration attributes
 *     declaration name
 *     declaration body presence
 *     ordered members
 *     property names
 *     property values
 *     nested blocks
 *     nested member ordering
 *     source spans
 *
 * Recommended semantic shape:
 *
 *     DiscoveryDeclaration
 *       - name
 *       - attributes
 *       - members
 *
 *     DiscoveryMember
 *       - Property
 *       - NestedBlock
 *
 *     DiscoveryProperty
 *       - name
 *       - value
 *
 * The grammar does not require a discovery-specific IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     service resolution
 *     endpoint resolution
 *     address resolution
 *     protocol compatibility
 *     channel compatibility
 *     request compatibility
 *     response compatibility
 *     route compatibility
 *     socket compatibility
 *     capability satisfaction
 *     resource requirement satisfaction
 *     constraint validation
 *     preference interpretation
 *     policy validation
 *     version compatibility
 *     availability interpretation
 *     health interpretation
 *     freshness interpretation
 *     consistency interpretation
 *     security interpretation
 *     distributed compatibility
 *     portability analysis
 *
 * Parser acceptance MUST NOT imply:
 *
 *     service exists
 *     endpoint exists
 *     registry exists
 *     network exists
 *     capability is available
 *     resource is available
 *     route is feasible
 *     target is executable
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * If semantic analysis determines that a discovery construct participates in
 * network or external I/O, the existing effects subsystem records that fact.
 *
 * This grammar does not declare a competing effect taxonomy.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Expressions may contain resource and capability requirements:
 *
 *     requires: capability("network.discovery");
 *     requires: capability("network.reliable");
 *     requires: capability("quantum.compute");
 *     requires: capability("gpu.compute");
 *     requires: memory >= required_memory;
 *     constraint: latency <= required_latency;
 *
 * Resolution occurs through:
 *
 *     resource analysis
 *         ->
 *     capability analysis
 *         ->
 *     negotiation
 *         ->
 *     execution planning
 *
 * No physical resource is selected by this grammar.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Discovery may reference policy intent:
 *
 *     policy: execution::adaptive;
 *
 * or:
 *
 *     policy {
 *         strategy: execution::adaptive;
 *         fallback: simulation;
 *     }
 *
 * Policy interpretation belongs to the shared policy subsystem.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Discovery declarations may contain provenance properties through the open
 * property model.
 *
 * The semantic subsystem is responsible for constructing the actual
 * provenance record.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It must not introduce:
 *
 *     DiscoveryIR
 *     ServiceDiscoveryIR
 *     NetworkDiscoveryIR
 *     PhysicalDiscoveryIR
 *
 * Discovery semantics feed the canonical semantic representation and whatever
 * domain IR is appropriate downstream.
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If a discovered service participates in quantum computation:
 *
 *     discovery
 *       ->
 *     networking semantics
 *       ->
 *     hybrid/quantum semantics
 *       ->
 *     quantum::ir
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Discovery may identify logical hardware or accelerator services.
 *
 * Physical realization is handled after:
 *
 *     semantic analysis
 *       ->
 *     capability/resource negotiation
 *       ->
 *     placement
 *       ->
 *     scheduling
 *       ->
 *     target realization
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may perform:
 *
 *     registry lookup
 *     endpoint resolution
 *     discovery refresh
 *     health observation
 *     failover
 *     cache management
 *     connection establishment
 *     route activation
 *     service binding
 *
 * None of those operations occur in the grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     imported grammar definitions
 *     parser configuration
 *
 * Parsing does not depend on:
 *
 *     registry state
 *     DNS
 *     network state
 *     hardware
 *     runtime state
 *     filesystem state
 *     environment state
 *     time
 *     randomness
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - handles no secrets;
 *     - performs no authentication;
 *     - performs no authorization;
 *     - performs no encryption;
 *     - performs no certificate validation;
 *     - performs no network I/O.
 *
 * Security semantics remain downstream.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must cover:
 *
 *     missing discover keyword
 *     missing discovery name
 *     malformed qualified name
 *     malformed discovery body
 *     malformed property
 *     missing colon
 *     missing expression
 *     missing semicolon
 *     malformed nested block
 *     malformed attribute
 *
 * Semantic diagnostics must cover:
 *
 *     duplicate discovery declaration
 *     unresolved service
 *     unresolved endpoint
 *     unresolved address
 *     unresolved protocol
 *     unresolved channel
 *     unresolved route
 *     unresolved request/response
 *     unresolved capability
 *     unsatisfied requirement
 *     contradictory constraint
 *     incompatible policy
 *     unavailable realization
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * discover compute;
 *
 * discover compute_service {
 *     service: compute;
 * }
 *
 * discover compute_service {
 *     service: compute;
 *     endpoint: compute::worker;
 *     protocol: network::reliable;
 * }
 *
 * discover quantum_backend {
 *     service: quantum::backend;
 *     requires: capability("quantum.compute");
 * }
 *
 * discover accelerator {
 *     service: hardware::accelerator;
 *     requires: capability("gpu.compute");
 * }
 *
 * discover compute {
 *     requires: memory >= required_memory;
 *     constraint: latency <= required_latency;
 *     prefers: locality::near;
 * }
 *
 * discover compute {
 *     selector {
 *         capability: compute::general;
 *         locality: locality::near;
 *     }
 *
 *     policy {
 *         strategy: execution::adaptive;
 *         fallback: simulation;
 *     }
 * }
 *
 * discover compute {
 *     vendor::discovery::policy: future_policy;
 *     application::metadata: metadata_value;
 * }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected syntactically:
 *
 *     discover;
 *
 *     discover 123;
 *
 *     discover compute {
 *         service
 *     }
 *
 *     discover compute {
 *         service:
 *     }
 *
 *     discover compute {
 *         service: ;
 *     }
 *
 *     discover compute {
 *         service compute;
 *     }
 *
 *     discover compute {
 *         : compute;
 *     }
 *
 *     discover compute {
 *         selector {
 *     }
 *
 *     discover compute {
 *         service: compute
 *         endpoint: worker;
 *     }
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Required boundaries:
 *
 *     discover a;
 *     discover a::b;
 *     discover a::b::c;
 *
 *     discover compute {};
 *
 *     discover compute {
 *         service: compute;
 *     };
 *
 *     deeply qualified discovery names
 *     deeply qualified property names
 *     deeply nested configuration
 *     large expressions
 *     large property collections
 *     large declaration collections
 *
 * No finite grammar-level boundary is permitted.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Discovery must remain syntactically usable for:
 *
 *     classical services
 *     quantum services
 *     hybrid services
 *     AI services
 *     tensor services
 *     accelerator services
 *     HDL/hardware services
 *     distributed services
 *     simulation services
 *     future domains
 *
 * Domain interpretation is semantic.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test generators must exercise increasing:
 *
 *     declaration counts
 *     property counts
 *     nested-block counts
 *     reference counts
 *     expression sizes
 *     qualified-name depths
 *
 * The tests must not turn a test fixture size into a language-level maximum.
 *
 * ============================================================================
 * PORTABILITY TEST CONTRACT
 * ============================================================================
 *
 * The same source discovery declaration must remain semantically portable
 * across targets when requirements/capabilities permit:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     heterogeneous
 *     future target
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing networking composition currently imports this grammar through:
 *
 *     NetworkingServiceDiscovery
 *
 * and dispatches:
 *
 *     networkingServiceDiscovery
 *         ->
 *     networkServiceDiscoveryConstruct
 *
 * Those public rules MUST remain stable.
 *
 * The canonical declaration remains:
 *
 *     discover qualified::name;
 *
 * or:
 *
 *     discover qualified::name {
 *         ...
 *     }
 *
 * Existing logical discovery references remain reusable through the exported
 * reference rules.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO:
 *
 *     MAX_SERVICES
 *     MAX_ENDPOINTS
 *     MAX_DISCOVERIES
 *     MAX_INSTANCES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_ROUTERS
 *     MAX_SWITCHES
 *     MAX_INTERFACES
 *     MAX_LINKS
 *     MAX_NETWORK_SIZE
 *     MAX_RESULTS
 *     MAX_RETRIES
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_THREADS
 *
 * It contains no physical-resource enumeration.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * networking.g4
 * -------------
 *
 * Already imports:
 *
 *     NetworkingServiceDiscovery
 *
 * and exposes:
 *
 *     networkingServiceDiscovery
 *
 * which delegates to:
 *
 *     networkServiceDiscoveryConstruct
 *
 * Therefore no change to the networking aggregate is required merely to
 * replace this file.
 *
 * ZamaniParser.g4
 * ---------------
 *
 * The root parser continues consuming the Networking aggregate.
 *
 * It must not import this leaf grammar directly.
 *
 * Lexer
 * -----
 *
 * `DISCOVER` is already a canonical lexical token:
 *
 *     DISCOVER : 'discover'
 *
 * It is owned by the central lexical vocabulary.
 *
 * No duplicate lexer rule must be added here.
 *
 * Names
 * -----
 *
 * `qualifiedName` remains owned by the canonical Names grammar.
 *
 * Expressions
 * -----------
 *
 * `expression` remains owned by the canonical Expressions grammar.
 *
 * Attributes
 * ----------
 *
 * `attribute` remains owned by the canonical Attributes grammar.
 *
 * Services
 * --------
 *
 * Discovery references services through logical qualified names.
 *
 * Endpoints
 * ---------
 *
 * Discovery references endpoints through logical qualified names.
 *
 * Protocols
 * ---------
 *
 * Discovery references protocols through logical qualified names.
 *
 * Channels
 * --------
 *
 * Discovery references channels through logical qualified names.
 *
 * Routes
 * ------
 *
 * Discovery references routes through logical qualified names.
 *
 * Resources / Capabilities
 * ------------------------
 *
 * Resource and capability expressions are preserved as ordinary expressions
 * and interpreted by their canonical semantic subsystems.
 *
 * Security
 * --------
 *
 * Security properties remain semantic data and are interpreted by the
 * security subsystem.
 *
 * Distributed
 * -----------
 *
 * Discovery does not own node membership, cluster management, replication,
 * consensus, placement, or distributed scheduling.
 *
 * Runtime
 * -------
 *
 * Runtime owns actual discovery mechanisms and service realization.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] One service-discovery grammar authority exists.
 * [x] The aggregate networking grammar imports this authority.
 * [x] The root parser reaches discovery only through Networking.
 * [x] The canonical DISCOVER token is reused.
 * [x] Qualified discovery names are supported.
 * [x] Optional declaration bodies are supported.
 * [x] Attributes are supported.
 * [x] Open-world property names are supported.
 * [x] Expression-valued properties are supported.
 * [x] Nested configuration is supported.
 * [x] Logical references are supported.
 * [x] Reference lists are unbounded at the grammar level.
 * [x] No physical discovery is performed by parsing.
 * [x] No transport is enumerated.
 * [x] No registry implementation is embedded.
 * [x] No routing implementation is embedded.
 * [x] No scheduler is embedded.
 * [x] No hardware assumptions are embedded.
 * [x] No resource ceilings are embedded.
 * [x] No second policy system is embedded.
 * [x] No second capability system is embedded.
 * [x] No second effect system is embedded.
 * [x] No second IR is introduced.
 * [x] Quantum participation remains compatible with quantum::ir.
 * [x] HDL/hardware participation remains target-independent.
 * [x] Classical, AI, distributed and hybrid services remain expressible.
 * [x] The grammar contains no Rust actions.
 * [x] The implementation requires no unsafe Rust.
 * [x] Positive tests exist.
 * [x] Negative tests exist.
 * [x] Boundary tests exist.
 * [x] Scalability tests exist.
 * [x] Portability tests exist.
 * [x] Determinism tests exist.
 * [x] Compatibility tests exist.
 *
 * BUILD VERIFICATION
 * ------------------
 *
 * [ ] ANTLR generation succeeds.
 * [ ] All imports resolve.
 * [ ] No duplicate rule names exist.
 * [ ] No undefined token references exist.
 * [ ] No ambiguity warnings remain.
 * [ ] Networking aggregate generation succeeds.
 * [ ] ZamaniParser generation succeeds.
 * [ ] Rust 1.97+ generated-parser integration succeeds.
 * [ ] Positive tests pass.
 * [ ] Negative tests pass.
 * [ ] Boundary tests pass.
 * [ ] Scalability tests pass.
 * [ ] Portability tests pass.
 * [ ] Determinism tests pass.
 * [ ] Compatibility tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar expresses:
 *
 *     LOGICAL DISCOVERY INTENT
 *
 * It does not express:
 *
 *     PHYSICAL DISCOVERY REALIZATION
 *
 * Therefore:
 *
 *     discover
 *        |
 *        v
 *     logical discovery intent
 *        |
 *        v
 *     AST
 *        |
 *        v
 *     semantic validation
 *        |
 *        +--> requirements
 *        +--> capabilities
 *        +--> resources
 *        +--> constraints
 *        +--> policies
 *        +--> effects
 *        +--> provenance
 *        |
 *        v
 *     target-independent planning
 *        |
 *        v
 *     runtime discovery / routing / placement / scheduling
 *        |
 *        v
 *     target realization
 *
 * This preserves the POCO-REAF boundary and allows networking discovery to
 * scale from the smallest useful deployment to arbitrarily large systems,
 * subject only to actual semantic feasibility and available resources.
 *
 * ============================================================================
 */