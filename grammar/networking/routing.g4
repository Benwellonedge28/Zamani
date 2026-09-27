/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/routing.g4
 *
 * Grammar:
 *     NetworkingRoutes
 *
 * Status:
 *     Production parser grammar contract
 *
 * Purpose:
 *     Define target-independent SOURCE-LEVEL ROUTING INTENT.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Routing in Zamani is a semantic intent, constraint, policy, or logical
 * relationship. It is NOT a physical routing implementation.
 *
 * This grammar describes what routing-related properties a program declares.
 *
 * It does NOT perform:
 *
 *     - route discovery;
 *     - shortest-path calculation;
 *     - topology discovery;
 *     - network probing;
 *     - packet forwarding;
 *     - interface selection;
 *     - router selection;
 *     - physical path allocation;
 *     - bandwidth allocation;
 *     - latency measurement;
 *     - congestion control;
 *     - scheduling;
 *     - placement;
 *     - deployment;
 *     - socket creation;
 *     - transport creation;
 *     - hardware discovery;
 *     - runtime execution.
 *
 * The architectural pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> networking analysis
 *          +--> distributed analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     routing analysis
 *          |
 *          v
 *     route realization
 *          |
 *          v
 *     scheduling / placement / deployment
 *          |
 *          v
 *     runtime / hardware
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical route declarations;
 *     - route declaration identity;
 *     - route contract bodies;
 *     - route properties;
 *     - route nested configuration;
 *     - route references;
 *     - routing intent;
 *     - routing constraints expressed syntactically;
 *     - routing requirements expressed syntactically;
 *     - routing preferences expressed syntactically;
 *     - routing capabilities expressed syntactically;
 *     - routing policy metadata;
 *     - stable routing parser entry points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - endpoints;
 *     - addresses;
 *     - protocols;
 *     - channels;
 *     - sockets;
 *     - messages;
 *     - services;
 *     - service discovery;
 *     - topology discovery;
 *     - distributed node membership;
 *     - distributed placement;
 *     - concurrency;
 *     - scheduling;
 *     - resource allocation;
 *     - hardware topology;
 *     - network interfaces;
 *     - routers;
 *     - switches;
 *     - packet forwarding;
 *     - transport implementations;
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - HTTP;
 *     - MPI;
 *     - RDMA;
 *     - cloud-provider networking;
 *     - vendor networking APIs;
 *     - security implementation;
 *     - cryptographic implementation;
 *     - quantum operations;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - runtime behavior.
 *
 * ============================================================================
 * RELATIONSHIP TO OTHER NETWORKING GRAMMARS
 * ============================================================================
 *
 *     endpoints.g4
 *         owns logical communication participants.
 *
 *     addresses.g4
 *         owns address-related syntax.
 *
 *     protocols.g4
 *         owns protocol declarations and protocol intent.
 *
 *     channels.g4
 *         owns logical networking channels.
 *
 *     messages.g4
 *         owns message schemas and values.
 *
 *     services.g4
 *         owns service contracts.
 *
 *     sockets.g4
 *         owns logical socket contracts.
 *
 *     requests.g4
 *         owns reusable request contracts.
 *
 *     responses.g4
 *         owns reusable response contracts.
 *
 *     network-capabilities.g4
 *         owns networking capability declarations.
 *
 *     routing.g4
 *         owns ROUTING INTENT.
 *
 * Routing may reference the concepts above through ordinary names and
 * expressions, but it must not duplicate their grammar.
 *
 * ============================================================================
 * ROUTING VS ROUTE REALIZATION
 * ============================================================================
 *
 * A source declaration such as:
 *
 *     route compute_path {
 *         source: client;
 *         destination: compute;
 *         policy: adaptive;
 *     }
 *
 * describes a logical routing contract.
 *
 * It does NOT mean:
 *
 *     use router 3;
 *     use interface eth0;
 *     use node 7;
 *     use link 12;
 *     use physical_path [ ... ];
 *
 * unless such information is explicitly part of a separately defined
 * target-specific semantic contract.
 *
 * Physical route realization belongs downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Routing must preserve:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * A route declaration therefore describes logical requirements and policies,
 * not today's physical network.
 *
 * A route may eventually be realized through:
 *
 *     - local execution;
 *     - shared memory;
 *     - IPC;
 *     - local networking;
 *     - data-center networking;
 *     - HPC interconnects;
 *     - distributed fabrics;
 *     - accelerator fabrics;
 *     - cloud networking;
 *     - edge networking;
 *     - quantum networking;
 *     - future communication substrates.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     Ethernet
 *     Wi-Fi
 *     5G
 *     6G
 *
 * as routing-specific grammar alternatives.
 *
 * Such concepts remain names, expressions, protocol declarations, capabilities,
 * dialects, or semantic registry entries.
 *
 * A future routing technology therefore does not require a new parser keyword
 * merely because its name did not exist when Zamani was designed.
 *
 * ============================================================================
 * OPEN-WORLD PROPERTY MODEL
 * ============================================================================
 *
 * Route properties intentionally use:
 *
 *     qualifiedName : expression ;
 *
 * This provides one structural syntax for:
 *
 *     source
 *     destination
 *     via
 *     policy
 *     metric
 *     cost
 *     latency
 *     bandwidth
 *     reliability
 *     ordering
 *     locality
 *     security
 *     requires
 *     constraint
 *     prefers
 *     capability
 *     metadata
 *     topology
 *     resilience
 *     observability
 *     provenance
 *
 * without turning those names into a closed grammar vocabulary.
 *
 * Semantic analysis determines the meaning of a property.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / CAPABILITY
 * ============================================================================
 *
 * The parser preserves these as syntax.
 *
 * Semantic analysis distinguishes:
 *
 *     requirement
 *     constraint
 *     preference
 *     capability
 *     hint
 *     policy
 *     metadata
 *
 * Examples:
 *
 *     requires: capability("network.reliable");
 *
 *     constraint: latency <= maximum_latency;
 *
 *     prefers: locality::near;
 *
 *     capability: network::multicast;
 *
 *     policy: adaptive;
 *
 * The grammar does not determine whether the requirement can be satisfied.
 *
 * ============================================================================
 * NO PHYSICAL TOPOLOGY IN THE CORE GRAMMAR
 * ============================================================================
 *
 * This grammar must not require:
 *
 *     router IDs;
 *     switch IDs;
 *     interface IDs;
 *     physical link IDs;
 *     physical node IDs;
 *     fixed path arrays;
 *     fixed hop counts;
 *     physical ports;
 *     MAC addresses;
 *     IP addresses;
 *     machine identifiers.
 *
 * If a program genuinely needs a target-specific deployment contract, that
 * information belongs to the appropriate hardware/deployment/dialect layer.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO language-level limits for:
 *
 *     routes;
 *     route properties;
 *     nested route blocks;
 *     route references;
 *     path elements;
 *     alternatives;
 *     policies;
 *     constraints;
 *     requirements;
 *     capabilities;
 *     qualified-name depth;
 *     expression complexity.
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_ROUTES
 *     MAX_ROUTE_PROPERTIES
 *     MAX_HOPS
 *     MAX_PATH_LENGTH
 *     MAX_NETWORK_SIZE
 *     MAX_NODES
 *     MAX_LINKS
 *     MAX_ROUTERS
 *     MAX_SWITCHES
 *     MAX_INTERFACES
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_CONNECTIONS
 *
 * Repetition is represented using ANTLR `*` / `+` constructs.
 *
 * Practical limits are determined by:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler resources;
 *     - runtime resources;
 *     - deployment resources;
 *     - target capabilities.
 *
 * Those are implementation/resource limits, not language limits.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * "Tiny to infinity" means that the language imposes no artificial finite
 * routing-system capacity.
 *
 * It does NOT mean physical networking has infinite capacity.
 *
 * The selected realization must have sufficient resources to execute the
 * program's declared semantics.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar defines NO lexer rules.
 *
 * It consumes the canonical:
 *
 *     ZamaniLexer
 *
 * and reuses:
 *
 *     Names
 *     Expressions
 *     Attributes
 *
 * Routing keywords such as `route`, `source`, `destination`, `via`, `policy`,
 * `requires`, `constraint`, and `prefers` are NOT made into a second keyword
 * authority here.
 *
 * `route` is therefore represented by a contextual marker.
 *
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar NetworkingRoutes;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary consumed by networking.g4.
 *
 * A route construct is a declaration.
 *
 * Route invocation/use remains ordinary expression/reference syntax unless a
 * future owning grammar explicitly defines another construct.
 * ============================================================================
 */

networkRouteConstruct
    : networkRouteDeclaration
    ;


/*
 * ============================================================================
 * ROUTE DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     route compute_path;
 *
 *     route compute_path {
 *         source: client;
 *         destination: compute;
 *     }
 *
 *     route quantum_result {
 *         source: quantum_service;
 *         destination: classical_service;
 *         protocol: quantum::network;
 *         requires: capability("network.reliable");
 *         prefers: locality::near;
 *     }
 *
 * The grammar intentionally does not prescribe which properties are mandatory.
 *
 * Semantic analysis determines contract completeness.
 * ============================================================================
 */

networkRouteDeclaration
    : attribute*
      networkRouteMarker
      identifier
      networkRouteBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL ROUTE MARKER
 * ============================================================================
 *
 * The route declaration marker remains an identifier so this grammar does not
 * create a second lexical authority.
 *
 * Semantic validation verifies the canonical source spelling in the routing
 * declaration context.
 * ============================================================================
 */

networkRouteMarker
    : identifier
    ;


/*
 * ============================================================================
 * ROUTE BODY
 * ============================================================================
 *
 * An arbitrary number of route members is allowed.
 * ============================================================================
 */

networkRouteBody
    : LBRACE
      networkRouteMember*
      RBRACE
    ;


/*
 * ============================================================================
 * ROUTE MEMBER
 * ============================================================================
 *
 * One canonical property structure plus nested configuration.
 *
 * This prevents dozens of parser alternatives that all have the same shape.
 * ============================================================================
 */

networkRouteMember
    : attribute*
      networkRouteProperty
    | attribute*
      networkRouteNestedBlock
    ;


/*
 * ============================================================================
 * ROUTE PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     property_name: expression;
 *
 * Examples:
 *
 *     source: client;
 *     destination: compute;
 *     via: preferred_region;
 *     policy: adaptive;
 *     metric: latency;
 *     requires: capability("network.reliable");
 *     constraint: latency <= maximum_latency;
 *     prefers: locality::near;
 *     topology: network::fabric;
 * ============================================================================
 */

networkRouteProperty
    : networkRoutePropertyName
      COLON
      networkRoutePropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * ROUTE PROPERTY NAME
 * ============================================================================
 *
 * Qualified names provide an open-world extension mechanism.
 *
 * Examples:
 *
 *     source
 *     destination
 *     network::source
 *     vendor::routing::policy
 *     application::routing::metadata
 * ============================================================================
 */

networkRoutePropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE PROPERTY VALUE
 * ============================================================================
 *
 * All values use the canonical expression grammar.
 *
 * Therefore routing does not create a second:
 *
 *     literal grammar;
 *     arithmetic grammar;
 *     comparison grammar;
 *     boolean grammar;
 *     collection grammar;
 *     call grammar;
 *     name grammar.
 * ============================================================================
 */

networkRoutePropertyValue
    : expression
    ;


/*
 * ============================================================================
 * NESTED ROUTE BLOCK
 * ============================================================================
 *
 * Nested blocks support structured policies and future routing extensions.
 *
 * Example:
 *
 *     route compute {
 *         policy {
 *             strategy: adaptive;
 *             metric: latency;
 *         }
 *
 *         constraints {
 *             require: capability("network.reliable");
 *         }
 *     }
 *
 * The grammar preserves structure.
 *
 * Semantic analysis determines whether the nested names are meaningful.
 * ============================================================================
 */

networkRouteNestedBlock
    : networkRouteNestedBlockName
      LBRACE
      networkRouteNestedMember*
      RBRACE
      SEMI?
    ;


networkRouteNestedBlockName
    : qualifiedName
    ;


networkRouteNestedMember
    : attribute*
      networkRouteProperty
    | attribute*
      networkRouteNestedBlock
    ;


/*
 * ============================================================================
 * ROUTE REFERENCE
 * ============================================================================
 *
 * This performs no symbol lookup.
 *
 * Examples:
 *
 *     compute_path
 *     network::compute_path
 *     quantum::routes::result
 *
 * Semantic analysis determines whether the referenced symbol denotes a route.
 * ============================================================================
 */

networkRouteReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE REFERENCE LIST
 * ============================================================================
 *
 * No artificial finite list size.
 *
 * Useful for constructs that need to refer to:
 *
 *     alternatives;
 *     fallback routes;
 *     preferred routes;
 *     route sets.
 *
 * ============================================================================
 */

networkRouteReferenceList
    : networkRouteReference
      (COMMA networkRouteReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL ROUTE REFERENCE LIST
 * ============================================================================
 */

optionalNetworkRouteReferenceList
    : networkRouteReferenceList?
    ;


/*
 * ============================================================================
 * ROUTE DECLARATION LIST
 * ============================================================================
 */

networkRouteDeclarationList
    : networkRouteDeclaration*
    ;


/*
 * ============================================================================
 * ROUTE PROPERTY LIST
 * ============================================================================
 */

networkRoutePropertyList
    : networkRouteProperty*
    ;


/*
 * ============================================================================
 * ROUTE ITEM
 * ============================================================================
 */

networkRouteItem
    : networkRouteDeclaration
    | networkRouteReference
    ;


/*
 * ============================================================================
 * ROUTE EXPRESSION
 * ============================================================================
 *
 * Explicit semantic/tooling boundary for a route reference without creating
 * a second expression language.
 * ============================================================================
 */

networkRouteExpression
    : networkRouteReference
    ;


/*
 * ============================================================================
 * SEMANTIC PROPERTY WRAPPERS
 * ============================================================================
 *
 * These wrappers provide stable names to AST builders, semantic analyzers,
 * documentation generators, and tooling without introducing duplicate syntax.
 *
 * They MUST NOT be used as separate route-member alternatives.
 * ============================================================================
 */

networkRouteSourceProperty
    : networkRouteProperty
    ;


networkRouteDestinationProperty
    : networkRouteProperty
    ;


networkRouteViaProperty
    : networkRouteProperty
    ;


networkRouteProtocolProperty
    : networkRouteProperty
    ;


networkRouteChannelProperty
    : networkRouteProperty
    ;


networkRouteEndpointProperty
    : networkRouteProperty
    ;


networkRoutePolicyProperty
    : networkRouteProperty
    ;


networkRouteMetricProperty
    : networkRouteProperty
    ;


networkRouteRequirementProperty
    : networkRouteProperty
    ;


networkRouteConstraintProperty
    : networkRouteProperty
    ;


networkRouteCapabilityProperty
    : networkRouteProperty
    ;


networkRoutePreferenceProperty
    : networkRouteProperty
    ;


networkRouteTopologyProperty
    : networkRouteProperty
    ;


networkRouteReliabilityProperty
    : networkRouteProperty
    ;


networkRouteSecurityProperty
    : networkRouteProperty
    ;


networkRouteMetadataProperty
    : networkRouteProperty
    ;


/*
 * ============================================================================
 * ROUTE POLICY BLOCK
 * ============================================================================
 *
 * A generic named policy block.
 *
 * This is intentionally not a fixed list of algorithms such as:
 *
 *     shortest_path
 *     dijkstra
 *     bellman_ford
 *     a_star
 *     ...
 *
 * Routing algorithms are implementation/semantic concerns.
 * ============================================================================
 */

networkRoutePolicyBlock
    : networkRouteNestedBlock
    ;


/*
 * ============================================================================
 * ROUTE CONSTRAINT BLOCK
 * ============================================================================
 */

networkRouteConstraintBlock
    : networkRouteNestedBlock
    ;


/*
 * ============================================================================
 * ROUTE REQUIREMENT BLOCK
 * ============================================================================
 */

networkRouteRequirementBlock
    : networkRouteNestedBlock
    ;


/*
 * ============================================================================
 * ROUTE CAPABILITY BLOCK
 * ============================================================================
 */

networkRouteCapabilityBlock
    : networkRouteNestedBlock
    ;


/*
 * ============================================================================
 * ROUTE PREFERENCE BLOCK
 * ============================================================================
 */

networkRoutePreferenceBlock
    : networkRouteNestedBlock
    ;


/*
 * ============================================================================
 * ROUTE TOPOLOGY REFERENCE
 * ============================================================================
 *
 * Topology identity is a logical reference only.
 *
 * The grammar does not inspect or resolve topology.
 * ============================================================================
 */

networkRouteTopologyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE ENDPOINT REFERENCE
 * ============================================================================
 *
 * Endpoint ownership remains with endpoints.g4.
 * ============================================================================
 */

networkRouteEndpointReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE CHANNEL REFERENCE
 * ============================================================================
 *
 * Channel ownership remains with channels.g4.
 * ============================================================================
 */

networkRouteChannelReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE PROTOCOL REFERENCE
 * ============================================================================
 *
 * Protocol ownership remains with protocols.g4.
 * ============================================================================
 */

networkRouteProtocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE SERVICE REFERENCE
 * ============================================================================
 *
 * Service ownership remains with services.g4.
 * ============================================================================
 */

networkRouteServiceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE REQUEST REFERENCE
 * ============================================================================
 *
 * Request ownership remains with requests.g4.
 * ============================================================================
 */

networkRouteRequestReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE RESPONSE REFERENCE
 * ============================================================================
 *
 * Response ownership remains with responses.g4.
 * ============================================================================
 */

networkRouteResponseReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE SOCKET REFERENCE
 * ============================================================================
 *
 * Socket ownership remains with sockets.g4.
 * ============================================================================
 */

networkRouteSocketReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE CAPABILITY REFERENCE
 * ============================================================================
 */

networkRouteCapabilityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROUTE LIST
 * ============================================================================
 *
 * Generic reusable list boundary.
 * ============================================================================
 */

networkRouteList
    : networkRouteReference
      (COMMA networkRouteReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * A route declaration should lower to the domain-neutral frontend AST with at
 * least:
 *
 *     - declaration attributes;
 *     - route name;
 *     - ordered route members;
 *     - property names;
 *     - property expressions;
 *     - nested route structures;
 *     - source spans.
 *
 * The parser must NOT resolve:
 *
 *     - endpoint names;
 *     - service names;
 *     - route names;
 *     - topology names;
 *     - protocol names;
 *     - capabilities;
 *     - resources.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - route-name resolution;
 *     - endpoint/reference validation;
 *     - protocol compatibility;
 *     - channel compatibility;
 *     - request/response compatibility;
 *     - capability satisfaction;
 *     - resource requirements;
 *     - security requirements;
 *     - topology compatibility;
 *     - route-policy validation;
 *     - constraint validation;
 *     - preference interpretation;
 *     - route feasibility;
 *     - route realization decisions.
 *
 * Parser acceptance MUST NOT imply route feasibility.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not construct IR.
 *
 * The intended path is:
 *
 *     route syntax
 *         ->
 *     frontend AST
 *         ->
 *     semantic routing model
 *         ->
 *     canonical/domain IR
 *         ->
 *     routing realization
 *
 * Routing may contribute metadata to:
 *
 *     - networking semantic representation;
 *     - distributed representation;
 *     - hardware communication representation;
 *     - classical computation;
 *     - hybrid computation;
 *     - quantum-related communication metadata.
 *
 * It does not create a second quantum IR.
 *
 * Where quantum computation participates:
 *
 *     semantic networking information
 *         ->
 *     canonical quantum::ir where appropriate
 *
 * remains the repository-wide quantum invariant.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use route semantics to:
 *
 *     - validate communication requirements;
 *     - select compatible realization strategies;
 *     - perform route planning;
 *     - optimize communication;
 *     - negotiate capabilities;
 *     - coordinate distributed placement;
 *     - generate target communication structures.
 *
 * These decisions occur after parsing.
 *
 * The compiler must not infer a physical route merely because a route
 * declaration exists.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns actual:
 *
 *     - route resolution;
 *     - route activation;
 *     - route failover;
 *     - packet/data forwarding;
 *     - transport operation;
 *     - endpoint binding;
 *     - network resource use;
 *     - runtime recovery.
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * CLASSICAL
 *
 * Classical computations may declare networking routes for communication
 * between logical computations.
 *
 * QUANTUM
 *
 * Quantum/classical programs may use route contracts for communication between
 * quantum services, classical control systems, distributed quantum resources,
 * or hybrid computations.
 *
 * The routing grammar does not define quantum operations.
 *
 * HDL / HARDWARE
 *
 * Hardware/software co-design may reference routing semantics, but physical
 * interconnect implementation belongs to hdl/hardware layers.
 *
 * DISTRIBUTED
 *
 * Distributed execution may consume route contracts for communication between
 * logical workers/services.
 *
 * Distributed node placement remains owned by distributed grammar/semantics.
 *
 * AI / DATA
 *
 * AI and data pipelines may use route contracts for model, dataset, service,
 * and stream communication.
 *
 * SECURITY
 *
 * Security requirements may be expressed as route properties and validated by
 * the security semantic layer.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Route properties may express semantic quantities such as:
 *
 *     latency;
 *     bandwidth;
 *     reliability;
 *     availability;
 *     energy;
 *     locality;
 *     cost;
 *     capacity.
 *
 * The grammar does not impose physical values.
 *
 * For example:
 *
 *     requires: bandwidth >= required_bandwidth;
 *
 * is valid semantic intent.
 *
 * The grammar must not define:
 *
 *     MAX_BANDWIDTH
 *     MIN_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_HOPS
 *
 * as universal language constants.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Security-related route properties are syntax only.
 *
 * They do not grant:
 *
 *     authentication;
 *     authorization;
 *     encryption;
 *     confidentiality;
 *     integrity;
 *     trust;
 *     key access.
 *
 * Security analysis must validate them independently.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no environment inspection;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access.
 *
 * Identical source/token input therefore produces identical parsing behavior.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * Every declaration, property, nested block, property name, expression,
 * reference, and attribute must remain traceable to its source span through
 * the normal ANTLR parse tree/frontend AST pipeline.
 *
 * No source information should be discarded by this grammar.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover syntax errors such as:
 *
 *     route;
 *     route { ... }
 *     route name { source }
 *     route name { : value; }
 *     route name { source:; }
 *     route name { source value; }
 *
 * Semantic diagnostics cover:
 *
 *     unknown route;
 *     unknown endpoint;
 *     unknown protocol;
 *     invalid route property;
 *     incompatible endpoint;
 *     unsatisfied capability;
 *     impossible constraint;
 *     unavailable resource;
 *     invalid security requirement.
 *
 * The parser must not attempt semantic diagnostics through actions/predicates.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is additive to the existing networking grammar architecture.
 *
 * Existing networking constructs retain their existing ownership:
 *
 *     endpoints.g4
 *     addresses.g4
 *     protocols.g4
 *     channels.g4
 *     messages.g4
 *     services.g4
 *     sockets.g4
 *     requests.g4
 *     responses.g4
 *     network-capabilities.g4
 *
 * The new public integration rule is:
 *
 *     networkRouteConstruct
 *
 * The aggregate networking grammar imports:
 *
 *     NetworkingRoutes
 *
 * and exposes it through:
 *
 *     networkingConstruct
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO universal:
 *
 *     MAX_ROUTES
 *     MAX_HOPS
 *     MAX_PATH_LENGTH
 *     MAX_NODES
 *     MAX_LINKS
 *     MAX_ROUTERS
 *     MAX_SWITCHES
 *     MAX_INTERFACES
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_CONNECTIONS
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * It contains no:
 *
 *     router_0
 *     switch_0
 *     interface_0
 *     node_0
 *     device_0
 *
 * as special language constructs.
 *
 * It contains no fixed transport vocabulary.
 *
 * It contains no fixed route-algorithm vocabulary.
 *
 * It contains no physical topology.
 *
 * It contains no target-selection logic.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 * route compute;
 *
 * route compute {
 *     source: client;
 *     destination: service;
 * }
 *
 * route compute {
 *     source: client;
 *     destination: service;
 *     policy: adaptive;
 * }
 *
 * route compute {
 *     source: client;
 *     destination: service;
 *     protocol: network::reliable;
 *     channel: results;
 * }
 *
 * route compute {
 *     requires: capability("network.reliable");
 *     constraint: latency <= maximum_latency;
 *     prefers: locality::near;
 * }
 *
 * route compute {
 *     topology: network::fabric;
 *     via: preferred_region;
 * }
 *
 * route compute {
 *     policy {
 *         strategy: adaptive;
 *         metric: latency;
 *     }
 *
 *     constraints {
 *         require: capability("network.reliable");
 *     }
 * }
 *
 * route quantum_result {
 *     source: quantum::service;
 *     destination: classical::controller;
 *     protocol: quantum::network;
 *     requires: capability("quantum.communication");
 * }
 *
 * route distributed_result {
 *     source: distributed::worker;
 *     destination: distributed::collector;
 *     requires: capability("network.reliable");
 * }
 *
 * route vendor_extension {
 *     vendor::routing::policy: future_policy;
 * }
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * route;
 *
 * route {
 * }
 *
 * route compute {
 *     source
 * }
 *
 * route compute {
 *     : client;
 * }
 *
 * route compute {
 *     source:;
 * }
 *
 * route compute {
 *     source client;
 * }
 *
 * route compute {
 *     source: client
 *     destination: service;
 * }
 *
 * route compute {
 *     policy {
 * }
 *
 * route compute {
 *     source: ;
 * }
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 *     - empty route body;
 *     - one property;
 *     - many properties;
 *     - one nested block;
 *     - deeply nested blocks;
 *     - deeply qualified property names;
 *     - deeply qualified references;
 *     - large expressions;
 *     - arbitrary property values;
 *     - arbitrary route declarations.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The test suite must verify:
 *
 *     - arbitrarily many route declarations;
 *     - arbitrarily many route properties;
 *     - arbitrarily many nested blocks;
 *     - arbitrarily many route references;
 *     - arbitrarily deep logical names;
 *     - large source programs.
 *
 * Tests must NOT encode a finite route capacity.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse the same source repeatedly and verify equivalent parse structures.
 *
 * No route decision may occur during parsing.
 *
 * ============================================================================
 * COMPATIBILITY TESTS
 * ============================================================================
 *
 * Verify routing integration with:
 *
 *     endpoints;
 *     addresses;
 *     protocols;
 *     channels;
 *     messages;
 *     services;
 *     sockets;
 *     requests;
 *     responses;
 *     capabilities;
 *     distributed constructs;
 *     security constructs;
 *     hardware intent;
 *     quantum/classical communication.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has one routing ownership boundary.
 * [x] It uses the canonical Zamani lexer.
 * [x] It uses canonical Names.
 * [x] It uses canonical Expressions.
 * [x] It uses canonical Attributes.
 * [x] It defines no lexer rules.
 * [x] It defines no second expression language.
 * [x] It defines no second type system.
 * [x] It defines no runtime behavior.
 * [x] It defines no routing algorithm.
 * [x] It defines no topology implementation.
 * [x] It defines no physical router model.
 * [x] It defines no physical interface model.
 * [x] It defines no transport enumeration.
 * [x] It defines no vendor enumeration.
 * [x] It defines no hardware enumeration.
 * [x] It has no artificial routing capacity.
 * [x] It supports open-world property names.
 * [x] It supports qualified names.
 * [x] It supports expressions as values.
 * [x] It supports nested routing configuration.
 * [x] It supports logical route references.
 * [x] It preserves source structure for AST construction.
 * [x] It preserves the canonical quantum::ir boundary.
 * [x] It integrates with networking semantic analysis.
 * [x] It integrates with resource/capability analysis.
 * [x] It integrates with distributed analysis.
 * [x] It integrates with security analysis.
 * [x] It integrates with compiler routing.
 * [x] It integrates with runtime routing.
 * [x] It is deterministic.
 * [x] It requires no unsafe Rust.
 * [x] It is compatible with Rust 1.97 / 1.97.1 generated-parser integration.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What routing intent does the program declare?"
 *
 * It does NOT answer:
 *
 *     "Which physical route will be used?"
 *
 *     "Which router will forward the data?"
 *
 *     "Which network interface will be selected?"
 *
 *     "Which machine will execute it?"
 *
 *     "Which hardware will realize it?"
 *
 * Those answers belong downstream.
 *
 * ============================================================================
 */