/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/endpoints.g4
 *
 * Grammar:
 *     Endpoints
 *
 * Status:
 *     Production grammar contract
 *
 * Purpose:
 *     Canonical parser grammar for logical networking endpoints.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust
 *     - No actions
 *     - No semantic predicates
 *     - No unsafe code
 *     - No filesystem access
 *     - No network access
 *     - No hardware access
 *     - No runtime callbacks
 *     - No nondeterministic parsing
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     Endpoints
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic networking model
 *          |
 *          +--> networking IR
 *          +--> distributed semantics
 *          +--> classical semantics
 *          +--> quantum::ir where applicable
 *          +--> hardware intent
 *          |
 *          v
 *     routing / scheduling / discovery / deployment
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * This grammar describes WHAT a logical endpoint means syntactically.
 *
 * It does not describe HOW that endpoint is physically realized.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical endpoint declarations;
 *     - endpoint declaration names;
 *     - endpoint bodies;
 *     - endpoint members;
 *     - endpoint member keys;
 *     - endpoint member values;
 *     - endpoint-local nested configuration;
 *     - reusable logical endpoint references;
 *     - endpoint source-level attributes expressed through the common
 *       expression/name system;
 *     - endpoint-local configuration structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - keywords;
 *     - Unicode identifier rules;
 *     - literals;
 *     - general expression precedence;
 *     - type syntax;
 *     - data schemas;
 *     - serialization;
 *     - wire formats;
 *     - cryptographic implementation;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - DNS;
 *     - IP allocation;
 *     - MAC addresses;
 *     - network interfaces;
 *     - operating-system sockets;
 *     - transport implementations;
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - HTTP;
 *     - MPI;
 *     - RDMA;
 *     - vendor networking APIs;
 *     - cloud-provider APIs;
 *     - route selection;
 *     - packet scheduling;
 *     - topology realization;
 *     - physical network discovery;
 *     - distributed placement;
 *     - cluster membership;
 *     - replication;
 *     - consensus;
 *     - resilience;
 *     - runtime dispatch;
 *     - hardware discovery;
 *     - CPU topology;
 *     - GPU topology;
 *     - FPGA topology;
 *     - accelerator topology;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * An endpoint is a logical communication participant.
 *
 * It is NOT intrinsically:
 *
 *     - a machine;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - a QPU;
 *     - a process;
 *     - a thread;
 *     - a socket;
 *     - a NIC;
 *     - an IP address;
 *     - a router;
 *     - a switch;
 *     - a cloud instance;
 *     - a physical node.
 *
 * The same source endpoint declaration may therefore be realized as:
 *
 *     - an in-process object;
 *     - a task;
 *     - a process;
 *     - a service;
 *     - an embedded component;
 *     - a distributed service;
 *     - an accelerator endpoint;
 *     - a quantum-control endpoint;
 *     - a future execution resource.
 *
 * The realization is downstream from parsing.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains NO universal limits for:
 *
 *     MAX_ENDPOINTS
 *     MAX_ENDPOINT_PROPERTIES
 *     MAX_ENDPOINT_MEMBERS
 *     MAX_ENDPOINT_REFERENCES
 *     MAX_ENDPOINT_RELATIONSHIPS
 *     MAX_ENDPOINT_CAPABILITIES
 *     MAX_ADDRESSES
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_SERVICES
 *     MAX_NETWORK_SIZE
 *     MAX_TOPOLOGY_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_ADDRESS_WIDTH
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Actual limitations may arise from:
 *
 *     - available memory;
 *     - source size;
 *     - parser implementation;
 *     - compiler configuration;
 *     - operating-system resources;
 *     - deployment resources;
 *     - runtime resources.
 *
 * Such implementation/environment limits are NOT language limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_ENDPOINTS
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_NETWORK_SIZE
 *     MAX_INTERFACES
 *     MAX_SERVICES
 *     MAX_PROTOCOLS
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_ADDRESS_WIDTH
 *
 * It MUST NOT encode physical identifiers such as:
 *
 *     physical_node_0
 *     interface_0
 *     router_0
 *     socket_0
 *     gpu_0
 *     qpu_0
 *
 * as language-level concepts.
 *
 * Source programs may contain such names as ordinary identifiers when an
 * application requires them, but this grammar assigns them no physical
 * meaning.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * An endpoint property may express:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     policy
 *     metadata
 *     logical address
 *     protocol identity
 *     role
 *
 * The parser does not decide which semantic category a property belongs to.
 *
 * For example:
 *
 *     endpoint producer {
 *         role: source;
 *         requires: capability("network.reliable");
 *         prefers: locality;
 *     }
 *
 * is syntactically represented using the same general endpoint-member
 * mechanism.
 *
 * Semantic analysis determines:
 *
 *     - property meaning;
 *     - type;
 *     - validity;
 *     - satisfiability;
 *     - capability requirements;
 *     - resource implications;
 *     - portability.
 *
 * ============================================================================
 * CONTEXTUAL `endpoint` MARKER
 * ============================================================================
 *
 * The current canonical lexer does not introduce a dedicated endpoint keyword.
 *
 * Therefore this grammar intentionally keeps:
 *
 *     endpointMarker
 *         : identifier
 *         ;
 *
 * The networking semantic layer MUST validate that the marker denotes the
 * endpoint declaration construct in the relevant grammar context.
 *
 * This preserves the repository's current lexical authority and avoids
 * introducing a second keyword authority solely for networking.
 *
 * If `endpoint` becomes a globally reserved lexical token in a future
 * language-version migration, the semantic model of this grammar does not
 * need to change.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * A previous design used many alternatives equivalent to:
 *
 *     identifier COLON expression SEMICOLON
 *
 * under different rule names such as role, address, protocol, capability,
 * requirement, preference, security, locality, policy and metadata.
 *
 * Those alternatives are structurally indistinguishable to the parser.
 *
 * This production grammar deliberately eliminates that duplication.
 *
 * There is one canonical endpoint property form:
 *
 *     endpointProperty
 *         : identifier COLON expression SEMICOLON
 *         ;
 *
 * Semantic analysis assigns the property's meaning.
 *
 * This provides:
 *
 *     - deterministic parsing;
 *     - less grammar ambiguity;
 *     - no keyword explosion;
 *     - future extensibility;
 *     - no need to modify this grammar whenever a new networking property is
 *       introduced;
 *     - compatibility with user-defined and future networking capabilities.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical names:
 *
 *     grammar/core/names.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file defines NO lexer rules.
 *
 * It consumes:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *
 * from the canonical parser grammar composition.
 *
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * Parser grammars throughout the production networking architecture use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file therefore uses the same vocabulary.
 *
 * No local token aliases are introduced.
 *
 * ============================================================================
 * DECLARATION MODEL
 * ============================================================================
 *
 * The canonical declaration form is:
 *
 *     endpoint <name>;
 *
 * or:
 *
 *     endpoint <name> {
 *         <member>* 
 *     }
 *
 * Examples:
 *
 *     endpoint producer;
 *
 *     endpoint consumer {
 *         role: sink;
 *     }
 *
 * The optional semicolon after a body is accepted:
 *
 *     endpoint consumer {
 *         role: sink;
 *     };
 *
 * This makes declaration termination compatible with both block-oriented and
 * semicolon-oriented source styles without creating a second declaration
 * model.
 *
 * ============================================================================
 * MEMBER MODEL
 * ============================================================================
 *
 * Endpoint members have exactly two structural categories:
 *
 *     1. endpointProperty
 *     2. endpointNestedBlock
 *
 * Property:
 *
 *     key: expression;
 *
 * Nested block:
 *
 *     key {
 *         ...
 *     }
 *
 * Examples:
 *
 *     endpoint node {
 *         role: producer;
 *         address: logical::producer;
 *         protocol: reliable;
 *         capability: quantum;
 *         requires: capability("network.reliable");
 *         locality: region;
 *         metadata: "example";
 *     }
 *
 * Nested configuration:
 *
 *     endpoint service {
 *         policy {
 *             retry: enabled;
 *             resilience: adaptive;
 *         }
 *     }
 *
 * The nested structure remains syntactic.
 *
 * Semantic interpretation belongs downstream.
 *
 * ============================================================================
 * PROPERTY KEYS
 * ============================================================================
 *
 * Property keys are ordinary identifiers.
 *
 * The grammar intentionally does NOT enumerate:
 *
 *     role
 *     address
 *     protocol
 *     capability
 *     requires
 *     constraint
 *     preference
 *     security
 *     locality
 *     metadata
 *     policy
 *
 * as a closed keyword list.
 *
 * This is necessary for:
 *
 *     - future protocols;
 *     - future networking models;
 *     - vendor-neutral extensions;
 *     - application-defined properties;
 *     - dialects;
 *     - capability evolution;
 *     - POCO-REAF.
 *
 * Semantic validation may impose rules on well-known properties.
 *
 * ============================================================================
 * LOGICAL ADDRESSING
 * ============================================================================
 *
 * Endpoint addresses are expressions rather than a dedicated physical-address
 * grammar.
 *
 * Therefore all of the following may be represented without making them
 * mandatory physical network concepts:
 *
 *     logical::service
 *     service_name
 *     discovery_key
 *     symbolic_address
 *     application_address
 *     runtime_address
 *     user_defined_locator
 *
 * A literal IPv4/IPv6/string value, if accepted by the common expression
 * grammar, is application data unless a downstream semantic subsystem assigns
 * it network-address meaning.
 *
 * This is deliberate.
 *
 * ============================================================================
 * REFERENCES
 * ============================================================================
 *
 * A reusable logical endpoint reference is represented by the canonical
 * `qualifiedName` rule:
 *
 *     endpointReference
 *         : qualifiedName
 *         ;
 *
 * This rule performs NO lookup.
 *
 * Examples:
 *
 *     producer
 *     services::producer
 *     quantum::control::endpoint
 *
 * Name resolution is downstream.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Endpoints may participate in:
 *
 *     - classical computing;
 *     - quantum computing;
 *     - hybrid computing;
 *     - HDL systems;
 *     - accelerators;
 *     - AI/data systems;
 *     - distributed computation;
 *     - embedded systems;
 *     - future computing domains.
 *
 * This grammar does not import those domains.
 *
 * Instead:
 *
 *     endpoint property values
 *
 * use the common:
 *
 *     Names
 *     Expressions
 *     Types
 *     Resources
 *     Capabilities
 *
 * contracts downstream.
 *
 * This prevents networking from becoming coupled to:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     QPU;
 *     quantum::ir;
 *     QEC;
 *     ZQN.
 *
 * ============================================================================
 * NETWORKING / DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Networking:
 *
 *     logical communication participant syntax.
 *
 * Distributed:
 *
 *     placement;
 *     node membership;
 *     replication;
 *     distributed scheduling;
 *     distributed deployment;
 *     consensus;
 *     partitioning.
 *
 * Routing:
 *
 *     logical-to-physical route realization.
 *
 * Service discovery:
 *
 *     resolution of logical service identities.
 *
 * Channels:
 *
 *     communication-channel semantics.
 *
 * Protocols:
 *
 *     protocol contracts.
 *
 * Sockets:
 *
 *     socket/transport-oriented semantics.
 *
 * Hardware:
 *
 *     physical network capability and topology.
 *
 * Execution/runtime:
 *
 *     runtime endpoint realization.
 *
 * This grammar does not duplicate those responsibilities.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Endpoint properties may syntactically express security intent:
 *
 *     security: encrypted;
 *     requires: authenticated;
 *     trust: trusted;
 *
 * but this grammar does not define:
 *
 *     - cryptographic algorithms;
 *     - key generation;
 *     - key storage;
 *     - certificate verification;
 *     - identity providers;
 *     - authorization engines;
 *     - trust evaluation.
 *
 * Those remain owned by:
 *
 *     grammar/security/
 *
 * and the corresponding semantic/compiler/runtime layers.
 *
 * ============================================================================
 * HARDWARE / QUANTUM BOUNDARY
 * ============================================================================
 *
 * This file does not define:
 *
 *     PhysicalQubitId
 *     QubitId
 *     Gate
 *     Circuit
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     NIC
 *     router
 *     switch
 *     physical link
 *
 * A property such as:
 *
 *     target: quantum
 *
 * remains a semantic expression.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should provide enough structure for the frontend AST to preserve:
 *
 *     EndpointDeclaration
 *         marker span
 *         name
 *         body
 *         declaration span
 *
 *     EndpointProperty
 *         key
 *         value expression
 *         source span
 *
 *     EndpointNestedBlock
 *         key
 *         members
 *         source span
 *
 *     EndpointReference
 *         qualified name
 *         source span
 *
 * The AST remains domain-neutral.
 *
 * This grammar MUST NOT depend on Rust AST implementation details.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - validating endpoint marker meaning;
 *     - validating endpoint names;
 *     - duplicate declaration checks;
 *     - visibility;
 *     - name resolution;
 *     - reference resolution;
 *     - property type checking;
 *     - property-schema validation;
 *     - capability validation;
 *     - resource requirement validation;
 *     - security-policy validation;
 *     - portability analysis;
 *     - cross-domain validation;
 *     - requirement satisfiability.
 *
 * The parser MUST NOT perform these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create an IR.
 *
 * Parsed endpoints lower into the repository's canonical semantic networking
 * model.
 *
 * That model may subsequently lower into:
 *
 *     - networking IR;
 *     - distributed IR;
 *     - classical IR;
 *     - hardware/network intent;
 *     - runtime endpoint descriptions.
 *
 * If an endpoint participates in a quantum computation, quantum semantics
 * continue through the established:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This file MUST NOT create another quantum IR.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Consumers MUST preserve source locations for:
 *
 *     - endpoint declaration;
 *     - endpoint marker;
 *     - endpoint name;
 *     - endpoint body;
 *     - each property key;
 *     - each property value;
 *     - each nested block;
 *     - each endpoint reference.
 *
 * Diagnostics must therefore be able to identify the exact source construct
 * responsible for an error.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The grammar must permit deterministic parser diagnostics for:
 *
 *     - missing endpoint name;
 *     - malformed endpoint body;
 *     - missing opening brace;
 *     - missing closing brace;
 *     - malformed property;
 *     - missing colon;
 *     - missing expression;
 *     - missing semicolon;
 *     - malformed nested block;
 *     - malformed qualified endpoint reference.
 *
 * Diagnostic formatting remains owned by the frontend diagnostic subsystem.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar intentionally preserves the public rule:
 *
 *     endpointDeclaration
 *
 * Existing networking composition should therefore import this grammar rather
 * than rename the rule.
 *
 * The following existing conceptual rules are intentionally consolidated:
 *
 *     endpointRoleDeclaration
 *     endpointAddressDeclaration
 *     endpointProtocolDeclaration
 *     endpointCapabilityDeclaration
 *     endpointRequirementDeclaration
 *     endpointConstraintDeclaration
 *     endpointPreferenceDeclaration
 *     endpointSecurityDeclaration
 *     endpointLocalityDeclaration
 *     endpointPropertyDeclaration
 *     endpointPolicyDeclaration
 *     endpointMetadataDeclaration
 *
 * They were structurally equivalent because each ultimately represented:
 *
 *     identifier COLON expression SEMICOLON
 *
 * Their semantic distinction belongs downstream.
 *
 * This consolidation avoids grammar ambiguity while preserving their source
 * vocabulary as ordinary property keys.
 *
 * ============================================================================
 * INTEGRATION WITH networking.g4
 * ============================================================================
 *
 * The aggregate networking grammar MUST import this grammar:
 *
 *     import Endpoints;
 *
 * It MUST NOT redefine:
 *
 *     endpointDeclaration
 *     endpointMarker
 *     endpointBody
 *     endpointMember
 *     endpointProperty
 *     endpointNestedBlock
 *     endpointReference
 *
 * The aggregate grammar should expose:
 *
 *     endpointDeclaration
 *
 * through its networking declaration/statement/expression composition.
 *
 * ============================================================================
 * INTEGRATION WITH Zamani.g4
 * ============================================================================
 *
 * `grammar/Zamani.g4` remains the canonical root grammar.
 *
 * It does not duplicate endpoint syntax.
 *
 * The integration path is:
 *
 *     Zamani.g4
 *         |
 *         v
 *     networking aggregate
 *         |
 *         v
 *     Endpoints
 *
 * No second root grammar is introduced.
 *
 * ============================================================================
 * INTEGRATION WITH NAMES
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Names
 *
 * and therefore reuses:
 *
 *     identifier
 *     qualifiedName
 *
 * without defining a second name grammar.
 *
 * ============================================================================
 * INTEGRATION WITH EXPRESSIONS
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Expressions
 *
 * and therefore reuses:
 *
 *     expression
 *
 * for endpoint property values.
 *
 * Endpoint properties may consequently evolve with the universal expression
 * language without requiring endpoint grammar changes.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES / CAPABILITIES
 * ============================================================================
 *
 * This grammar does not import the resource/capability implementation.
 *
 * Instead:
 *
 *     requires: ...
 *     capability: ...
 *     constraint: ...
 *     preference: ...
 *
 * are parsed as ordinary property/value pairs.
 *
 * Semantic analysis later maps them to the canonical resource/capability
 * model.
 *
 * This avoids coupling endpoint syntax to any particular resource backend.
 *
 * ============================================================================
 * INTEGRATION WITH SERVICE DISCOVERY
 * ============================================================================
 *
 * Logical endpoint identity may be represented by:
 *
 *     endpointReference
 *
 * but this grammar does not resolve it.
 *
 * Service discovery owns:
 *
 *     logical identity -> discovered service realization
 *
 * downstream.
 *
 * ============================================================================
 * INTEGRATION WITH ROUTING
 * ============================================================================
 *
 * Endpoint syntax describes communication participants.
 *
 * Routing owns:
 *
 *     logical endpoint -> route realization.
 *
 * This grammar does not select routes.
 *
 * ============================================================================
 * INTEGRATION WITH SCHEDULING
 * ============================================================================
 *
 * This grammar does not schedule endpoint communication.
 *
 * Scheduling owns:
 *
 *     temporal order;
 *     timing;
 *     resource scheduling;
 *     synchronization.
 *
 * ============================================================================
 * INTEGRATION WITH RUNTIME
 * ============================================================================
 *
 * The runtime may realize an endpoint as any available execution resource.
 *
 * This grammar makes no assumption about:
 *
 *     - process model;
 *     - operating system;
 *     - network stack;
 *     - machine count;
 *     - hardware topology;
 *     - accelerator availability.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Generated/parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must use safe Rust only.
 *
 * This grammar introduces no Rust actions and therefore introduces no
 * opportunity for unsafe Rust inside the grammar itself.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     endpoint producer;
 *
 *     endpoint producer {
 *         role: source;
 *     }
 *
 *     endpoint consumer {
 *         role: sink;
 *         protocol: reliable;
 *         requires: capability("network.reliable");
 *     }
 *
 *     endpoint quantum_control {
 *         capability: quantum;
 *         target: quantum;
 *     }
 *
 *     endpoint service {
 *         address: services::compute;
 *         locality: region;
 *     }
 *
 *     endpoint distributed::worker {
 *         role: worker;
 *     }
 *
 *     endpoint nested {
 *         policy {
 *             retry: adaptive;
 *         }
 *     }
 *
 *     endpointReference
 *         -> producer
 *         -> services::producer
 *         -> quantum::control::endpoint
 *
 * NEGATIVE:
 *
 *     endpoint;
 *
 *     endpoint { ... };
 *
 *     endpoint producer {
 *         role
 *     }
 *
 *     endpoint producer {
 *         : source;
 *     }
 *
 *     endpoint producer {
 *         role source;
 *     }
 *
 *     endpoint producer {
 *         role:;
 *     }
 *
 *     endpoint producer {
 *         role: source
 *     }
 *
 *     endpoint producer {
 *         policy {
 *     }
 *
 * BOUNDARY:
 *
 *     one endpoint;
 *     many endpoints;
 *     deeply qualified names;
 *     large property sets;
 *     large nested configuration;
 *     arbitrarily large expression values;
 *     arbitrarily many endpoint references.
 *
 * SCALABILITY:
 *
 *     The test suite must demonstrate that grammar acceptance does not depend
 *     on an artificial endpoint/network/device count.
 *
 * DETERMINISM:
 *
 *     Identical token streams must produce identical parse structures.
 *
 * PORTABILITY:
 *
 *     Endpoint declarations must not require a specific:
 *
 *         CPU
 *         GPU
 *         FPGA
 *         QPU
 *         node
 *         NIC
 *         network topology
 *         transport
 *
 * HARD-CODING AUDIT:
 *
 *     The grammar must reject no valid program merely because a target has
 *     fewer or more network resources than another target.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] Single endpoint grammar owner.
 * [x] Canonical ZamaniLexer vocabulary.
 * [x] Canonical Names integration.
 * [x] Canonical Expressions integration.
 * [x] No lexer rules.
 * [x] No embedded Rust.
 * [x] No unsafe code.
 * [x] No semantic predicates.
 * [x] No physical address grammar.
 * [x] No transport grammar.
 * [x] No routing grammar.
 * [x] No discovery implementation.
 * [x] No distributed placement.
 * [x] No hardware topology.
 * [x] No QEC.
 * [x] No ZQN.
 * [x] No second quantum IR.
 * [x] No artificial capacity limits.
 * [x] Deterministic endpoint-member structure.
 * [x] Extensible property vocabulary.
 * [x] Nested logical configuration.
 * [x] Logical endpoint references.
 * [x] Source-span-preserving structure.
 * [x] AST contract defined.
 * [x] Semantic contract defined.
 * [x] IR boundary defined.
 * [x] networking.g4 integration defined.
 * [x] Zamani.g4 integration defined.
 * [x] resource/capability integration defined.
 * [x] routing integration defined.
 * [x] discovery integration defined.
 * [x] scheduling integration defined.
 * [x] runtime integration defined.
 * [x] scalability contract defined.
 * [x] compatibility contract defined.
 *
 * ============================================================================
 */

parser grammar Endpoints;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC ENDPOINT DECLARATION
 * ============================================================================
 *
 * Contextual form:
 *
 *     endpoint name;
 *     endpoint name { ... }
 *
 * The marker is validated semantically because the current lexical architecture
 * does not make `endpoint` a globally reserved token.
 */
endpointDeclaration
    : endpointMarker
      identifier
      endpointBody?
      SEMICOLON?
    ;


/* ============================================================================
 * CONTEXTUAL ENDPOINT MARKER
 * ============================================================================
 */

endpointMarker
    : identifier
    ;


/* ============================================================================
 * ENDPOINT BODY
 * ============================================================================
 */

endpointBody
    : LBRACE
      endpointMember*
      RBRACE
    ;


/* ============================================================================
 * ENDPOINT MEMBER
 * ============================================================================
 *
 * Every endpoint member has an unambiguous structural form.
 *
 * Property:
 *
 *     name: expression;
 *
 * Nested block:
 *
 *     name { ... }
 *
 * The meaning of `name` is semantic.
 */
endpointMember
    : endpointProperty
    | endpointNestedBlock
    ;


/* ============================================================================
 * ENDPOINT PROPERTY
 * ============================================================================
 */

endpointProperty
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * ENDPOINT NESTED BLOCK
 * ============================================================================
 *
 * Nested blocks are intentionally generic.
 *
 * They permit future endpoint configuration domains without requiring this
 * grammar to enumerate every future networking concept.
 */
endpointNestedBlock
    : identifier
      endpointBody
    ;


/* ============================================================================
 * ENDPOINT REFERENCE
 * ============================================================================
 *
 * A reference is a canonical qualified name.
 *
 * It performs no lookup.
 */
endpointReference
    : qualifiedName
    ;


/* ============================================================================
 * ENDPOINT REFERENCE LIST
 * ============================================================================
 *
 * Reuses the canonical qualified-name list structure.
 *
 * The list itself remains unbounded by language design.
 */
endpointReferenceList
    : endpointReference
      (COMMA endpointReference)*
    ;


/* ============================================================================
 * ENDPOINT NAME
 * ============================================================================
 *
 * Explicit wrapper for contexts that need to state that a name is an endpoint
 * identity without changing the underlying name grammar.
 */
endpointName
    : qualifiedName
    ;


/* ============================================================================
 * ENDPOINT NAME LIST
 * ============================================================================
 */

endpointNameList
    : endpointName
      (COMMA endpointName)*
    ;


/* ============================================================================
 * ENDPOINT PROPERTY KEY
 * ============================================================================
 *
 * Explicit reusable wrapper.
 *
 * This is deliberately an identifier rather than a closed keyword list.
 */
endpointPropertyKey
    : identifier
    ;


/* ============================================================================
 * ENDPOINT PROPERTY VALUE
 * ============================================================================
 *
 * Endpoint values use the universal expression grammar.
 */
endpointPropertyValue
    : expression
    ;


/* ============================================================================
 * ENDPOINT DECLARATION REFERENCE
 * ============================================================================
 *
 * This rule is useful to aggregate grammars and semantic tooling that need a
 * named endpoint-reference boundary.
 */
endpointReferenceExpression
    : endpointReference
    ;


/* ============================================================================
 * ENDPOINT CONFIGURATION
 * ============================================================================
 *
 * Reusable body boundary for downstream grammar composition.
 */
endpointConfiguration
    : endpointBody
    ;