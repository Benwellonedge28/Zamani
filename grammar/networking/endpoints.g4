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
 * Purpose:
 *     Canonical production parser grammar for logical networking endpoint
 *     declarations in Zamani.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, hardware access, runtime calls,
 *     random behavior, or unsafe code.
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
 *     Zamani parser
 *          |
 *          v
 *     Endpoints
 *          |
 *          v
 *     Frontend AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource/constraint analysis
 *          |
 *          v
 *     Canonical semantic representation
 *          |
 *          +--> networking model
 *          +--> distributed model
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> hardware/runtime model
 *          |
 *          v
 *     routing / scheduling / deployment / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical endpoint declaration syntax;
 *     - endpoint names;
 *     - endpoint roles;
 *     - endpoint references;
 *     - endpoint relationships;
 *     - endpoint capabilities as source-level declarations;
 *     - endpoint requirements;
 *     - endpoint constraints;
 *     - endpoint preferences;
 *     - endpoint policies;
 *     - endpoint metadata;
 *     - logical endpoint addressing expressions;
 *     - endpoint-local configuration;
 *     - endpoint-local annotations/configuration represented by expressions;
 *     - reusable endpoint-reference syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - keywords;
 *     - Unicode identifier rules;
 *     - qualified-name lexical structure;
 *     - general expressions;
 *     - data schemas;
 *     - message schemas;
 *     - serialization;
 *     - cryptographic algorithms;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - physical network discovery;
 *     - network-interface discovery;
 *     - IP allocation;
 *     - DNS;
 *     - sockets;
 *     - ports as operating-system resources;
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - HTTP;
 *     - MPI;
 *     - RDMA;
 *     - vendor networking APIs;
 *     - cloud provider APIs;
 *     - routing algorithms;
 *     - packet scheduling;
 *     - distributed placement;
 *     - cluster membership;
 *     - replication;
 *     - consensus;
 *     - distributed fault recovery;
 *     - resilience;
 *     - hardware discovery;
 *     - accelerator discovery;
 *     - CPU topology;
 *     - GPU topology;
 *     - FPGA topology;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * An endpoint is a LOGICAL communication participant.
 *
 * It is not inherently:
 *
 *     - a machine;
 *     - a CPU;
 *     - a GPU;
 *     - a QPU;
 *     - an FPGA;
 *     - a process;
 *     - a thread;
 *     - a socket;
 *     - an IP address;
 *     - a physical NIC;
 *     - a router;
 *     - a switch;
 *     - a cloud instance.
 *
 * A Zamani source program can therefore describe:
 *
 *     endpoint producer { ... }
 *     endpoint consumer { ... }
 *
 * without deciding where those endpoints will execute.
 *
 * The compiler/runtime may later realize them as:
 *
 *     - local objects;
 *     - processes;
 *     - services;
 *     - machines;
 *     - containers;
 *     - embedded nodes;
 *     - accelerators;
 *     - quantum control systems;
 *     - distributed services;
 *     - future execution resources.
 *
 * That realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level constants for:
 *
 *     MAX_ENDPOINTS
 *     MAX_ADDRESSES
 *     MAX_CHANNELS
 *     MAX_SERVICES
 *     MAX_PROTOCOLS
 *     MAX_ENDPOINT_PROPERTIES
 *     MAX_ENDPOINT_CAPABILITIES
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *
 * Repetition uses ANTLR's unbounded repetition operators.
 *
 * Practical limits may still arise from:
 *
 *     - source size;
 *     - available memory;
 *     - parser implementation;
 *     - compiler policy;
 *     - operating-system resources;
 *     - runtime resources;
 *     - deployment resources.
 *
 * Those limits MUST NOT be encoded into this grammar.
 *
 * ============================================================================
 * PHYSICAL RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     IPv4
 *     IPv6
 *     MAC
 *     hostname
 *     TCP port
 *     socket
 *     interface
 *     router
 *     switch
 *     subnet
 *     physical link
 *     device identifier
 *
 * An endpoint may contain a source-level address/value expression if an
 * application explicitly needs one.
 *
 * Such a value remains application/semantic data.
 *
 * It does not make the corresponding physical mechanism a requirement of
 * the language.
 *
 * ============================================================================
 * NETWORK / DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Networking owns the expression of communication participants.
 *
 * Distributed execution owns:
 *
 *     - placement;
 *     - node discovery;
 *     - cluster membership;
 *     - replication;
 *     - distributed scheduling;
 *     - service deployment;
 *     - consensus;
 *     - distributed checkpointing.
 *
 * Hardware owns:
 *
 *     - physical devices;
 *     - topology;
 *     - device capabilities;
 *     - calibration;
 *     - physical resource allocation.
 *
 * Scheduling owns:
 *
 *     - temporal ordering;
 *     - resource scheduling;
 *     - timing;
 *     - synchronization.
 *
 * Routing owns:
 *
 *     - logical-to-physical network realization;
 *     - route selection.
 *
 * Resilience owns:
 *
 *     - recovery decisions;
 *     - retry;
 *     - rerouting;
 *     - failover;
 *     - backend adaptation.
 *
 * This file does not duplicate any of those responsibilities.
 *
 * ============================================================================
 * DATA BOUNDARY
 * ============================================================================
 *
 * Endpoint payload/data types are represented through existing expressions,
 * names, and downstream data/type systems.
 *
 * This grammar does not define:
 *
 *     - serialization;
 *     - schemas;
 *     - binary encodings;
 *     - compression;
 *     - database formats;
 *     - wire formats.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Endpoint syntax may express source-level security requirements, such as:
 *
 *     security: encrypted;
 *     requires: authenticated;
 *     trust: trusted;
 *
 * but does not implement:
 *
 *     - cryptography;
 *     - key generation;
 *     - key storage;
 *     - certificate validation;
 *     - identity verification;
 *     - authorization;
 *     - trust evaluation.
 *
 * Those belong to the security grammar and semantic/security subsystems.
 *
 * ============================================================================
 * QUANTUM / HARDWARE BOUNDARY
 * ============================================================================
 *
 * An endpoint may participate in:
 *
 *     - classical computation;
 *     - quantum computation;
 *     - hybrid computation;
 *     - HDL-controlled systems;
 *     - accelerator computation;
 *     - AI/data workloads;
 *     - distributed computation.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     Gate
 *     Circuit
 *     GPU topology
 *     FPGA resources
 *     CPU topology
 *     hardware topology
 *     pulse schedules
 *     calibration
 *
 * If an endpoint participates in quantum computation, the quantum semantics
 * continue through the canonical `quantum::ir` boundary.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical name grammar:
 *
 *     grammar/core/names.g4
 *
 * Canonical expression grammar:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file MUST NOT define lexer rules.
 *
 * It consumes the canonical parser-level:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *
 * rules.
 *
 * ============================================================================
 * CONTEXTUAL DECLARATION WORD
 * ============================================================================
 *
 * The current Zamani lexer intentionally does not require every domain
 * construct to become a reserved keyword.
 *
 * Therefore `endpoint` is represented structurally as an identifier marker:
 *
 *     endpointDeclaration
 *         : endpointMarker identifier ...
 *
 * Semantic analysis MUST validate that the first identifier has the required
 * endpoint declaration meaning.
 *
 * This avoids:
 *
 *     - changing the canonical lexer merely for this file;
 *     - introducing duplicate lexical authorities;
 *     - making physical/network implementation vocabulary globally reserved;
 *     - preventing future dialects from using related vocabulary.
 *
 * If Zamani later makes `endpoint` a globally reserved keyword, that migration
 * can be performed by the lexer/specification compatibility process without
 * changing the endpoint semantic model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST SHOULD preserve:
 *
 *     - endpoint declaration source span;
 *     - endpoint name;
 *     - endpoint marker source span;
 *     - endpoint members in source order;
 *     - property names;
 *     - property expressions;
 *     - nested blocks;
 *     - references;
 *     - annotations/metadata represented by the enclosing grammar.
 *
 * Parsing MUST NOT resolve:
 *
 *     endpoint -> machine
 *     endpoint -> process
 *     endpoint -> IP address
 *     endpoint -> hardware
 *     endpoint -> network device
 *
 * Name resolution and resource realization occur later.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for validating:
 *
 *     - endpoint marker spelling;
 *     - duplicate endpoint declarations where prohibited;
 *     - endpoint name visibility;
 *     - reference validity;
 *     - property compatibility;
 *     - capability compatibility;
 *     - requirement satisfiability;
 *     - constraint satisfiability;
 *     - preference validity;
 *     - security policy validity;
 *     - cross-domain compatibility.
 *
 * The grammar accepts syntax.
 *
 * Semantic analysis determines meaning.
 *
 * ============================================================================
 * NO HARD-CODED MACHINE SIZE
 * ============================================================================
 *
 * Nothing in this grammar depends on:
 *
 *     - number of machines;
 *     - number of network interfaces;
 *     - number of nodes;
 *     - number of endpoints;
 *     - number of services;
 *     - number of links;
 *     - number of channels;
 *     - bandwidth;
 *     - latency;
 *     - address width;
 *     - topology size.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file depends on:
 *
 *     ZamaniLexer
 *     Names
 *     Expressions
 *
 * It must not depend on:
 *
 *     Hardware
 *     Distributed
 *     Quantum
 *     QEC
 *     ZQN
 *     Scheduling
 *     Routing
 *     Runtime
 *     Resilience
 *
 * Higher-level networking grammar imports this grammar.
 *
 * Required future integration in:
 *
 *     grammar/networking/networking.g4
 *
 *     import Names, Expressions, Endpoints;
 *
 * and its local endpointDeclaration/endpointMarker definitions MUST be
 * removed so this file becomes the sole owner of endpoint syntax.
 *
 * The existing networking aggregate then exposes:
 *
 *     endpointDeclaration
 *
 * through this imported grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no environment inspection;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no randomness;
 *     - no target-dependent parsing.
 *
 * The same token stream therefore has the same syntactic interpretation.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * The grammar deliberately uses explicit member separators and braces.
 *
 * Required parser diagnostics should identify:
 *
 *     - missing endpoint name;
 *     - malformed endpoint body;
 *     - malformed property;
 *     - missing colon;
 *     - missing semicolon;
 *     - malformed expression;
 *     - malformed nested endpoint block.
 *
 * Diagnostic formatting is owned by the frontend diagnostic subsystem rather
 * than this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive examples MUST include:
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
 *     }
 *
 *     endpoint control {
 *         capability: quantum;
 *         requires: authenticated;
 *     }
 *
 *     endpoint::logical;
 *
 *     endpoint node {
 *         address: logical_address;
 *         locality: region;
 *         bandwidth: requested_bandwidth;
 *     }
 *
 * Cross-domain examples MUST include endpoints used by:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     distributed computation;
 *     HDL systems;
 *     accelerators;
 *     AI/data workloads.
 *
 * Negative tests MUST include:
 *
 *     endpoint;
 *     endpoint { ... };
 *     endpoint producer { role };
 *     endpoint producer { : value; };
 *     endpoint producer { role value; };
 *     endpoint producer { role: };
 *
 * Scalability tests MUST verify that the grammar imposes no finite limit on:
 *
 *     endpoint declarations;
 *     endpoint properties;
 *     nested logical configuration where allowed;
 *     qualified endpoint names;
 *     endpoint references.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It is the sole grammar owner of endpoint declaration syntax.
 *
 * [ ] It imports only canonical Names and Expressions dependencies.
 *
 * [ ] It contains no lexer rules.
 *
 * [ ] It contains no embedded Rust.
 *
 * [ ] It contains no unsafe code.
 *
 * [ ] It contains no semantic predicates.
 *
 * [ ] It contains no filesystem/network/hardware access.
 *
 * [ ] It contains no fixed endpoint/resource limits.
 *
 * [ ] It does not define physical networking.
 *
 * [ ] It does not define routing.
 *
 * [ ] It does not define distributed placement.
 *
 * [ ] It does not define security implementation.
 *
 * [ ] It does not define serialization.
 *
 * [ ] It does not duplicate `quantum::ir`.
 *
 * [ ] It does not duplicate hardware identifiers.
 *
 * [ ] `networking.g4` imports this grammar.
 *
 * [ ] The old endpoint rules in `networking.g4` are removed.
 *
 * [ ] Frontend AST lowering maps endpoint declarations to the canonical
 *     networking semantic model.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary/scalability tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * ============================================================================
 */

parser grammar Endpoints;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Canonical logical endpoint declaration.
 *
 * Examples:
 *
 *     endpoint producer;
 *
 *     endpoint producer {
 *         role: source;
 *     }
 *
 *     endpoint consumer {
 *         role: sink;
 *         requires: reliable;
 *     }
 *
 * The declaration marker is contextual rather than a dedicated lexer token.
 */
endpointDeclaration
    : endpointMarker
      identifier
      endpointBody?
      SEMICOLON?
    ;


/* ============================================================================
 * ENDPOINT MARKER
 * ========================================================================== */

/*
 * The semantic layer validates that this identifier represents the endpoint
 * declaration marker.
 *
 * This deliberately avoids adding a second lexical authority.
 */
endpointMarker
    : identifier
    ;


/* ============================================================================
 * ENDPOINT BODY
 * ========================================================================== */

endpointBody
    : LBRACE
      endpointMember*
      RBRACE
    ;


/* ============================================================================
 * ENDPOINT MEMBERS
 * ========================================================================== */

endpointMember
    : endpointRoleDeclaration
    | endpointAddressDeclaration
    | endpointProtocolDeclaration
    | endpointCapabilityDeclaration
    | endpointRequirementDeclaration
    | endpointConstraintDeclaration
    | endpointPreferenceDeclaration
    | endpointSecurityDeclaration
    | endpointLocalityDeclaration
    | endpointPropertyDeclaration
    | endpointReferenceDeclaration
    | endpointRelationshipDeclaration
    | endpointPolicyDeclaration
    | endpointMetadataDeclaration
    | endpointBlock
    ;


/* ============================================================================
 * ROLE
 * ========================================================================== */

/*
 * Examples:
 *
 *     role: source;
 *     role: sink;
 *     role: peer;
 *     role: control;
 *
 * Role vocabulary is semantic rather than hardware-specific.
 */
endpointRoleDeclaration
    : endpointRoleMarker
      COLON
      expression
      SEMICOLON
    ;

endpointRoleMarker
    : identifier
    ;


/* ============================================================================
 * LOGICAL ADDRESS
 * ========================================================================== */

/*
 * An endpoint address is an expression.
 *
 * It may represent:
 *
 *     logical address;
 *     symbolic address;
 *     service identity;
 *     application-defined locator;
 *     runtime-resolved address;
 *     physical address where explicitly required.
 *
 * This grammar does not impose an address format.
 */
endpointAddressDeclaration
    : endpointAddressMarker
      COLON
      expression
      SEMICOLON
    ;

endpointAddressMarker
    : identifier
    ;


/* ============================================================================
 * PROTOCOL
 * ========================================================================== */

/*
 * Protocol is represented as an expression/name.
 *
 * The grammar does not implement TCP, UDP, QUIC, MPI, RDMA, HTTP, or any
 * future transport.
 */
endpointProtocolDeclaration
    : endpointProtocolMarker
      COLON
      expression
      SEMICOLON
    ;

endpointProtocolMarker
    : identifier
    ;


/* ============================================================================
 * CAPABILITY
 * ========================================================================== */

/*
 * Examples:
 *
 *     capability: publish;
 *     capability: subscribe;
 *     capability: reliable;
 *     capability: quantum;
 *     capability: streaming;
 *
 * Capability availability is resolved later.
 */
endpointCapabilityDeclaration
    : endpointCapabilityMarker
      COLON
      expression
      SEMICOLON
    ;

endpointCapabilityMarker
    : identifier
    ;


/* ============================================================================
 * REQUIREMENT
 * ========================================================================== */

/*
 * Requirements are mandatory semantic conditions.
 *
 * They do not allocate a physical resource.
 */
endpointRequirementDeclaration
    : endpointRequirementMarker
      COLON
      expression
      SEMICOLON
    ;

endpointRequirementMarker
    : identifier
    ;


/* ============================================================================
 * CONSTRAINT
 * ========================================================================== */

/*
 * Constraints restrict valid realizations without selecting one.
 */
endpointConstraintDeclaration
    : endpointConstraintMarker
      COLON
      expression
      SEMICOLON
    ;

endpointConstraintMarker
    : identifier
    ;


/* ============================================================================
 * PREFERENCE
 * ========================================================================== */

/*
 * Preferences are optimization hints.
 *
 * They may be ignored if stronger requirements or constraints make them
 * impossible or undesirable.
 */
endpointPreferenceDeclaration
    : endpointPreferenceMarker
      COLON
      expression
      SEMICOLON
    ;

endpointPreferenceMarker
    : identifier
    ;


/* ============================================================================
 * SECURITY
 * ========================================================================== */

/*
 * Security expresses intent only.
 *
 * Examples:
 *
 *     security: encrypted;
 *     security: authenticated;
 *     security: confidential;
 *     security: trusted;
 *
 * Cryptographic implementation belongs elsewhere.
 */
endpointSecurityDeclaration
    : endpointSecurityMarker
      COLON
      expression
      SEMICOLON
    ;

endpointSecurityMarker
    : identifier
    ;


/* ============================================================================
 * LOCALITY
 * ========================================================================== */

/*
 * Locality is a logical preference/constraint.
 *
 * Examples:
 *
 *     locality: local;
 *     locality: regional;
 *     locality: remote;
 *     locality: symbolic_region;
 *
 * It does not define a physical topology.
 */
endpointLocalityDeclaration
    : endpointLocalityMarker
      COLON
      expression
      SEMICOLON
    ;

endpointLocalityMarker
    : identifier
    ;


/* ============================================================================
 * GENERIC PROPERTY
 * ========================================================================== */

/*
 * Future networking properties can be represented without modifying this
 * grammar every time a new semantic property is introduced.
 *
 * Example:
 *
 *     bandwidth: required_bandwidth;
 *     latency: maximum_latency;
 *     reliability: required_reliability;
 *     energy: energy_preference;
 *
 * Semantic analysis determines whether a property is recognized and valid.
 */
endpointPropertyDeclaration
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * ENDPOINT REFERENCE
 * ========================================================================== */

/*
 * References identify another logical endpoint.
 *
 * Examples:
 *
 *     peer: producer;
 *     parent: control;
 *     mirror: backup;
 *
 * A reference does not perform resolution during parsing.
 */
endpointReferenceDeclaration
    : endpointReferenceMarker
      COLON
      qualifiedName
      SEMICOLON
    ;

endpointReferenceMarker
    : identifier
    ;


/* ============================================================================
 * ENDPOINT RELATIONSHIP
 * ========================================================================== */

/*
 * A relationship expresses a logical relationship between endpoints.
 *
 * Examples:
 *
 *     connects: producer;
 *     peers: consumer;
 *     depends_on: control;
 *
 * The semantic layer determines relationship validity.
 */
endpointRelationshipDeclaration
    : endpointRelationshipMarker
      COLON
      endpointReferenceList
      SEMICOLON
    ;

endpointRelationshipMarker
    : identifier
    ;


/* ============================================================================
 * ENDPOINT REFERENCE LIST
 * ========================================================================== */

endpointReferenceList
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
    ;


/* ============================================================================
 * POLICY
 * ========================================================================== */

/*
 * Policy contains endpoint-scoped source-level policy declarations.
 *
 * Policy implementation belongs to semantic/security/runtime layers.
 */
endpointPolicyDeclaration
    : endpointPolicyMarker
      endpointPolicyBody
    ;

endpointPolicyMarker
    : identifier
    ;

endpointPolicyBody
    : LBRACE
      endpointPolicyMember*
      RBRACE
    ;

endpointPolicyMember
    : identifier
      COLON
      expression
      SEMICOLON
    | endpointPolicyBlock
    ;

endpointPolicyBlock
    : identifier
      LBRACE
      endpointPolicyMember*
      RBRACE
    ;


/* ============================================================================
 * METADATA
 * ========================================================================== */

/*
 * Metadata is descriptive information.
 *
 * It must not silently change program semantics.
 */
endpointMetadataDeclaration
    : endpointMetadataMarker
      endpointMetadataBody
    ;

endpointMetadataMarker
    : identifier
    ;

endpointMetadataBody
    : LBRACE
      endpointMetadataMember*
      RBRACE
    ;

endpointMetadataMember
    : identifier
      COLON
      expression
      SEMICOLON
    | endpointMetadataBlock
    ;

endpointMetadataBlock
    : identifier
      LBRACE
      endpointMetadataMember*
      RBRACE
    ;


/* ============================================================================
 * GENERIC ENDPOINT BLOCK
 * ========================================================================== */

/*
 * Extensibility mechanism for networking dialects.
 *
 * This is deliberately structural. It does not authorize arbitrary runtime
 * behavior.
 */
endpointBlock
    : identifier
      LBRACE
      endpointBlockMember*
      RBRACE
    ;

endpointBlockMember
    : identifier
      COLON
      expression
      SEMICOLON
    | endpointBlock
    ;