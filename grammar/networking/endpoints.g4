/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/networking/endpoints.g4
 *
 * Grammar:
 *     Endpoints
 *
 * Status:
 *     Production networking component grammar
 *
 * Purpose:
 *     Define the canonical source-level syntax for LOGICAL NETWORKING
 *     ENDPOINTS.
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust.
 *     - No actions.
 *     - No semantic predicates.
 *     - No unsafe implementation requirement.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No target discovery.
 *     - No resource discovery.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
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
 *     Endpoints
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> names
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     networking semantic model
 *          |
 *          +--> services
 *          +--> channels
 *          +--> protocols
 *          +--> requests/responses
 *          +--> discovery
 *          +--> routing
 *          +--> sockets
 *          +--> streaming
 *          +--> distributed semantics
 *          |
 *          v
 *     target-independent planning
 *          |
 *          v
 *     routing / scheduling / placement / resilience
 *          |
 *          v
 *     runtime / ZQN / HAL / target realization
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Define the syntax of a logical communication endpoint.
 *
 * An endpoint identifies a logical communication participant or communication
 * boundary. It does not identify a physical machine resource.
 *
 * ============================================================================
 *
 * OWNS
 * -----
 *
 * This file owns:
 *
 *     - endpoint declarations;
 *     - endpoint declaration identity;
 *     - endpoint bodies;
 *     - endpoint members;
 *     - endpoint properties;
 *     - endpoint nested configuration blocks;
 *     - endpoint references;
 *     - endpoint reference lists;
 *     - endpoint-name wrappers;
 *     - endpoint configuration boundaries.
 *
 * ============================================================================
 *
 * DOES NOT OWN
 * -------------
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - qualified-name syntax;
 *     - attributes;
 *     - visibility;
 *     - expressions;
 *     - types;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - policies;
 *     - effects;
 *     - contracts;
 *     - provenance semantics;
 *     - physical addresses;
 *     - address-family syntax;
 *     - sockets;
 *     - channels;
 *     - protocols;
 *     - requests;
 *     - responses;
 *     - streams;
 *     - service discovery;
 *     - routing;
 *     - topology;
 *     - placement;
 *     - scheduling;
 *     - transport implementation;
 *     - authentication;
 *     - authorization;
 *     - cryptography;
 *     - distributed execution;
 *     - actors;
 *     - processes;
 *     - threads;
 *     - machines;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - QPUs;
 *     - physical qubits;
 *     - quantum routing;
 *     - QEC;
 *     - classical IR;
 *     - quantum::ir;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/core/core.g4
 *     grammar/expressions/expressions.g4
 *
 * Core provides:
 *
 *     identifier
 *     qualifiedName
 *     attribute
 *     visibility
 *     canonical source foundations
 *
 * Expressions provides:
 *
 *     expression
 *
 * ============================================================================
 *
 * EXPORTS
 * -------
 *
 * Public networking composition rules:
 *
 *     endpointDeclaration
 *     endpointReference
 *     endpointReferenceList
 *     endpointName
 *     endpointNameList
 *     endpointConfiguration
 *
 * ============================================================================
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/networking/networking.g4
 *     grammar/networking/services.g4
 *     grammar/networking/service-discovery.g4
 *     grammar/networking/routing.g4
 *     grammar/networking/channels.g4
 *     grammar/networking/sockets.g4
 *     semantic networking analysis
 *     frontend AST construction
 *
 * No consumer may reinterpret endpoint syntax as a physical allocation
 * mechanism.
 *
 * ============================================================================
 *
 * AST_OWNER
 * ---------
 *
 * Existing domain-neutral Zamani frontend AST.
 *
 * This grammar defines parser structure only.
 *
 * The AST must preserve source spans for:
 *
 *     - attributes;
 *     - visibility;
 *     - endpoint marker;
 *     - endpoint name;
 *     - endpoint body;
 *     - every endpoint member;
 *     - every property key;
 *     - every property value;
 *     - every nested block;
 *     - every endpoint reference.
 *
 * ============================================================================
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Networking semantic analysis.
 *
 * Semantic analysis is responsible for:
 *
 *     - endpoint declaration uniqueness;
 *     - name resolution;
 *     - endpoint reference resolution;
 *     - property-schema validation;
 *     - endpoint/service compatibility;
 *     - capability checking;
 *     - resource checking;
 *     - effect checking;
 *     - policy checking;
 *     - security validation;
 *     - portability validation;
 *     - endpoint/channel compatibility;
 *     - endpoint/protocol compatibility;
 *     - endpoint/request/response compatibility.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * -------------
 *
 * Endpoint property values use the canonical expression grammar.
 *
 * This file does not define a networking-specific type system.
 *
 * When an endpoint property semantically represents a type, the downstream
 * semantic layer resolves it against the canonical Zamani type system.
 *
 * ============================================================================
 *
 * EFFECT CONTRACT
 * ---------------
 *
 * Endpoint declarations do not intrinsically execute effects.
 *
 * Endpoint properties MAY describe effect requirements through ordinary
 * canonical expressions or semantic properties.
 *
 * Actual effects such as:
 *
 *     network
 *     io
 *     foreign
 *     native
 *     distributed
 *     measurement
 *
 * are determined by semantic analysis and execution planning.
 *
 * ============================================================================
 *
 * CAPABILITY CONTRACT
 * -------------------
 *
 * Endpoint properties may express capability requirements or associations.
 *
 * Example:
 *
 *     endpoint compute::worker {
 *         requires: capability("network.reliable");
 *     }
 *
 * The grammar parses the expression.
 *
 * Capability resolution occurs downstream through the canonical capability
 * subsystem.
 *
 * This file does not enumerate network capabilities.
 *
 * ============================================================================
 *
 * RESOURCE CONTRACT
 * -----------------
 *
 * Endpoint declarations may carry abstract resource requirements through
 * canonical expressions.
 *
 * Examples:
 *
 *     requires: memory >= required_memory;
 *     requires: capability("network.reliable");
 *
 * No physical capacity is encoded here.
 *
 * ============================================================================
 *
 * CONTRACT CONTRACT
 * -----------------
 *
 * Endpoint syntax does not implement a second contract language.
 *
 * Contract expressions, if attached through endpoint properties, are consumed
 * by the canonical validation/contract subsystem.
 *
 * ============================================================================
 *
 * POLICY CONTRACT
 * ---------------
 *
 * Endpoint policy information is represented structurally through endpoint
 * properties or nested configuration.
 *
 * Policy semantics belong to the canonical policy subsystem.
 *
 * This file does not implement authorization, trust, access control, or
 * policy evaluation.
 *
 * ============================================================================
 *
 * PROVENANCE CONTRACT
 * -------------------
 *
 * Every endpoint declaration and member must remain source-traceable.
 *
 * The frontend must preserve source spans so that semantic and compiler
 * provenance can record:
 *
 *     source
 *       ->
 *     endpoint declaration
 *       ->
 *     semantic interpretation
 *       ->
 *     routing / planning
 *       ->
 *     realization
 *
 * ============================================================================
 *
 * IR CONTRACT
 * -----------
 *
 * This file creates NO IR.
 *
 * Endpoint syntax lowers into the canonical networking semantic model.
 *
 * From there it may participate in:
 *
 *     networking representation
 *     distributed representation
 *     classical representation
 *     hardware communication intent
 *     execution planning
 *
 * If networking is associated with a quantum computation, the established
 * quantum semantic pipeline remains:
 *
 *     AST
 *       ->
 *     quantum semantics
 *       ->
 *     quantum::ir
 *
 * This grammar never creates a second quantum IR.
 *
 * ============================================================================
 *
 * QUANTUM BOUNDARY
 * ----------------
 *
 * Networking endpoints may participate in quantum-classical systems, but this
 * grammar does not own:
 *
 *     qubits;
 *     quantum operations;
 *     quantum states;
 *     measurement;
 *     physical QPU topology;
 *     quantum routing;
 *     QEC.
 *
 * A quantum-related endpoint is represented through ordinary logical names,
 * properties, capabilities, and semantic references.
 *
 * ============================================================================
 *
 * HDL BOUNDARY
 * ------------
 *
 * HDL/hardware endpoint intent may be attached through logical properties.
 *
 * This grammar does not own:
 *
 *     pins;
 *     wires;
 *     physical ports;
 *     clocks;
 *     timing implementation;
 *     FPGA routing;
 *     ASIC placement;
 *     physical interconnect.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 *
 * BACKEND BOUNDARY
 * ----------------
 *
 * The backend may realize an endpoint as:
 *
 *     - an in-process communication boundary;
 *     - an actor endpoint;
 *     - a task endpoint;
 *     - an IPC endpoint;
 *     - a service endpoint;
 *     - a distributed endpoint;
 *     - an accelerator communication boundary;
 *     - a simulator endpoint;
 *     - a quantum-control boundary;
 *     - a future computational substrate.
 *
 * The source declaration does not select any of these realizations.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ------------------
 *
 * Endpoint syntax must remain independent of target scale.
 *
 * The same endpoint source must be representable on:
 *
 *     tiny systems
 *     embedded systems
 *     single CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * subject to semantic feasibility and available resources.
 *
 * There are no grammar-level limits on:
 *
 *     endpoint count;
 *     endpoint-member count;
 *     nested configuration depth;
 *     reference count;
 *     property count;
 *     logical network size;
 *     address count;
 *     service count;
 *     connection count.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * -----------------
 *
 * This file contains no universal physical limits.
 *
 * It must never introduce:
 *
 *     MAX_ENDPOINTS
 *     MAX_ENDPOINT_PROPERTIES
 *     MAX_ENDPOINT_MEMBERS
 *     MAX_ENDPOINT_REFERENCES
 *     MAX_ADDRESSES
 *     MAX_CONNECTIONS
 *     MAX_SERVICES
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_DEVICES
 *
 * It also does not enumerate:
 *
 *     CPU models;
 *     GPU models;
 *     FPGA families;
 *     ASIC technologies;
 *     QPU vendors;
 *     network vendors;
 *     cloud providers;
 *     physical interfaces;
 *     transport implementations.
 *
 * ============================================================================
 *
 * OPEN-WORLD CONTRACT
 * -------------------
 *
 * Endpoint properties use ordinary identifiers rather than a closed list of
 * networking property keywords.
 *
 * Therefore future properties can be introduced semantically without editing
 * this grammar.
 *
 * Examples of valid property names include:
 *
 *     role
 *     address
 *     protocol
 *     channel
 *     service
 *     requires
 *     capability
 *     locality
 *     security
 *     metadata
 *     topology
 *     policy
 *     reliability
 *     provenance
 *     custom_property
 *
 * These names do not acquire semantics merely by parsing.
 *
 * The semantic layer decides which properties are:
 *
 *     standard;
 *     dialect-defined;
 *     capability-defined;
 *     application-defined;
 *     invalid.
 *
 * ============================================================================
 *
 * DETERMINISM CONTRACT
 * --------------------
 *
 * Parsing depends only on:
 *
 *     source token stream;
 *     grammar version;
 *     parser configuration.
 *
 * Parsing must not depend on:
 *
 *     network state;
 *     hardware availability;
 *     filesystem state;
 *     runtime state;
 *     wall-clock time;
 *     randomness;
 *     resource availability;
 *     target selection.
 *
 * ============================================================================
 *
 * DIAGNOSTIC CONTRACT
 * -------------------
 *
 * Parser diagnostics should distinguish:
 *
 *     - missing ENDPOINT keyword;
 *     - missing endpoint name;
 *     - malformed qualified name;
 *     - malformed endpoint body;
 *     - malformed property;
 *     - malformed nested configuration;
 *     - malformed endpoint reference.
 *
 * Semantic diagnostics should distinguish:
 *
 *     - duplicate endpoint;
 *     - unresolved endpoint reference;
 *     - invalid endpoint property;
 *     - incompatible endpoint/service;
 *     - incompatible endpoint/channel;
 *     - incompatible endpoint/protocol;
 *     - unsatisfied capability;
 *     - unsatisfied resource requirement;
 *     - invalid policy;
 *     - invalid security requirement;
 *     - invalid portability requirement.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * -------------
 *
 * POSITIVE:
 *
 *     endpoint compute::worker;
 *
 *     endpoint compute::worker {
 *         role: producer;
 *     }
 *
 *     public endpoint telemetry::source {
 *         role: producer;
 *         service: telemetry::Service;
 *         protocol: network::reliable;
 *     }
 *
 *     endpoint quantum::control {
 *         capability: capability("quantum.measurement");
 *     }
 *
 *     endpoint compute::worker {
 *         policy {
 *             reliability: required;
 *             locality: preferred;
 *         }
 *     }
 *
 *     endpoint::not-valid;
 *
 * The last example is intentionally NOT valid because `endpoint` is a
 * reserved lexical token and therefore cannot be an ordinary identifier in
 * that position.
 *
 * NEGATIVE:
 *
 *     endpoint;
 *
 *     endpoint {
 *     }
 *
 *     endpoint compute::worker {
 *         role
 *     }
 *
 *     endpoint compute::worker {
 *         role:
 *     }
 *
 *     endpoint compute::worker {
 *         role: value
 *     }
 *
 * The final example is invalid because endpoint properties require their
 * canonical statement terminator.
 *
 * BOUNDARY:
 *
 *     endpoint a;
 *     endpoint a::b;
 *     endpoint a::b::c;
 *
 *     endpoint service::producer {
 *         service: services::Telemetry;
 *         channel: channels::Results;
 *         protocol: protocols::Reliable;
 *     }
 *
 * CROSS-DOMAIN:
 *
 *     endpoint classical::worker {
 *         capability: capability("cpu.compute");
 *     }
 *
 *     endpoint accelerator::worker {
 *         capability: capability("gpu.compute");
 *     }
 *
 *     endpoint quantum::worker {
 *         capability: capability("quantum.measurement");
 *     }
 *
 *     endpoint hybrid::worker {
 *         requires: capability("quantum.measurement");
 *         requires_memory: required_memory;
 *     }
 *
 * SCALABILITY:
 *
 *     - arbitrarily many endpoint declarations;
 *     - arbitrarily many endpoint members;
 *     - arbitrarily deep qualified names subject to implementation resources;
 *     - arbitrarily many references;
 *     - arbitrarily many nested configuration members;
 *     - no language-level machine capacity.
 *
 * DETERMINISM:
 *
 *     Identical token streams produce equivalent parse structures.
 *
 * COMPATIBILITY:
 *
 *     The canonical declaration spelling is:
 *
 *         endpoint qualified::name { ... }
 *
 *     or:
 *
 *         endpoint qualified::name;
 *
 *     Because ENDPOINT is now consumed as the canonical lexical token, an
 *     older implementation that treated the marker as an arbitrary identifier
 *     must migrate to the canonical tokenized form.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * --------------------
 *
 * `networking.g4` must consume:
 *
 *     endpointDeclaration
 *
 * through:
 *
 *     networkingEndpoint
 *         : endpointDeclaration
 *         ;
 *
 * Services consume endpoint references through their existing:
 *
 *     networkServiceEndpointReference
 *
 * and do NOT import or duplicate endpoint declaration syntax.
 *
 * Channels, protocols, routes, sockets and discovery may consume:
 *
 *     endpointReference
 *
 * but must perform resolution semantically.
 *
 * Service discovery owns discovery.
 *
 * Routing owns route realization.
 *
 * Sockets own socket semantics.
 *
 * Channels own channel semantics.
 *
 * Protocols own protocol semantics.
 *
 * Addresses own address semantics.
 *
 * This file remains the logical endpoint boundary between them.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * -------------------
 *
 * This file is DONE when:
 *
 *     [x] The grammar is a parser grammar.
 *     [x] tokenVocab is ZamaniLexer.
 *     [x] ENDPOINT is the canonical endpoint marker.
 *     [x] Endpoint names use qualifiedName.
 *     [x] Attributes use the canonical Core layer.
 *     [x] Visibility uses the canonical Core layer.
 *     [x] Endpoint properties use canonical expressions.
 *     [x] Endpoint nested blocks are open-ended.
 *     [x] Endpoint references use qualifiedName.
 *     [x] No endpoint lookup occurs in parsing.
 *     [x] No physical networking semantics are encoded.
 *     [x] No hardware topology is encoded.
 *     [x] No resource limits are encoded.
 *     [x] No protocol catalog is encoded.
 *     [x] No transport catalog is encoded.
 *     [x] No second policy system is encoded.
 *     [x] No second capability system is encoded.
 *     [x] No second type system is encoded.
 *     [x] No second expression system is encoded.
 *     [x] No IR is created.
 *     [x] No runtime behavior is created.
 *     [x] No embedded Rust exists.
 *     [x] No unsafe implementation is required.
 *     [x] Parsing is deterministic.
 *     [x] Source structure can be preserved for provenance.
 *     [x] The grammar is open to future networking properties.
 *
 * Repository-level verification still required:
 *
 *     [ ] ANTLR generation succeeds.
 *     [ ] ZamaniParser generation succeeds.
 *     [ ] networking.g4 composition succeeds.
 *     [ ] endpoint tests pass.
 *     [ ] networking integration tests pass.
 *     [ ] AST construction tests pass.
 *     [ ] semantic endpoint tests pass.
 *     [ ] negative tests pass.
 *     [ ] cross-domain tests pass.
 *     [ ] scalability tests pass.
 *     [ ] determinism tests pass.
 *     [ ] compatibility tests pass.
 *
 * ============================================================================
 */

parser grammar Endpoints;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC ENDPOINT CONSTRUCT
 * ============================================================================
 *
 * The aggregate networking grammar consumes endpointDeclaration.
 *
 * Endpoint references are deliberately separate from declarations so that
 * services, channels, protocols, discovery and routing can reference logical
 * endpoints without accidentally creating declarations.
 * ============================================================================
 */

endpointDeclaration
    : attribute*
      visibility?
      ENDPOINT
      qualifiedName
      endpointBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * ENDPOINT BODY
 * ============================================================================
 *
 * A body is mandatory for the block form.
 *
 * The declaration rule therefore supports exactly:
 *
 *     endpoint name { ... }
 *
 * and:
 *
 *     endpoint name { ... };
 *
 * A declaration without a body uses the explicit declaration terminator:
 *
 *     endpoint name;
 *
 * This prevents an incomplete:
 *
 *     endpoint name
 *
 * from being accepted as a complete declaration.
 * ============================================================================
 */

endpointBody
    : LBRACE
      endpointMember*
      RBRACE
    ;


/*
 * ============================================================================
 * EMPTY / FORWARD DECLARATION
 * ============================================================================
 *
 * Separate rule retained for parser composition where a declaration is
 * intentionally body-less.
 * ============================================================================
 */

endpointForwardDeclaration
    : attribute*
      visibility?
      ENDPOINT
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * ENDPOINT DECLARATION ADAPTER
 * ============================================================================
 *
 * Public declaration accepts either a complete body or an explicit forward
 * declaration.
 * ============================================================================
 */

endpointDeclaration
    : attribute*
      visibility?
      ENDPOINT
      qualifiedName
      (
          endpointBody SEMICOLON?
        | SEMICOLON
      )
    ;


/*
 * ============================================================================
 * ENDPOINT MEMBER
 * ============================================================================
 *
 * Exactly two structural forms exist:
 *
 *     property:
 *         name: expression;
 *
 *     nested configuration:
 *         name { ... }
 *
 * This avoids a large closed keyword catalogue while retaining deterministic
 * structural parsing.
 * ============================================================================
 */

endpointMember
    : endpointProperty
    | endpointNestedBlock
    ;


/*
 * ============================================================================
 * ENDPOINT PROPERTY
 * ============================================================================
 */

endpointProperty
    : endpointPropertyKey
      COLON
      endpointPropertyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * ENDPOINT PROPERTY KEY
 * ============================================================================
 */

endpointPropertyKey
    : identifier
    ;


/*
 * ============================================================================
 * ENDPOINT PROPERTY VALUE
 * ============================================================================
 *
 * All values use the canonical Zamani expression system.
 * ============================================================================
 */

endpointPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * ENDPOINT NESTED BLOCK
 * ============================================================================
 *
 * Nested configuration remains structurally generic.
 *
 * The semantic subsystem determines whether a nested key represents:
 *
 *     policy;
 *     security;
 *     metadata;
 *     capabilities;
 *     requirements;
 *     transport intent;
 *     service binding;
 *     channel binding;
 *     protocol intent;
 *     vendor/dialect extension;
 *     application metadata;
 *     another supported endpoint configuration.
 *
 * No such semantic catalog is encoded here.
 * ============================================================================
 */

endpointNestedBlock
    : endpointPropertyKey
      endpointBody
      SEMICOLON?
    ;


/*
 * ============================================================================
 * ENDPOINT REFERENCE
 * ============================================================================
 *
 * Symbolic only.
 *
 * This rule performs NO:
 *
 *     - lookup;
 *     - discovery;
 *     - routing;
 *     - address resolution;
 *     - placement;
 *     - transport selection.
 * ============================================================================
 */

endpointReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ENDPOINT REFERENCE LIST
 * ============================================================================
 *
 * Unbounded by language design.
 * ============================================================================
 */

endpointReferenceList
    : endpointReference
      (COMMA endpointReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * ENDPOINT NAME
 * ============================================================================
 *
 * Explicit semantic boundary for contexts that need to identify an endpoint
 * name while retaining the canonical qualified-name syntax.
 * ============================================================================
 */

endpointName
    : qualifiedName
    ;


/*
 * ============================================================================
 * ENDPOINT NAME LIST
 * ============================================================================
 */

endpointNameList
    : endpointName
      (COMMA endpointName)*
      COMMA?
    ;


/*
 * ============================================================================
 * ENDPOINT REFERENCE EXPRESSION
 * ============================================================================
 *
 * Networking consumers may use this boundary when they need to distinguish a
 * logical endpoint reference from a generic expression.
 * ============================================================================
 */

endpointReferenceExpression
    : endpointReference
    ;


/*
 * ============================================================================
 * ENDPOINT CONFIGURATION
 * ============================================================================
 *
 * Reusable endpoint-body boundary.
 * ============================================================================
 */

endpointConfiguration
    : endpointBody
    ;


/*
 * ============================================================================
 * FINAL OWNERSHIP INVARIANT
 * ============================================================================
 *
 *     ENDPOINT declaration
 *          |
 *          v
 *     logical endpoint AST
 *          |
 *          v
 *     semantic endpoint model
 *          |
 *          +--> address semantics
 *          +--> service semantics
 *          +--> protocol semantics
 *          +--> channel semantics
 *          +--> discovery semantics
 *          +--> routing semantics
 *          +--> socket semantics
 *          +--> distributed semantics
 *          |
 *          v
 *     target-independent execution planning
 *          |
 *          v
 *     physical realization
 *
 * The endpoint grammar never chooses the physical realization.
 *
 * ============================================================================
 */