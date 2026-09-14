/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/networking.g4
 *
 * Grammar:
 *     Networking
 *
 * Purpose:
 *     Canonical production parser grammar for backend-independent networking
 *     intent in Zamani.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, hardware access, runtime calls, or
 *     unsafe code.
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
 *     Networking parser grammar
 *          |
 *          v
 *     Frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource/constraint analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> distributed execution model
 *          +--> hardware/runtime networking model
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
 *     - networking declaration syntax;
 *     - logical endpoint declarations;
 *     - logical channel declarations;
 *     - service declarations;
 *     - protocol declarations;
 *     - message/communication intent declarations;
 *     - connection/communication intent;
 *     - networking requirements;
 *     - networking constraints;
 *     - networking preferences;
 *     - networking policy blocks;
 *     - networking metadata;
 *     - networking-scoped nested configuration syntax;
 *     - source-level networking expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - data schemas;
 *     - serialization formats;
 *     - cryptographic algorithms;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - physical network discovery;
 *     - physical topology;
 *     - routing algorithms;
 *     - packet scheduling;
 *     - resource allocation;
 *     - hardware discovery;
 *     - device discovery;
 *     - IP allocation;
 *     - socket implementation;
 *     - transport implementation;
 *     - TCP implementation;
 *     - UDP implementation;
 *     - QUIC implementation;
 *     - HTTP implementation;
 *     - MPI implementation;
 *     - RDMA implementation;
 *     - vendor APIs;
 *     - cloud-provider APIs;
 *     - runtime networking;
 *     - distributed fault recovery;
 *     - resilience;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Networking syntax expresses communication intent rather than requiring a
 * particular physical realization.
 *
 * A Zamani program may therefore express:
 *
 *     - communication;
 *     - service interaction;
 *     - endpoint requirements;
 *     - protocol requirements;
 *     - latency preferences;
 *     - bandwidth requirements;
 *     - reliability requirements;
 *     - locality preferences;
 *     - security requirements;
 *     - ordering requirements;
 *     - delivery semantics;
 *     - communication relationships;
 *
 * without embedding a particular machine or network topology.
 *
 * The following are deliberately NOT grammar-level limits:
 *
 *     - number of endpoints;
 *     - number of services;
 *     - number of channels;
 *     - number of nodes;
 *     - number of messages;
 *     - number of communication relationships;
 *     - bandwidth;
 *     - latency;
 *     - network size;
 *     - address-space size;
 *     - topology size;
 *     - deployment size.
 *
 * ============================================================================
 * PHYSICAL NETWORK INDEPENDENCE
 * ============================================================================
 *
 * The grammar does not require:
 *
 *     host
 *     port
 *     IPv4
 *     IPv6
 *     MAC address
 *     socket
 *     interface
 *     router
 *     switch
 *     subnet
 *     physical link
 *     device identifier
 *
 * A source program MAY express an address as semantic data where a particular
 * application requires one, but such an address is data/intent and MUST NOT
 * become an implicit requirement of the networking grammar.
 *
 * Physical realization is selected downstream.
 *
 * ============================================================================
 * TRANSPORT INDEPENDENCE
 * ============================================================================
 *
 * Protocol syntax is intentionally represented as a source-level name or
 * expression.
 *
 * This permits:
 *
 *     protocol: tcp;
 *     protocol: udp;
 *     protocol: quic;
 *     protocol: mpi;
 *     protocol: rdma;
 *     protocol: custom::transport;
 *
 * without this grammar implementing or assuming any of them.
 *
 * Whether a protocol name is:
 *
 *     - available;
 *     - supported;
 *     - preferred;
 *     - required;
 *     - portable;
 *     - secure;
 *     - executable;
 *
 * is determined by semantic/capability/runtime layers.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Networking syntax may express security requirements such as:
 *
 *     security: secure_channel;
 *     requires: encrypted_transport;
 *
 * but MUST NOT define:
 *
 *     - cryptographic algorithms;
 *     - key generation;
 *     - key storage;
 *     - certificate verification;
 *     - identity verification;
 *     - authorization;
 *     - trust policy implementation.
 *
 * Those belong to grammar/security and downstream security systems.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Networking describes communication.
 *
 * Distributed execution describes execution placement and distributed
 * computation.
 *
 * Therefore this grammar MUST NOT define:
 *
 *     - node discovery;
 *     - process placement;
 *     - cluster membership;
 *     - replication;
 *     - consensus;
 *     - distributed scheduling;
 *     - distributed checkpoint implementation.
 *
 * Those belong to grammar/distributed and downstream distributed systems.
 *
 * ============================================================================
 * DATA BOUNDARY
 * ============================================================================
 *
 * Networking can reference a message/data value through expressions.
 *
 * It does not define:
 *
 *     - schemas;
 *     - serialization formats;
 *     - database structures;
 *     - stream transformations;
 *     - binary encodings.
 *
 * Those belong to grammar/data and interoperability/serialization layers.
 *
 * ============================================================================
 * HARDWARE / QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking can connect classical, quantum, HDL, accelerator, AI, or
 * heterogeneous computations.
 *
 * It MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     Gate
 *     Circuit
 *     FPGA resource
 *     GPU resource
 *     CPU topology
 *     hardware topology
 *     pulse
 *     calibration
 *
 * If communication participates in quantum computation, quantum semantics
 * continue through the canonical `quantum::ir` boundary.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All collections use unbounded ANTLR repetition operators.
 *
 * There are deliberately no grammar constants such as:
 *
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_SERVICES
 *     MAX_NODES
 *     MAX_MESSAGES
 *     MAX_CONNECTIONS
 *
 * Practical limits are implementation/resource-policy concerns.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no random behavior;
 *     - no environment inspection;
 *     - no network access;
 *     - no filesystem access;
 *     - no hardware access.
 *
 * Parsing is therefore independent of the current execution environment.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This parser consumes:
 *
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     DOT
 *     DOUBLE_COLON
 *
 * and the expression grammar.
 *
 * This file MUST NOT define lexer rules.
 *
 * Networking terminology is intentionally contextual rather than introducing
 * a second lexer authority.
 *
 * ============================================================================
 * INTEGRATION
 * ============================================================================
 *
 * Required parser imports:
 *
 *     Names
 *     Expressions
 *
 * Canonical public entry point:
 *
 *     networkingDeclaration
 *
 * The networking aggregate/parser integration should consume that rule.
 *
 * ============================================================================
 */

parser grammar Networking;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * A networking declaration introduces a logical networking specification.
 *
 * Canonical source shape:
 *
 *     networking my_network {
 *         ...
 *     }
 *
 * The spelling `networking` is validated by semantic analysis because the
 * canonical lexer intentionally does not reserve every future domain word.
 */
networkingDeclaration
    : networkingMarker
      identifier
      networkingBody?
      SEMICOLON?
    ;


/* ============================================================================
 * CONTEXTUAL MARKER
 * ========================================================================== */

networkingMarker
    : identifier
    ;


/* ============================================================================
 * NETWORKING BODY
 * ========================================================================== */

networkingBody
    : LBRACE
      networkingMember*
      RBRACE
    ;


/* ============================================================================
 * TOP-LEVEL NETWORKING MEMBERS
 * ========================================================================== */

networkingMember
    : endpointDeclaration
    | channelDeclaration
    | serviceDeclaration
    | protocolDeclaration
    | messageDeclaration
    | connectionDeclaration
    | communicationDeclaration
    | requirementDeclaration
    | constraintDeclaration
    | preferenceDeclaration
    | policyDeclaration
    | metadataDeclaration
    | networkingBlock
    | networkingExpressionMember
    ;


/* ============================================================================
 * LOGICAL ENDPOINT
 * ========================================================================== */

/*
 * Endpoint means a logical communication participant.
 *
 * It does not inherently mean:
 *
 *     IP address
 *     hostname
 *     socket
 *     physical interface
 *     machine
 *     device
 *
 * Example:
 *
 *     endpoint producer {
 *         role: source;
 *         capability: publish;
 *     }
 */
endpointDeclaration
    : endpointMarker
      identifier
      networkingObjectBody?
      SEMICOLON?
    ;

endpointMarker
    : identifier
    ;


/* ============================================================================
 * LOGICAL CHANNEL
 * ========================================================================== */

/*
 * A channel expresses a communication relationship.
 *
 * It does not imply a particular transport or physical link.
 */
channelDeclaration
    : channelMarker
      identifier
      networkingObjectBody?
      SEMICOLON?
    ;

channelMarker
    : identifier
    ;


/* ============================================================================
 * SERVICE
 * ========================================================================== */

/*
 * A service is a logical communication interface.
 *
 * Service discovery, process placement and deployment are downstream concerns.
 */
serviceDeclaration
    : serviceMarker
      identifier
      networkingObjectBody?
      SEMICOLON?
    ;

serviceMarker
    : identifier
    ;


/* ============================================================================
 * PROTOCOL
 * ========================================================================== */

/*
 * A protocol declaration identifies a logical protocol requirement or
 * capability.
 *
 * Example:
 *
 *     protocol control {
 *         requires: reliable;
 *     }
 *
 * The protocol name itself remains an ordinary source-level name.
 */
protocolDeclaration
    : protocolMarker
      identifier
      networkingObjectBody?
      SEMICOLON?
    ;

protocolMarker
    : identifier
    ;


/* ============================================================================
 * MESSAGE
 * ========================================================================== */

/*
 * Message declarations identify logical messages.
 *
 * Message schema and serialization remain owned by data/interoperability
 * layers.
 */
messageDeclaration
    : messageMarker
      identifier
      messageBody?
      SEMICOLON?
    ;

messageMarker
    : identifier
    ;

messageBody
    : LBRACE
      messageMember*
      RBRACE
    ;

messageMember
    : identifier
      COLON
      expression
      SEMICOLON
    | networkingBlock
    ;


/* ============================================================================
 * CONNECTION INTENT
 * ========================================================================== */

/*
 * A connection is an intent to establish or represent a logical relationship.
 *
 * It does not execute a connection.
 */
connectionDeclaration
    : connectionMarker
      identifier
      connectionBody?
      SEMICOLON?
    ;

connectionMarker
    : identifier
    ;

connectionBody
    : LBRACE
      connectionMember*
      RBRACE
    ;

connectionMember
    : fromMember
    | toMember
    | protocolMember
    | channelMember
    | connectionOptionMember
    | networkingBlock
    ;

fromMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

toMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

protocolMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

channelMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

connectionOptionMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * COMMUNICATION INTENT
 * ========================================================================== */

/*
 * Communication expresses intent to send, receive, publish, subscribe, or
 * otherwise communicate.
 *
 * Runtime behavior belongs downstream.
 */
communicationDeclaration
    : communicationMarker
      identifier
      communicationBody?
      SEMICOLON?
    ;

communicationMarker
    : identifier
    ;

communicationBody
    : LBRACE
      communicationMember*
      RBRACE
    ;

communicationMember
    : sourceMember
    | destinationMember
    | payloadMember
    | messageMember
    | deliveryMember
    | orderingMember
    | communicationOptionMember
    | networkingBlock
    ;

sourceMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

destinationMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

payloadMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

messageMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

deliveryMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

orderingMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;

communicationOptionMember
    : identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * REQUIREMENTS
 * ========================================================================== */

/*
 * Requirements are mandatory semantic conditions.
 *
 * They are not physical resource allocations.
 *
 * Examples:
 *
 *     requires: reliable;
 *     requires: encrypted;
 *     requires: low_latency;
 *     requires: quantum_networking;
 */
requirementDeclaration
    : requirementMarker
      COLON
      expression
      SEMICOLON
    ;

requirementMarker
    : identifier
    ;


/* ============================================================================
 * CONSTRAINTS
 * ========================================================================== */

/*
 * Constraints restrict legal realizations without selecting a particular
 * realization.
 */
constraintDeclaration
    : constraintMarker
      COLON
      expression
      SEMICOLON
    ;

constraintMarker
    : identifier
    ;


/* ============================================================================
 * PREFERENCES
 * ========================================================================== */

/*
 * Preferences are optimization hints rather than semantic requirements.
 *
 * A preference may be ignored when satisfying it would violate stronger
 * requirements or constraints.
 */
preferenceDeclaration
    : preferenceMarker
      COLON
      expression
      SEMICOLON
    ;

preferenceMarker
    : identifier
    ;


/* ============================================================================
 * POLICY
 * ========================================================================== */

/*
 * Policy data is source-level intent.
 *
 * It does not execute retries, failover, recovery, authentication, routing,
 * scheduling, or resilience behavior.
 */
policyDeclaration
    : policyMarker
      identifier
      networkingObjectBody?
      SEMICOLON?
    ;

policyMarker
    : identifier
    ;


/* ============================================================================
 * METADATA
 * ========================================================================== */

/*
 * Metadata is descriptive information and has no inherent execution effect.
 */
metadataDeclaration
    : metadataMarker
      COLON
      expression
      SEMICOLON
    ;

metadataMarker
    : identifier
    ;


/* ============================================================================
 * GENERIC NETWORKING BLOCK
 * ========================================================================== */

/*
 * Extension point for networking-specific constructs.
 *
 * Example:
 *
 *     qos {
 *         latency: requirement;
 *         throughput: requirement;
 *     }
 *
 * The grammar remains extensible without adding a global keyword for every
 * future networking concept.
 */
networkingBlock
    : identifier
      networkingObjectBody
    ;

networkingObjectBody
    : LBRACE
      networkingProperty*
      RBRACE
    ;


/* ============================================================================
 * NETWORKING PROPERTY
 * ========================================================================== */

networkingProperty
    : networkingAssignment
    | networkingNestedBlock
    ;

networkingAssignment
    : identifier
      COLON
      expression
      SEMICOLON
    ;

networkingNestedBlock
    : identifier
      networkingObjectBody
    ;


/* ============================================================================
 * NETWORKING EXPRESSION MEMBER
 * ========================================================================== */

/*
 * Allows a networking-scoped expression to be represented without inventing
 * another statement syntax.
 *
 * Semantic analysis decides whether the expression has networking meaning.
 */
networkingExpressionMember
    : expression
      SEMICOLON
    ;


/* ============================================================================
 * REUSABLE NETWORKING LISTS
 * ========================================================================== */

/*
 * These rules intentionally contain no finite bounds.
 */

networkingIdentifierList
    : identifier
      (COMMA identifier)*
    ;

networkingQualifiedNameList
    : qualifiedName
      (COMMA qualifiedName)*
    ;

networkingExpressionList
    : expression
      (COMMA expression)*
    ;


/* ============================================================================
 * CONNECTION TARGET
 * ========================================================================== */

/*
 * A communication target may be:
 *
 *     - a logical endpoint;
 *     - a service;
 *     - a channel;
 *     - a name;
 *     - an expression producing a target;
 *     - a future runtime-resolved target.
 *
 * This grammar does not force one physical addressing mechanism.
 */
networkingTarget
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * DELIVERY INTENT
 * ========================================================================== */

/*
 * Delivery semantics are represented as source-level intent.
 *
 * Examples:
 *
 *     reliable
 *     best_effort
 *     ordered
 *     unordered
 *     exactly_once
 *     at_least_once
 *     at_most_once
 *
 * These are not interpreted by the parser.
 */
deliveryIntent
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * ORDERING INTENT
 * ========================================================================== */

orderingIntent
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * PROTOCOL REFERENCE
 * ========================================================================== */

protocolReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * SERVICE REFERENCE
 * ========================================================================== */

serviceReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * ENDPOINT REFERENCE
 * ========================================================================== */

endpointReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * CHANNEL REFERENCE
 * ========================================================================== */

channelReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * NETWORKING CAPABILITY REFERENCE
 * ========================================================================== */

/*
 * Capability names are deliberately not enumerated here.
 *
 * This allows future networking capabilities without grammar churn.
 */
networkingCapabilityReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * NETWORKING REQUIREMENT REFERENCE
 * ========================================================================== */

networkingRequirementReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * NETWORKING CONSTRAINT REFERENCE
 * ========================================================================== */

networkingConstraintReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * NETWORKING PREFERENCE REFERENCE
 * ========================================================================== */

networkingPreferenceReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * NETWORKING POLICY REFERENCE
 * ========================================================================== */

networkingPolicyReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * END OF GRAMMAR
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 *
 * 1. The canonical lexer remains:
 *
 *        grammar/antlr/ZamaniLexer.g4
 *
 * 2. Names come from:
 *
 *        grammar/core/names.g4
 *
 * 3. Expressions come from:
 *
 *        grammar/expressions/expressions.g4
 *
 * 4. No networking lexer is introduced here.
 *
 * 5. No networking AST is introduced here.
 *
 * 6. No network runtime behavior is introduced here.
 *
 * 7. No physical topology is introduced here.
 *
 * 8. No hard-coded resource limits are introduced here.
 *
 * 9. Networking semantics are lowered by frontend semantic analysis.
 *
 * 10. Distributed execution consumes the resulting semantic model rather
 *     than importing networking implementation details.
 *
 * 11. Hardware/resource systems determine physical realization.
 *
 * 12. Security systems determine security realization.
 *
 * 13. Data/interoperability systems determine representation and serialization.
 *
 * 14. Scheduling determines temporal ordering.
 *
 * 15. Routing/deployment determines physical/network placement.
 *
 * 16. Runtime determines actual communication.
 *
 * 17. Quantum communication continues through the canonical quantum::ir
 *     semantic boundary when quantum computation is involved.
 *
 * 18. QEC, ZQN and resilience remain outside this grammar.
 *
 * ============================================================================
 */