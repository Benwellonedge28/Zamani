/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/responses.g4
 *
 * Grammar:
 *     NetworkingResponses
 *
 * Status:
 *     PRODUCTION NETWORKING COMPONENT GRAMMAR
 *
 * Purpose:
 *     Define reusable, first-class, target-independent NETWORK RESPONSE
 *     CONTRACTS.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
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
 *     - No randomness
 *     - No environment inspection
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical parser composition
 *          |
 *          v
 *     NetworkingResponses
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> portability analysis
 *          +--> networking analysis
 *          +--> distributed analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> networking semantics
 *          +--> distributed semantics
 *          +--> classical semantics
 *          +--> hybrid semantics
 *          +--> hardware communication intent
 *          +--> quantum-related semantic metadata
 *          |
 *          v
 *     canonical IR / lowering
 *          |
 *          +--> classical representation
 *          +--> quantum::ir where quantum computation participates
 *          +--> networking/distributed representation
 *          +--> HDL/hardware representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / discovery / placement / scheduling
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * THIS FILE NEVER:
 *
 *     - sends a response;
 *     - receives a response;
 *     - opens a socket;
 *     - selects a route;
 *     - selects a machine;
 *     - selects a CPU/GPU/FPGA/QPU;
 *     - allocates a network resource;
 *     - serializes data;
 *     - deserializes data;
 *     - authenticates;
 *     - authorizes;
 *     - performs cryptography;
 *     - performs service discovery;
 *     - performs scheduling;
 *     - constructs IR;
 *     - executes runtime code.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - reusable network response declarations;
 *     - response declaration identity;
 *     - response contract bodies;
 *     - response contract properties;
 *     - response nested configuration;
 *     - response request relationships;
 *     - response message/schema references expressed through properties;
 *     - response source/destination intent expressed through properties;
 *     - response protocol/channel intent expressed through properties;
 *     - response status/outcome intent;
 *     - response error/failure intent;
 *     - response requirements;
 *     - response constraints;
 *     - response capabilities;
 *     - response preferences;
 *     - response policies;
 *     - response metadata;
 *     - stable parser entry points for response constructs;
 *     - reusable response references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - message schema definitions;
 *     - message values;
 *     - endpoints;
 *     - addresses;
 *     - channels;
 *     - protocols;
 *     - services;
 *     - sockets;
 *     - transports;
 *     - serialization;
 *     - wire formats;
 *     - HTTP;
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - MQTT;
 *     - gRPC;
 *     - MPI;
 *     - RDMA;
 *     - routing;
 *     - scheduling;
 *     - topology;
 *     - distributed placement;
 *     - node membership;
 *     - replication;
 *     - consensus;
 *     - security implementation;
 *     - cryptographic implementation;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime execution;
 *     - quantum gates;
 *     - quantum states;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * grammar/networking/services.g4 already owns SERVICE-LOCAL response
 * declarations:
 *
 *     networkServiceResponseDeclaration
 *
 * and service-local response references.
 *
 * Those constructs describe responses inside a service declaration.
 *
 * THIS FILE owns reusable, first-class response contracts that can be
 * referenced by:
 *
 *     - services;
 *     - requests;
 *     - channels;
 *     - distributed computation;
 *     - hybrid execution;
 *     - classical computation;
 *     - quantum/classical communication;
 *     - data pipelines;
 *     - AI systems;
 *     - hardware/software communication;
 *     - future networking domains.
 *
 * Therefore:
 *
 *     services.g4
 *         -> service-local response syntax
 *
 *     responses.g4
 *         -> reusable response contracts
 *
 * Neither grammar duplicates the other's declaration implementation.
 *
 * ============================================================================
 * REQUEST / RESPONSE RELATIONSHIP
 * ============================================================================
 *
 * A reusable response may refer to a reusable request:
 *
 *     response GetUserResult {
 *         request: api::GetUser;
 *         message: User;
 *     }
 *
 * This grammar records the relationship syntactically.
 *
 * Semantic analysis determines:
 *
 *     - whether the referenced request exists;
 *     - whether the request is compatible;
 *     - whether the response is legal for that request;
 *     - whether cardinality is valid;
 *     - whether the response direction is valid;
 *     - whether the referenced message/type is valid.
 *
 * The grammar itself performs none of those checks.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A response describes LOGICAL COMMUNICATION INTENT.
 *
 * It does not describe a physical realization.
 *
 * The same response may eventually be realized through:
 *
 *     - an in-process call;
 *     - shared memory;
 *     - IPC;
 *     - local messaging;
 *     - a network;
 *     - a distributed fabric;
 *     - an accelerator fabric;
 *     - CPU/GPU/FPGA communication;
 *     - quantum/classical communication;
 *     - HPC interconnects;
 *     - cloud infrastructure;
 *     - edge infrastructure;
 *     - future computational substrates.
 *
 * The source response contract remains independent of those realizations.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     HTTP
 *     HTTPS
 *     TCP
 *     UDP
 *     QUIC
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transports
 *     cloud providers
 *     hardware vendors
 *     network devices
 *
 * Such concepts remain ordinary names and expressions where appropriate.
 *
 * Example:
 *
 *     protocol: network::request_response;
 *
 *     protocol: vendor::future_transport;
 *
 *     transport: selected_transport;
 *
 * Semantic analysis determines whether a referenced protocol, transport,
 * capability, or policy is valid and realizable.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / CAPABILITY
 * ============================================================================
 *
 * Response properties are syntactic data.
 *
 * Semantic analysis determines whether a property represents:
 *
 *     - requirement;
 *     - constraint;
 *     - capability;
 *     - preference;
 *     - policy;
 *     - metadata;
 *     - implementation hint;
 *     - request/response relationship;
 *     - response semantics.
 *
 * Examples:
 *
 *     requires: capability("network.reliable");
 *
 *     requires: bandwidth >= required_bandwidth;
 *
 *     prefers: locality::near;
 *
 *     constraint: latency < maximum_latency;
 *
 *     capability: capability("network.streaming");
 *
 *     security: security::authenticated;
 *
 * The parser does not determine whether the requirements can be satisfied.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO language-level limits for:
 *
 *     response contracts;
 *     response members;
 *     response properties;
 *     response arguments;
 *     nested response blocks;
 *     qualified-name depth;
 *     response declarations;
 *     request relationships;
 *     message relationships;
 *     endpoints;
 *     channels;
 *     protocols;
 *     services;
 *     nodes;
 *     devices;
 *     network size;
 *     payload size;
 *     bandwidth;
 *     latency;
 *     retries;
 *     distributed participants.
 *
 * Repetition uses ANTLR `*` / `+` constructs.
 *
 * Practical limitations may arise from:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler resources;
 *     - operating-system resources;
 *     - runtime resources;
 *     - deployment resources;
 *     - target hardware.
 *
 * Those are implementation/resource limitations, NOT language-level
 * networking limits.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * "Scale from tiny to infinity" means:
 *
 *     the language grammar does not impose an artificial finite upper bound
 *     on the number, size, or logical complexity of response contracts.
 *
 * It does NOT mean that a physical implementation has infinite resources.
 *
 * Actual execution remains constrained by resources available to the
 * selected compilation/runtime realization.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_RESPONSES
 *     MAX_RESPONSE_PROPERTIES
 *     MAX_RESPONSE_MEMBERS
 *     MAX_REQUESTS
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *     MAX_MESSAGE_SIZE
 *     MAX_PAYLOAD_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *
 * It MUST NOT encode physical resources such as:
 *
 *     cpu_0
 *     gpu_0
 *     fpga_0
 *     qpu_0
 *     node_0
 *     socket_0
 *     interface_0
 *
 * as special networking language concepts.
 *
 * Application source may contain such names as ordinary program data.
 * This grammar assigns them no physical meaning.
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
 * and canonical parser components:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * No networking-specific lexer is introduced.
 *
 * The declaration marker is intentionally contextual:
 *
 *     response
 *
 * is represented structurally through:
 *
 *     networkResponseMarker
 *         : identifier
 *         ;
 *
 * Semantic validation establishes whether the marker is the canonical
 * response declaration spelling in the networking declaration context.
 *
 * This preserves the repository's current single lexical-authority model.
 *
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar NetworkingResponses;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary for the networking aggregate.
 *
 * Only reusable response declarations are public response constructs.
 *
 * Response invocation/receipt is NOT defined here.
 *
 * Invocation and ordinary expression behavior belong to the canonical
 * expression/call system and to downstream networking semantics.
 * ============================================================================
 */

networkResponseConstruct
    : networkResponseDeclaration
    ;


/*
 * ============================================================================
 * RESPONSE DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     response GetUserResult;
 *
 *     response GetUserResult {
 *         request: api::GetUser;
 *         message: User;
 *         source: user_service;
 *         destination: client;
 *         protocol: network::request_response;
 *     }
 *
 *     response ComputeResult {
 *         request: Compute;
 *         message: ComputeResult;
 *         channel: compute_results;
 *         requires: capability("network.reliable");
 *     };
 *
 * The grammar does not prescribe which properties are mandatory.
 *
 * Completeness and compatibility are semantic responsibilities.
 * ============================================================================
 */

networkResponseDeclaration
    : attribute*
      networkResponseMarker
      identifier
      networkResponseBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL RESPONSE MARKER
 * ============================================================================
 *
 * The current canonical lexer does not require a dedicated RESPONSE token
 * for this networking construct.
 *
 * Keeping this as an identifier:
 *
 *     - preserves lexical authority;
 *     - avoids a second keyword registry;
 *     - remains compatible with the current parser hierarchy;
 *     - allows future lexical reservation through the normal compatibility
 *       process without changing the semantic response model.
 * ============================================================================
 */

networkResponseMarker
    : identifier
    ;


/*
 * ============================================================================
 * RESPONSE BODY
 * ============================================================================
 *
 * A response body contains zero or more members.
 *
 * No language-level cardinality limit is imposed.
 * ============================================================================
 */

networkResponseBody
    : LBRACE
      networkResponseMember*
      RBRACE
    ;


/*
 * ============================================================================
 * RESPONSE MEMBER
 * ============================================================================
 *
 * There is deliberately ONE canonical property structure plus a generic
 * nested-block structure.
 *
 * This avoids creating many parser alternatives that all match:
 *
 *     identifier COLON expression SEMI
 *
 * Semantic analysis determines the meaning of the property name.
 * ============================================================================
 */

networkResponseMember
    : attribute*
      networkResponseProperty
    | attribute*
      networkResponseNestedBlock
    ;


/*
 * ============================================================================
 * RESPONSE PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     key: expression;
 *
 * Examples:
 *
 *     request: api::GetUser;
 *
 *     message: User;
 *
 *     payload: User;
 *
 *     source: user_service;
 *
 *     destination: client;
 *
 *     protocol: network::request_response;
 *
 *     channel: user_results;
 *
 *     status: success;
 *
 *     outcome: accepted;
 *
 *     error: none;
 *
 *     requires: capability("network.reliable");
 *
 *     prefers: locality::near;
 *
 *     constraint: latency < maximum_latency;
 *
 * Qualified property names provide open-world extension:
 *
 *     network::timeout: timeout_value;
 *
 *     vendor::feature: feature_value;
 *
 *     application::metadata: metadata_value;
 * ============================================================================
 */

networkResponseProperty
    : networkResponsePropertyName
      COLON
      networkResponsePropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * RESPONSE PROPERTY NAME
 * ============================================================================
 */

networkResponsePropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * RESPONSE PROPERTY VALUE
 * ============================================================================
 *
 * Values use the canonical expression grammar.
 *
 * Networking responses therefore do not create a second:
 *
 *     - literal grammar;
 *     - arithmetic grammar;
 *     - boolean grammar;
 *     - collection grammar;
 *     - call grammar;
 *     - comparison grammar.
 * ============================================================================
 */

networkResponsePropertyValue
    : expression
    ;


/*
 * ============================================================================
 * RESPONSE NESTED BLOCK
 * ============================================================================
 *
 * Nested blocks support structured response configuration without creating
 * one new parser grammar for every future networking concept.
 *
 * Example:
 *
 *     response ComputeResult {
 *         error {
 *             retryable: true;
 *             category: transient;
 *         }
 *
 *         security {
 *             policy: security::authenticated;
 *         }
 *     }
 *
 * Semantic analysis decides whether a nested block is legal.
 * ============================================================================
 */

networkResponseNestedBlock
    : networkResponseNestedBlockName
      LBRACE
      networkResponseNestedMember*
      RBRACE
      SEMI?
    ;


networkResponseNestedBlockName
    : qualifiedName
    ;


networkResponseNestedMember
    : attribute*
      networkResponseProperty
    | attribute*
      networkResponseNestedBlock
    ;


/*
 * ============================================================================
 * STANDARD SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * These names are documented semantic vocabulary.
 *
 * They are NOT lexer keywords and are NOT closed parser alternatives.
 *
 * Common response properties include:
 *
 *     request
 *     message
 *     payload
 *     source
 *     destination
 *     channel
 *     protocol
 *     direction
 *     status
 *     outcome
 *     error
 *     errors
 *     metadata
 *     headers
 *     body
 *     result
 *     requires
 *     constraint
 *     constraints
 *     prefers
 *     preference
 *     capability
 *     capabilities
 *     security
 *     policy
 *     reliability
 *     delivery
 *     ordering
 *     latency
 *     availability
 *     durability
 *     priority
 *     deadline
 *     timeout
 *
 * Semantic specifications may standardize these names without making them
 * parser keywords.
 *
 * Future names remain possible without modifying this grammar.
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUEST REFERENCE
 * ============================================================================
 *
 * A response can identify the request contract it answers.
 *
 * Example:
 *
 *     request: api::GetUser;
 *
 * This rule performs no lookup.
 *
 * Semantic analysis determines whether the name denotes a reusable request
 * contract.
 * ============================================================================
 */

networkResponseRequestReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * MESSAGE REFERENCE
 * ============================================================================
 *
 * Message declarations remain owned by:
 *
 *     grammar/networking/messages.g4
 *
 * Data schemas remain owned by:
 *
 *     grammar/data/
 *
 * This grammar only provides the syntactic reference boundary.
 * ============================================================================
 */

networkResponseMessageReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCE
 * ============================================================================
 *
 * Channel declarations remain owned by:
 *
 *     grammar/networking/channels.g4
 *
 * This grammar only references them.
 * ============================================================================
 */

networkResponseChannelReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCE
 * ============================================================================
 *
 * Protocol declarations remain owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * This grammar only references them.
 * ============================================================================
 */

networkResponseProtocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * RESPONSE REFERENCE
 * ============================================================================
 *
 * Reusable response references are intentionally separate from response
 * declarations.
 *
 * Example:
 *
 *     response: api::GetUserResult;
 *
 * This rule does not resolve the symbol.
 * ============================================================================
 */

networkResponseReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * RESPONSE REFERENCE LIST
 * ============================================================================
 *
 * No finite language-level number of response references is imposed.
 * ============================================================================
 */

networkResponseReferenceList
    : networkResponseReference
      (COMMA networkResponseReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL RESPONSE REFERENCE LIST
 * ============================================================================
 */

optionalNetworkResponseReferenceList
    : networkResponseReferenceList?
    ;


/*
 * ============================================================================
 * RESPONSE DECLARATION LIST
 * ============================================================================
 */

networkResponseDeclarationList
    : networkResponseDeclaration*
    ;


/*
 * ============================================================================
 * RESPONSE PROPERTY LIST
 * ============================================================================
 */

networkResponsePropertyList
    : networkResponseProperty*
    ;


/*
 * ============================================================================
 * RESPONSE ITEM
 * ============================================================================
 *
 * A response item can be either:
 *
 *     - a reusable response declaration;
 *     - a reference to an already declared response.
 *
 * This rule is intentionally not the networking-domain root.
 * ============================================================================
 */

networkResponseItem
    : networkResponseDeclaration
    | networkResponseReference
    ;


/*
 * ============================================================================
 * RESPONSE EXPRESSION
 * ============================================================================
 *
 * This wrapper provides a stable networking-domain boundary while reusing
 * canonical name syntax.
 *
 * It does not create a second expression language.
 * ============================================================================
 */

networkResponseExpression
    : networkResponseReference
    ;


/*
 * ============================================================================
 * RESPONSE TYPE ANNOTATION
 * ============================================================================
 *
 * A response may carry a semantic type annotation as a property-shaped
 * construct.
 *
 * Example:
 *
 *     payload_type: Result<User>;
 *
 * The type itself belongs to the canonical type grammar.
 *
 * This rule is reusable by downstream grammar components when they require a
 * syntactically explicit response type annotation.
 * ============================================================================
 */

networkResponseTypeAnnotation
    : networkResponsePropertyName
      COLON
      typeExpression
      SEMI
    ;


/*
 * ============================================================================
 * RESPONSE RELATIONSHIP HELPERS
 * ============================================================================
 *
 * These are syntactic helpers only.
 * ============================================================================
 */

networkResponseRequestName
    : qualifiedName
    ;


networkResponseMessageName
    : qualifiedName
    ;


networkResponseChannelName
    : qualifiedName
    ;


networkResponseProtocolName
    : qualifiedName
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must map into the EXISTING domain-neutral frontend AST.
 *
 * It MUST NOT create a second networking-specific AST hierarchy merely
 * because responses are a networking feature.
 *
 * Conceptually:
 *
 *     networkResponseDeclaration
 *         ->
 *     existing declaration representation
 *
 *     networkResponseProperty
 *         ->
 *     existing property/member representation
 *
 *     networkResponseNestedBlock
 *         ->
 *     existing nested-block/member representation
 *
 * The AST must preserve:
 *
 *     - source span;
 *     - response name;
 *     - declaration attributes;
 *     - ordered members;
 *     - property names;
 *     - property expressions;
 *     - nested structure;
 *     - exact source ordering.
 *
 * The parser MUST NOT:
 *
 *     - resolve request names;
 *     - resolve message names;
 *     - resolve channel names;
 *     - resolve protocol names;
 *     - resolve endpoints;
 *     - evaluate expressions;
 *     - validate capabilities;
 *     - select resources;
 *     - select hardware;
 *     - perform routing;
 *     - construct IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - validating the response declaration name;
 *     - symbol resolution;
 *     - duplicate property detection where required;
 *     - request/response compatibility;
 *     - message/type compatibility;
 *     - source/destination compatibility;
 *     - protocol compatibility;
 *     - channel compatibility;
 *     - response direction;
 *     - status/outcome semantics;
 *     - error semantics;
 *     - capability requirements;
 *     - resource requirements;
 *     - security requirements;
 *     - constraint satisfiability;
 *     - preference handling;
 *     - portability analysis;
 *     - version compatibility;
 *     - dialect validation.
 *
 * An unknown property is syntactically legal because the property namespace
 * is open.
 *
 * It is not automatically semantically valid.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct IR dependency.
 *
 * The lowering path is:
 *
 *     source
 *       |
 *       v
 *     NetworkingResponses
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic response contract
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> networking representation
 *       +--> distributed representation
 *       +--> classical representation
 *       +--> hardware communication representation
 *       +--> quantum-related semantic metadata
 *       |
 *       v
 *     downstream lowering
 *
 * If a response participates in quantum computation, the quantum computation
 * itself continues through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar MUST NOT introduce:
 *
 *     ResponseIR
 *     NetworkResponseIR
 *     QuantumResponseIR
 *     PhysicalResponseIR
 *
 * merely to represent syntax.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler may use semantic response information for:
 *
 *     - communication lowering;
 *     - serialization selection;
 *     - transport selection;
 *     - protocol negotiation;
 *     - routing;
 *     - placement;
 *     - scheduling;
 *     - optimization;
 *     - distributed execution;
 *     - hardware communication;
 *     - accelerator communication;
 *     - quantum/classical communication.
 *
 * Those decisions are downstream.
 *
 * The source response contract must remain portable.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may eventually realize a response through:
 *
 *     - function return;
 *     - local message passing;
 *     - IPC;
 *     - shared memory;
 *     - network transport;
 *     - distributed communication;
 *     - accelerator fabric;
 *     - hardware interconnect;
 *     - quantum/classical communication.
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Response properties may express:
 *
 *     requires: memory >= required_memory;
 *
 *     requires: capability("network.reliable");
 *
 *     requires: capability("network.streaming");
 *
 *     requires: topology(required_topology);
 *
 *     constraint: latency < maximum_latency;
 *
 * These are expressions.
 *
 * Semantic/resource analysis determines:
 *
 *     - whether the expressions are valid;
 *     - whether the requirements are satisfiable;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - whether the selected realization satisfies them.
 *
 * The grammar imposes no physical limit.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Response properties may express security intent:
 *
 *     security: security::authenticated;
 *
 *     requires: capability("secure.communication");
 *
 *     policy: security::confidential;
 *
 * The grammar does NOT implement:
 *
 *     - encryption;
 *     - decryption;
 *     - signatures;
 *     - key management;
 *     - authentication;
 *     - authorization;
 *     - certificate validation;
 *     - trust evaluation.
 *
 * Those responsibilities belong to downstream security semantics/runtime.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * Responses may carry results originating from:
 *
 *     - classical computation;
 *     - quantum computation;
 *     - hybrid computation;
 *     - AI/ML computation;
 *     - HDL/hardware computation;
 *     - distributed computation;
 *     - future computational domains.
 *
 * This grammar deliberately does not create separate variants such as:
 *
 *     QuantumResponse
 *     GPUResponse
 *     FPGAResponse
 *     AIResponse
 *
 * A response remains a generic communication contract.
 *
 * Quantum computation continues through:
 *
 *     domain-neutral AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * No second quantum IR is created.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *
 * This file does not define:
 *
 *     node
 *     worker
 *     cluster
 *     replica
 *     partition
 *     consensus
 *     placement
 *     distributed scheduler
 *
 * Such information may be expressed through properties and interpreted by
 * semantic/distributed analysis.
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * Message schemas remain owned by:
 *
 *     grammar/networking/messages.g4
 *
 * General data schemas remain owned by:
 *
 *     grammar/data/
 *
 * Serialization and wire representation are downstream concerns.
 *
 * ============================================================================
 * ENDPOINT / ADDRESS INTEGRATION
 * ============================================================================
 *
 * Endpoint declarations remain owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * Address declarations remain owned by:
 *
 *     grammar/networking/addresses.g4
 *
 * A response can reference logical endpoints through expressions or qualified
 * names without embedding an address grammar here.
 *
 * ============================================================================
 * CHANNEL / PROTOCOL INTEGRATION
 * ============================================================================
 *
 * Channels are owned by:
 *
 *     grammar/networking/channels.g4
 *
 * Protocols are owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * A response may reference either through properties:
 *
 *     channel: result_channel;
 *
 *     protocol: network::reliable;
 *
 * No protocol implementation is embedded in this grammar.
 *
 * ============================================================================
 * SERVICE INTEGRATION
 * ============================================================================
 *
 * Service-local response declarations remain in:
 *
 *     grammar/networking/services.g4
 *
 * A service may semantically reference a reusable response declared through
 * this grammar.
 *
 * The service grammar should not duplicate this grammar's reusable response
 * body.
 *
 * ============================================================================
 * NETWORKING AGGREGATE INTEGRATION
 * ============================================================================
 *
 * The networking aggregate:
 *
 *     grammar/networking/networking.g4
 *
 * should import:
 *
 *     NetworkingResponses
 *
 * and expose:
 *
 *     networkingResponse
 *         : networkResponseConstruct
 *         ;
 *
 * The aggregate should also include:
 *
 *     networkResponseConstruct
 *
 * in:
 *
 *     networkingConstruct
 *
 * This file itself does not modify the aggregate and therefore remains
 * independently completable.
 *
 * ============================================================================
 * CANONICAL PARSER INTEGRATION
 * ============================================================================
 *
 * The wider canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * should consume the networking aggregate rather than importing this grammar
 * directly.
 *
 * Desired dependency direction:
 *
 *     ZamaniParser
 *          |
 *          v
 *     Networking
 *          |
 *          v
 *     NetworkingResponses
 *
 * NOT:
 *
 *     ZamaniParser
 *          |
 *          +--> Networking
 *          |
 *          +--> NetworkingResponses
 *
 * because the latter creates parallel composition paths.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token stream;
 *     - grammar version;
 *     - imported grammar definitions;
 *     - explicitly selected language/dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware;
 *     - CPU count;
 *     - GPU availability;
 *     - QPU availability;
 *     - network availability;
 *     - filesystem state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime state;
 *     - deployment topology.
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no runtime callbacks;
 *     - no hardware discovery.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser errors are syntax errors.
 *
 * Examples:
 *
 *     response;
 *
 *     response 123;
 *
 *     response Result {
 *         message:
 *     };
 *
 *     response Result {
 *         broken
 *     };
 *
 *     response Result {
 *         message: Value
 *         source: service;
 *     };
 *
 * Semantic errors include:
 *
 *     - unknown request;
 *     - unknown message;
 *     - unknown endpoint;
 *     - unknown channel;
 *     - unknown protocol;
 *     - incompatible request/response relationship;
 *     - invalid response type;
 *     - unsatisfied capability;
 *     - impossible resource requirement;
 *     - contradictory constraints.
 *
 * Those belong to semantic analysis, not parser recovery.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - executes no source code;
 *     - performs no I/O;
 *     - evaluates no expressions;
 *     - performs no networking;
 *     - performs no hardware discovery;
 *     - performs no cryptography;
 *     - accesses no secrets;
 *     - requires no unsafe Rust.
 *
 * Hostile-input resource protection belongs to explicit parser/compiler
 * resource policy and must not be disguised as a language-level grammar
 * capacity.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is a new independent networking component.
 *
 * It must follow the repository's compatibility/versioning policy.
 *
 * Existing service-local response syntax in services.g4 remains valid.
 *
 * Introducing this reusable response grammar does not require renaming:
 *
 *     services.g4
 *     requests.g4
 *     channels.g4
 *     networking.g4
 *     Zamani.g4
 *
 * If the language eventually reserves `response` lexically, the migration
 * must occur through the canonical lexer/specification compatibility process.
 *
 * The semantic response contract remains unchanged.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive examples:
 *
 *     response GetUserResult;
 *
 *     response GetUserResult {
 *         request: api::GetUser;
 *         message: User;
 *     };
 *
 *     response ComputeResult {
 *         request: Compute;
 *         message: ComputeResult;
 *         source: compute_service;
 *         destination: client;
 *         protocol: network::request_response;
 *     };
 *
 *     response QuantumResult {
 *         request: QuantumMeasurement;
 *         message: QuantumMeasurement;
 *         requires: capability("quantum.measurement");
 *     };
 *
 *     response DistributedResult {
 *         request: DistributedCompute;
 *         message: ComputeResult;
 *         constraint: latency < required_latency;
 *         prefers: locality::near;
 *     };
 *
 *     response ExtendedResult {
 *         network::timeout: timeout_value;
 *         vendor::transport: vendor::future_transport;
 *         application::metadata: metadata_value;
 *     };
 *
 *     response StructuredResult {
 *         message: Result;
 *
 *         error {
 *             retryable: true;
 *             category: transient;
 *         }
 *
 *         security {
 *             policy: security::authenticated;
 *         }
 *     };
 *
 *     @portable
 *     response PortableResult {
 *         message: Result;
 *     };
 *
 * Negative examples:
 *
 *     response;
 *
 *     response 123;
 *
 *     response Result {
 *         message;
 *     };
 *
 *     response Result {
 *         message:
 *     };
 *
 *     response Result {
 *         broken
 *     };
 *
 *     response Result {
 *         message: Value
 *         source: service;
 *     };
 *
 * Boundary/scalability tests:
 *
 *     - empty response declaration;
 *     - one response property;
 *     - many response properties;
 *     - many nested blocks;
 *     - deeply qualified property names;
 *     - deeply qualified request references;
 *     - large expressions;
 *     - large response contracts;
 *     - many reusable response declarations;
 *     - very small programs;
 *     - very large programs.
 *
 * Tests MUST NOT establish an artificial maximum.
 *
 * Determinism tests MUST produce equivalent parse structures for identical
 * token streams under identical grammar/version configuration.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no parser-level resource capacities.
 *
 * In particular it contains no:
 *
 *     MAX_RESPONSES
 *     MAX_RESPONSE_PROPERTIES
 *     MAX_RESPONSE_MEMBERS
 *     MAX_REQUESTS
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * It contains no physical device selection.
 *
 * It contains no fixed transport enumeration.
 *
 * It contains no fixed message-size limit.
 *
 * It contains no fixed payload-size limit.
 *
 * It contains no fixed topology.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar uses:
 *
 *     - linear member repetition;
 *     - canonical qualified names;
 *     - canonical expressions;
 *     - recursive nested blocks.
 *
 * It intentionally does not introduce semantic lookups or runtime behavior
 * into parsing.
 *
 * Performance limits for hostile or extremely large source inputs belong to
 * explicit parser/compiler resource policy rather than language semantics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is an independent parser grammar.
 *
 * [x] Its grammar name matches the file role.
 *
 * [x] It uses tokenVocab = ZamaniLexer.
 *
 * [x] It imports canonical Names.
 *
 * [x] It imports canonical Types.
 *
 * [x] It imports canonical Expressions.
 *
 * [x] It imports canonical Attributes.
 *
 * [x] It owns reusable response contracts.
 *
 * [x] It does not duplicate service-local response declarations.
 *
 * [x] It does not duplicate message schemas.
 *
 * [x] It does not duplicate endpoint syntax.
 *
 * [x] It does not duplicate address syntax.
 *
 * [x] It does not duplicate channel syntax.
 *
 * [x] It does not duplicate protocol syntax.
 *
 * [x] It does not duplicate socket syntax.
 *
 * [x] It does not duplicate distributed syntax.
 *
 * [x] It does not enumerate transport implementations.
 *
 * [x] It does not enumerate hardware.
 *
 * [x] It does not encode machine capacities.
 *
 * [x] It does not encode networking capacities.
 *
 * [x] It does not create a second expression grammar.
 *
 * [x] It does not create a second type grammar.
 *
 * [x] It does not create a second AST hierarchy.
 *
 * [x] It does not create an IR.
 *
 * [x] It does not create a second quantum IR.
 *
 * [x] It preserves source structure.
 *
 * [x] It supports attributes.
 *
 * [x] It supports nested configuration.
 *
 * [x] It supports qualified property names.
 *
 * [x] It supports reusable request relationships.
 *
 * [x] It supports reusable response references.
 *
 * [x] It is deterministic.
 *
 * [x] It contains no actions.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no unsafe Rust.
 *
 * [x] It defines the AST contract before implementation.
 *
 * [x] It defines the semantic contract before implementation.
 *
 * [x] It defines the IR boundary before implementation.
 *
 * [x] It defines compiler/runtime integration before implementation.
 *
 * [x] It defines positive/negative/boundary/scalability/determinism tests.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How is a reusable logical networking response contract represented
 *      syntactically in Zamani?"
 *
 * It does NOT answer:
 *
 *     "How is the response transmitted?"
 *
 *     "Which transport is selected?"
 *
 *     "Which machine handles it?"
 *
 *     "Which network route is selected?"
 *
 *     "Which CPU/GPU/FPGA/QPU is used?"
 *
 *     "How is the response serialized?"
 *
 *     "How is security implemented?"
 *
 *     "How is the response scheduled?"
 *
 *     "How is hardware allocated?"
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 */

parser grammar NetworkingResponses;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC CONSTRUCT
 * ============================================================================
 */

networkResponseConstruct
    : networkResponseDeclaration
    ;


/*
 * ============================================================================
 * DECLARATION
 * ============================================================================
 */

networkResponseDeclaration
    : attribute*
      networkResponseMarker
      identifier
      networkResponseBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL MARKER
 * ============================================================================
 */

networkResponseMarker
    : identifier
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 */

networkResponseBody
    : LBRACE
      networkResponseMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MEMBERS
 * ============================================================================
 */

networkResponseMember
    : attribute*
      networkResponseProperty
    | attribute*
      networkResponseNestedBlock
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 */

networkResponseProperty
    : networkResponsePropertyName
      COLON
      networkResponsePropertyValue
      SEMI
    ;


networkResponsePropertyName
    : qualifiedName
    ;


networkResponsePropertyValue
    : expression
    ;


/*
 * ============================================================================
 * NESTED BLOCK
 * ============================================================================
 */

networkResponseNestedBlock
    : networkResponseNestedBlockName
      LBRACE
      networkResponseNestedMember*
      RBRACE
      SEMI?
    ;


networkResponseNestedBlockName
    : qualifiedName
    ;


networkResponseNestedMember
    : attribute*
      networkResponseProperty
    | attribute*
      networkResponseNestedBlock
    ;


/*
 * ============================================================================
 * REUSABLE REFERENCES
 * ============================================================================
 */

networkResponseRequestReference
    : qualifiedName
    ;


networkResponseMessageReference
    : qualifiedName
    ;


networkResponseChannelReference
    : qualifiedName
    ;


networkResponseProtocolReference
    : qualifiedName
    ;


networkResponseReference
    : qualifiedName
    ;


networkResponseReferenceList
    : networkResponseReference
      (COMMA networkResponseReference)*
      COMMA?
    ;


optionalNetworkResponseReferenceList
    : networkResponseReferenceList?
    ;


/*
 * ============================================================================
 * LISTS
 * ============================================================================
 */

networkResponseDeclarationList
    : networkResponseDeclaration*
    ;


networkResponsePropertyList
    : networkResponseProperty*
    ;


/*
 * ============================================================================
 * ITEMS
 * ============================================================================
 */

networkResponseItem
    : networkResponseDeclaration
    | networkResponseReference
    ;


networkResponseExpression
    : networkResponseReference
    ;


/*
 * ============================================================================
 * TYPE ANNOTATION
 * ============================================================================
 */

networkResponseTypeAnnotation
    : networkResponsePropertyName
      COLON
      typeExpression
      SEMI
    ;


/*
 * ============================================================================
 * STABLE NAME HELPERS
 * ============================================================================
 */

networkResponseRequestName
    : qualifiedName
    ;


networkResponseMessageName
    : qualifiedName
    ;


networkResponseChannelName
    : qualifiedName
    ;


networkResponseProtocolName
    : qualifiedName
    ;