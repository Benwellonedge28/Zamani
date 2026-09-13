/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/network.g4
 *
 * Status:
 *     Canonical production parser grammar for source-level networking effects.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains:
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no unsafe code;
 *       - no filesystem access;
 *       - no network access;
 *       - no hardware discovery;
 *       - no runtime calls;
 *       - no machine-specific assumptions.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL NETWORK EFFECT SYNTAX.
 *
 * It provides syntax for expressing:
 *
 *     - network effects;
 *     - endpoint intent;
 *     - abstract channels;
 *     - communication intent;
 *     - message operations;
 *     - protocol requirements;
 *     - service interaction intent;
 *     - connection lifecycle intent;
 *     - communication policies;
 *     - network capabilities;
 *     - network requirements;
 *     - network constraints;
 *     - network preferences;
 *     - transport/security composition;
 *     - distributed/network effect composition.
 *
 * This file does NOT implement networking.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - network effect declarations;
 *     - network operation declarations;
 *     - endpoint declarations;
 *     - abstract endpoint references;
 *     - channel declarations;
 *     - communication declarations;
 *     - message declarations;
 *     - protocol declarations;
 *     - service declarations;
 *     - connection declarations;
 *     - network requirements;
 *     - network constraints;
 *     - network preferences;
 *     - network capability requirements;
 *     - network policy syntax;
 *     - network lifecycle intent;
 *     - network operation invocation syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - IP addresses;
 *     - MAC addresses;
 *     - sockets;
 *     - file descriptors;
 *     - operating-system network APIs;
 *     - DNS resolution;
 *     - routing;
 *     - packet transmission;
 *     - physical network topology;
 *     - network hardware discovery;
 *     - NIC selection;
 *     - device selection;
 *     - port allocation;
 *     - fixed network sizes;
 *     - bandwidth enforcement;
 *     - QoS enforcement;
 *     - cryptographic implementation;
 *     - authentication;
 *     - authorization;
 *     - TLS implementation;
 *     - encryption;
 *     - identity verification;
 *     - distributed consensus;
 *     - runtime scheduling;
 *     - hardware placement;
 *     - resource allocation;
 *     - network simulation;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Core parser
 *       |
 *       +--> Types
 *       +--> Expressions
 *       +--> Effects
 *       +--> Security
 *       +--> THIS FILE
 *       |
 *       v
 *     Frontend AST
 *       |
 *       +--> name resolution
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> network analysis
 *       +--> security analysis
 *       +--> resource analysis
 *       |
 *       v
 *     Canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> distributed representation
 *       +--> hardware representation
 *       +--> network/effect metadata
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / resilience / ZQN
 *       |
 *       v
 *     target lowering
 *       |
 *       v
 *     runtime / hardware
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Network syntax describes WHAT communication behavior is required or
 * requested.
 *
 * It must not permanently encode WHERE that behavior occurs.
 *
 * For example:
 *
 *     requires network::communication;
 *
 * does NOT imply:
 *
 *     use NIC X
 *     use interface Y
 *     use address Z
 *     use exactly N nodes
 *     use exactly N connections
 *     use topology T
 *     use transport implementation P
 *
 * Those are downstream realization decisions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level limits on:
 *
 *     - endpoints;
 *     - channels;
 *     - messages;
 *     - services;
 *     - protocols;
 *     - connections;
 *     - network operations;
 *     - nodes;
 *     - peers;
 *     - routes;
 *     - policy rules;
 *     - capability requirements;
 *     - requirement expressions;
 *     - generic parameters;
 *     - nesting;
 *     - distributed participants.
 *
 * Repetition uses ANTLR repetition operators.
 *
 * Physical limits belong to:
 *
 *     - resource models;
 *     - target descriptions;
 *     - capability discovery;
 *     - deployment;
 *     - runtime policies;
 *     - hardware abstraction;
 *     - operating-system facilities.
 *
 * ============================================================================
 * OPEN-WORLD NETWORK MODEL
 * ============================================================================
 *
 * Network identities are names.
 *
 * The grammar therefore does NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     HTTP2
 *     HTTP3
 *     MQTT
 *     gRPC
 *     IPv4
 *     IPv6
 *     Ethernet
 *     InfiniBand
 *     device_1
 *     node_1
 *     port_1
 *
 * as closed semantic categories.
 *
 * They may be represented as source-level names:
 *
 *     protocol tcp;
 *     protocol custom::transport;
 *     endpoint service::worker;
 *
 * Their semantic meaning is determined downstream.
 *
 * This allows future protocols and communication substrates to be introduced
 * without modifying this grammar.
 *
 * ============================================================================
 * ADDRESSING BOUNDARY
 * ============================================================================
 *
 * Network source syntax may refer to an abstract endpoint.
 *
 * An endpoint identifier is NOT inherently a physical address.
 *
 * Therefore:
 *
 *     endpoint worker;
 *
 * does not imply:
 *
 *     IP address;
 *     MAC address;
 *     socket;
 *     machine;
 *     physical node.
 *
 * Explicit address-like values, when supported by the lexical/type system,
 * remain values. Their interpretation belongs downstream.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Network security composes with:
 *
 *     grammar/effects/security.g4
 *
 * Security syntax remains responsible for:
 *
 *     - security requirements;
 *     - permissions;
 *     - trust;
 *     - authorization intent;
 *     - authentication intent;
 *     - classification;
 *     - cryptographic intent.
 *
 * This file does not redefine those concepts.
 *
 * Example:
 *
 *     requires security::confidentiality;
 *
 * Network transport realization remains downstream.
 *
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * Network operations are effects.
 *
 * This grammar therefore composes with:
 *
 *     grammar/effects/effects.g4
 *
 * It does not create a second effect system.
 *
 * Example:
 *
 *     fn fetch() -> Data
 *         with effects {
 *             network::request,
 *             network::receive
 *         }
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING BOUNDARY
 * ============================================================================
 *
 * Network communication may participate in distributed computation.
 *
 * This file expresses communication intent.
 *
 * It does not own:
 *
 *     - node placement;
 *     - cluster membership;
 *     - replication;
 *     - consistency algorithms;
 *     - consensus algorithms;
 *     - distributed scheduling;
 *     - distributed fault recovery.
 *
 * Those belong to distributed and runtime subsystems.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Network effects may surround quantum computation.
 *
 * Examples:
 *
 *     network::quantum_transport
 *     network::remote_quantum_execution
 *     network::measurement_transport
 *
 * This grammar does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     QEC codes
 *     ZQN faults
 *
 * Quantum semantics remain downstream and the canonical quantum semantic
 * boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Network syntax may describe abstract communication interfaces for hardware
 * and hardware/software co-design.
 *
 * It does not define:
 *
 *     - pins;
 *     - physical addresses;
 *     - FPGA routing;
 *     - ASIC wiring;
 *     - fixed bus widths;
 *     - fixed device counts.
 *
 * Those belong to HDL/hardware semantic layers.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     - source spelling;
 *     - declaration kind;
 *     - qualified-name structure;
 *     - expression structure;
 *     - parameter structure;
 *     - source ordering;
 *     - source spans;
 *     - attributes;
 *     - explicit modifiers;
 *     - explicit requirement/constraint/preference distinctions.
 *
 * The parser MUST NOT construct semantic runtime objects.
 *
 * Semantic analysis may construct:
 *
 *     NetworkEffect
 *     NetworkEndpoint
 *     NetworkChannel
 *     NetworkMessage
 *     NetworkProtocol
 *     NetworkService
 *     NetworkConnection
 *     NetworkRequirement
 *     NetworkConstraint
 *     NetworkPreference
 *     NetworkCapabilityRequirement
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - has no embedded actions;
 *     - has no semantic predicates;
 *     - has no I/O;
 *     - has no random behavior;
 *     - has no runtime-dependent decisions;
 *     - has no hardware discovery.
 *
 * Given a deterministic token stream, parsing is deterministic.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Shared syntax is imported rather than duplicated.
 *
 * Core:
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     parameterList
 *     returnType
 *     whereClause
 *
 * Types:
 *     typeExpression
 *
 * Expressions:
 *     expression
 *     argumentList
 *     blockExpression
 *
 * Effects:
 *     effect references and effect composition.
 *
 * Security is intentionally NOT imported directly here.
 *
 * Security composition is represented through ordinary qualified names and
 * semantic integration with grammar/effects/security.g4.
 *
 * This prevents a circular grammar dependency:
 *
 *     Network -> Security -> Network
 *
 * ============================================================================
 */

parser grammar Network;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions, Effects;


/*
 * ============================================================================
 * 1. NETWORK DECLARATION
 * ============================================================================
 *
 * Introduces an abstract network effect/domain.
 *
 * Examples:
 *
 *     network;
 *
 *     network fabric;
 *
 *     network fabric<T>;
 *
 *     network fabric {
 *         endpoint worker;
 *     }
 *
 * The declaration does not instantiate physical infrastructure.
 */

networkDeclaration
    : attributes?
      visibility?
      NETWORK
      identifier
      genericParameters?
      networkDeclarationBody?
      SEMI?
    ;

networkDeclarationBody
    : LBRACE
      networkMember*
      RBRACE
    ;

networkMember
    : networkEndpointDeclaration
    | networkChannelDeclaration
    | networkMessageDeclaration
    | networkProtocolDeclaration
    | networkServiceDeclaration
    | networkConnectionDeclaration
    | networkRequirementDeclaration
    | networkConstraintDeclaration
    | networkPreferenceDeclaration
    | networkCapabilityDeclaration
    ;


/*
 * ============================================================================
 * 2. NETWORK EFFECT
 * ============================================================================
 *
 * Declares a named network effect.
 *
 * Example:
 *
 *     network effect communication;
 */

networkEffectDeclaration
    : attributes?
      visibility?
      NETWORK
      EFFECT
      identifier
      genericParameters?
      effectDeclarationSignature?
      networkEffectBody?
      SEMI?
    ;

networkEffectBody
    : LBRACE
      networkOperationDeclaration*
      RBRACE
    ;

networkOperationDeclaration
    : attributes?
      visibility?
      ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/*
 * ============================================================================
 * 3. ENDPOINT DECLARATIONS
 * ============================================================================
 *
 * Endpoints are logical communication participants.
 *
 * They are intentionally independent from:
 *
 *     IP;
 *     MAC;
 *     socket;
 *     machine;
 *     device;
 *     node.
 */

networkEndpointDeclaration
    : attributes?
      visibility?
      ENDPOINT
      identifier
      genericParameters?
      networkEndpointTypeClause?
      networkEndpointBody?
      SEMI?
    ;

networkEndpointTypeClause
    : COLON
      qualifiedName
    ;

networkEndpointBody
    : LBRACE
      networkEndpointMember*
      RBRACE
    ;

networkEndpointMember
    : networkEndpointProperty
    | networkCapabilityReference
    | networkRequirement
    | networkConstraint
    | networkPreference
    ;

networkEndpointProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 4. ENDPOINT REFERENCES
 * ============================================================================
 */

networkEndpointReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. CHANNEL DECLARATIONS
 * ============================================================================
 *
 * Channels are abstract communication relationships.
 *
 * No physical transport is implied.
 */

networkChannelDeclaration
    : attributes?
      visibility?
      CHANNEL
      identifier
      genericParameters?
      networkChannelTypeClause?
      networkChannelBody?
      SEMI?
    ;

networkChannelTypeClause
    : COLON
      qualifiedName
    ;

networkChannelBody
    : LBRACE
      networkChannelMember*
      RBRACE
    ;

networkChannelMember
    : networkEndpointBinding
    | networkProtocolBinding
    | networkChannelProperty
    | networkRequirement
    | networkConstraint
    | networkPreference
    ;

networkEndpointBinding
    : ENDPOINT
      networkEndpointReference
      SEMI
    ;

networkProtocolBinding
    : PROTOCOL
      qualifiedName
      SEMI
    ;

networkChannelProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 6. MESSAGE DECLARATIONS
 * ============================================================================
 *
 * A message is a logical data item intended for communication.
 *
 * The type system remains responsible for the payload type.
 */

networkMessageDeclaration
    : attributes?
      visibility?
      MESSAGE
      identifier
      genericParameters?
      networkMessageTypeClause?
      networkMessageBody?
      SEMI?
    ;

networkMessageTypeClause
    : COLON
      typeExpression
    ;

networkMessageBody
    : LBRACE
      networkMessageField*
      RBRACE
    ;

networkMessageField
    : identifier
      COLON
      typeExpression
      SEMI
    ;


/*
 * ============================================================================
 * 7. PROTOCOL DECLARATIONS
 * ============================================================================
 *
 * Protocol identities are open-world names.
 *
 * Example:
 *
 *     protocol custom::reliable;
 *
 * The parser does not know whether a protocol is:
 *
 *     transport;
 *     application;
 *     link;
 *     quantum;
 *     future;
 *     proprietary;
 *     simulated.
 */

networkProtocolDeclaration
    : attributes?
      visibility?
      PROTOCOL
      identifier
      genericParameters?
      networkProtocolBody?
      SEMI?
    ;

networkProtocolBody
    : LBRACE
      networkProtocolMember*
      RBRACE
    ;

networkProtocolMember
    : networkProtocolProperty
    | networkProtocolOperation
    | networkRequirement
    | networkConstraint
    | networkPreference
    ;

networkProtocolProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;

networkProtocolOperation
    : FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      SEMI?
    ;


/*
 * ============================================================================
 * 8. SERVICE DECLARATIONS
 * ============================================================================
 *
 * A service represents an abstract communication service.
 *
 * It is not tied to a machine, process, port, or address.
 */

networkServiceDeclaration
    : attributes?
      visibility?
      SERVICE
      identifier
      genericParameters?
      networkServiceBody?
      SEMI?
    ;

networkServiceBody
    : LBRACE
      networkServiceMember*
      RBRACE
    ;

networkServiceMember
    : networkServiceOperation
    | networkServiceProperty
    | networkEndpointBinding
    | networkProtocolBinding
    | networkRequirement
    | networkConstraint
    | networkPreference
    ;

networkServiceOperation
    : ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMI?
    ;

networkServiceProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 9. CONNECTION DECLARATIONS
 * ============================================================================
 *
 * A connection expresses an abstract relationship.
 *
 * It does not allocate a runtime connection.
 */

networkConnectionDeclaration
    : attributes?
      visibility?
      CONNECTION
      identifier
      genericParameters?
      networkConnectionBody?
      SEMI?
    ;

networkConnectionBody
    : LBRACE
      networkConnectionMember*
      RBRACE
    ;

networkConnectionMember
    : networkEndpointBinding
    | networkProtocolBinding
    | networkConnectionProperty
    | networkRequirement
    | networkConstraint
    | networkPreference
    ;

networkConnectionProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 10. NETWORK OPERATIONS
 * ============================================================================
 *
 * These are source-level operation names.
 *
 * The operation vocabulary remains open-world.
 */

networkOperation
    : networkConnectOperation
    | networkDisconnectOperation
    | networkSendOperation
    | networkReceiveOperation
    | networkRequestOperation
    | networkRespondOperation
    | networkPublishOperation
    | networkSubscribeOperation
    | networkAcceptOperation
    | networkListenOperation
    | networkCloseOperation
    | networkOpenOperation
    | networkInvokeOperation
    ;


/*
 * ============================================================================
 * 11. CONNECT
 * ============================================================================
 */

networkConnectOperation
    : CONNECT
      networkEndpointReference
      networkOperationOptions?
    ;

networkDisconnectOperation
    : DISCONNECT
      networkEndpointReference?
      networkOperationOptions?
    ;


/*
 * ============================================================================
 * 12. SEND / RECEIVE
 * ============================================================================
 */

networkSendOperation
    : SEND
      expression
      networkToClause?
      networkOperationOptions?
    ;

networkReceiveOperation
    : RECEIVE
      networkFromClause?
      networkOperationOptions?
    ;

networkToClause
    : TO
      networkEndpointReference
    ;

networkFromClause
    : FROM
      networkEndpointReference
    ;


/*
 * ============================================================================
 * 13. REQUEST / RESPONSE
 * ============================================================================
 */

networkRequestOperation
    : REQUEST
      expression
      networkToClause?
      networkOperationOptions?
    ;

networkRespondOperation
    : RESPOND
      expression?
      networkOperationOptions?
    ;


/*
 * ============================================================================
 * 14. PUBLISH / SUBSCRIBE
 * ============================================================================
 */

networkPublishOperation
    : PUBLISH
      expression
      networkToClause?
      networkOperationOptions?
    ;

networkSubscribeOperation
    : SUBSCRIBE
      expression
      networkFromClause?
      networkOperationOptions?
    ;


/*
 * ============================================================================
 * 15. LISTEN / ACCEPT / OPEN / CLOSE
 * ============================================================================
 */

networkListenOperation
    : LISTEN
      networkEndpointReference?
      networkOperationOptions?
    ;

networkAcceptOperation
    : ACCEPT
      networkEndpointReference?
      networkOperationOptions?
    ;

networkOpenOperation
    : OPEN
      networkEndpointReference?
      networkOperationOptions?
    ;

networkCloseOperation
    : CLOSE
      networkEndpointReference?
      networkOperationOptions?
    ;


/*
 * ============================================================================
 * 16. SERVICE INVOCATION
 * ============================================================================
 *
 * The service name and operation remain semantic names.
 */

networkInvokeOperation
    : INVOKE
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
      networkOperationOptions?
    ;


/*
 * ============================================================================
 * 17. OPERATION OPTIONS
 * ============================================================================
 *
 * Options are intentionally generic.
 *
 * They do not define a closed transport configuration language.
 */

networkOperationOptions
    : WITH
      networkOptionList
    ;

networkOptionList
    : networkOption
      (COMMA networkOption)*
      COMMA?
    ;

networkOption
    : qualifiedName
    | qualifiedName
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 18. NETWORK EFFECT INVOCATION
 * ============================================================================
 *
 * Integrates network operations with the canonical effect model.
 *
 * Example:
 *
 *     perform network::send(data);
 */

networkPerformExpression
    : PERFORM
      networkOperation
    ;

networkPerformStatement
    : networkPerformExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 19. NETWORK REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * It must not silently degrade into a preference.
 */

networkRequirementDeclaration
    : attributes?
      visibility?
      REQUIREMENT
      NETWORK
      identifier?
      networkRequirement
      SEMI?
    ;

networkRequirement
    : REQUIRES
      networkRequirementExpression
    ;

networkRequirementExpression
    : networkReference
    | networkRequirementCall
    | networkRequirementSet
    | expression
    ;

networkRequirementCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;

networkRequirementSet
    : LBRACE
      networkRequirementItem*
      RBRACE
    ;

networkRequirementItem
    : networkRequirementExpression
      COMMA?
    ;


/*
 * ============================================================================
 * 20. NETWORK CONSTRAINTS
 * ============================================================================
 *
 * A constraint describes a condition a valid realization must satisfy.
 */

networkConstraintDeclaration
    : attributes?
      visibility?
      CONSTRAINT
      NETWORK
      identifier?
      networkConstraint
      SEMI?
    ;

networkConstraint
    : CONSTRAIN
      networkConstraintExpression
    ;

networkConstraintExpression
    : networkReference
    | networkConstraintCall
    | networkConstraintSet
    | expression
    ;

networkConstraintCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;

networkConstraintSet
    : LBRACE
      networkConstraintItem*
      RBRACE
    ;

networkConstraintItem
    : networkConstraintExpression
      COMMA?
    ;


/*
 * ============================================================================
 * 21. NETWORK PREFERENCES
 * ============================================================================
 *
 * A preference expresses a desirable but non-mandatory realization.
 */

networkPreferenceDeclaration
    : attributes?
      visibility?
      PREFERENCE
      NETWORK
      identifier?
      networkPreference
      SEMI?
    ;

networkPreference
    : PREFER
      networkPreferenceExpression
    ;

networkPreferenceExpression
    : networkReference
    | networkPreferenceCall
    | networkPreferenceSet
    | expression
    ;

networkPreferenceCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;

networkPreferenceSet
    : LBRACE
      networkPreferenceItem*
      RBRACE
    ;

networkPreferenceItem
    : networkPreferenceExpression
      COMMA?
    ;


/*
 * ============================================================================
 * 22. NETWORK CAPABILITIES
 * ============================================================================
 *
 * Capability syntax describes required/provided abstract capability names.
 *
 * It does not perform capability discovery.
 */

networkCapabilityDeclaration
    : attributes?
      visibility?
      CAPABILITY
      NETWORK
      identifier
      genericParameters?
      networkCapabilityBody?
      SEMI?
    ;

networkCapabilityBody
    : LBRACE
      networkCapabilityMember*
      RBRACE
    ;

networkCapabilityMember
    : networkCapabilityProperty
    | networkRequirement
    | networkConstraint
    | networkPreference
    ;

networkCapabilityProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;

networkCapabilityReference
    : CAPABILITY
      networkReference
    ;


/*
 * ============================================================================
 * 23. NETWORK REFERENCES
 * ============================================================================
 *
 * All network identities remain open-world qualified names.
 */

networkReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 24. NETWORK REQUIREMENT EXPRESSION HELPERS
 * ============================================================================
 *
 * These named wrappers make semantic AST lowering explicit without duplicating
 * the general expression grammar.
 */

networkSecurityRequirement
    : REQUIRES
      qualifiedName
    ;

networkCapabilityRequirement
    : REQUIRES
      CAPABILITY
      qualifiedName
    ;

networkProtocolRequirement
    : REQUIRES
      PROTOCOL
      qualifiedName
    ;


/*
 * ============================================================================
 * 25. NETWORK EFFECT SIGNATURE
 * ============================================================================
 *
 * Reusable signature syntax for tools and aggregate grammars.
 */

networkEffectSignature
    : NETWORK
      EFFECT
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      whereClause?
    ;


/*
 * ============================================================================
 * 26. NETWORK COMMUNICATION DECLARATION
 * ============================================================================
 *
 * Declares abstract communication intent between logical endpoints.
 */

networkCommunicationDeclaration
    : attributes?
      visibility?
      COMMUNICATION
      identifier?
      networkCommunicationBody
      SEMI?
    ;

networkCommunicationBody
    : LBRACE
      networkCommunicationMember*
      RBRACE
    ;

networkCommunicationMember
    : networkFromEndpoint
    | networkToEndpoint
    | networkMessageBinding
    | networkProtocolBinding
    | networkRequirement
    | networkConstraint
    | networkPreference
    ;

networkFromEndpoint
    : FROM
      networkEndpointReference
      SEMI
    ;

networkToEndpoint
    : TO
      networkEndpointReference
      SEMI
    ;

networkMessageBinding
    : MESSAGE
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 27. NETWORK POLICY
 * ============================================================================
 *
 * Policy syntax is deliberately lightweight.
 *
 * Actual authorization/policy evaluation belongs to security semantics.
 */

networkPolicyDeclaration
    : attributes?
      visibility?
      POLICY
      NETWORK
      identifier
      networkPolicyBody?
      SEMI?
    ;

networkPolicyBody
    : LBRACE
      networkPolicyRule*
      RBRACE
    ;

networkPolicyRule
    : networkPolicyCondition?
      networkPolicyDecision
      networkPolicyAction?
      SEMI?
    ;

networkPolicyCondition
    : WHEN
      expression
    ;

networkPolicyDecision
    : ALLOW
    | DENY
    | REQUIRE
    | REJECT
    ;

networkPolicyAction
    : ON
      networkReference
    ;


/*
 * ============================================================================
 * 28. NETWORK LIFECYCLE
 * ============================================================================
 *
 * Lifecycle declarations express intent only.
 */

networkLifecycleDeclaration
    : attributes?
      visibility?
      LIFECYCLE
      identifier
      networkLifecycleBody?
      SEMI?
    ;

networkLifecycleBody
    : LBRACE
      networkLifecycleStep*
      RBRACE
    ;

networkLifecycleStep
    : OPEN
    | CONNECT
    | LISTEN
    | ACCEPT
    | CLOSE
    | DISCONNECT
    | qualifiedName
    ;


/*
 * ============================================================================
 * 29. NETWORK BLOCK
 * ============================================================================
 *
 * Generic composition block for source-level network intent.
 */

networkBlock
    : NETWORK
      LBRACE
      networkBlockItem*
      RBRACE
    ;

networkBlockItem
    : networkOperation SEMI?
    | networkRequirement
    | networkConstraint
    | networkPreference
    | networkCommunicationDeclaration
    | networkPolicyDeclaration
    ;


/*
 * ============================================================================
 * 30. NETWORK STATEMENT
 * ============================================================================
 *
 * Aggregate parsers may use this rule to embed networking operations into
 * ordinary statement contexts.
 */

networkStatement
    : networkPerformStatement
    | networkOperation SEMI?
    | networkBlock
    ;


/*
 * ============================================================================
 * 31. NETWORK EXPRESSION
 * ============================================================================
 *
 * Aggregate expression grammars may use this rule where effect expressions
 * are permitted.
 */

networkExpression
    : networkPerformExpression
    | networkOperation
    | networkInvokeOperation
    ;


/*
 * ============================================================================
 * 32. NETWORK ANNOTATION TARGET
 * ============================================================================
 *
 * Provides a stable grammar composition point for future attributes without
 * making annotations themselves network semantics.
 */

networkAnnotationTarget
    : networkReference
    | networkEndpointReference
    | networkChannelReference
    | networkServiceReference
    | networkProtocolReference
    ;


/*
 * ============================================================================
 * 33. CHANNEL / SERVICE / PROTOCOL REFERENCES
 * ============================================================================
 */

networkChannelReference
    : qualifiedName
    ;

networkServiceReference
    : qualifiedName
    ;

networkProtocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 34. SEMANTICALLY NAMED CONVENIENCE FORMS
 * ============================================================================
 *
 * These remain syntax aliases, not closed vocabularies.
 */

networkSend
    : SEND
      expression
      (TO networkEndpointReference)?
    ;

networkReceive
    : RECEIVE
      (FROM networkEndpointReference)?
    ;

networkRequest
    : REQUEST
      expression
      (TO networkEndpointReference)?
    ;

networkPublish
    : PUBLISH
      expression
      (TO networkEndpointReference)?
    ;

networkSubscribe
    : SUBSCRIBE
      expression
      (FROM networkEndpointReference)?
    ;


/*
 * ============================================================================
 * 35. GENERIC NETWORK REQUIREMENT
 * ============================================================================
 *
 * Generic form permits future network properties without grammar modification.
 *
 * Examples:
 *
 *     requires network::reliability;
 *     requires network::latency(bound);
 *     requires network::availability(level);
 *
 * Semantic validation determines the meaning.
 */

networkPropertyRequirement
    : REQUIRES
      qualifiedName
      (
          LPAREN
          argumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 36. NETWORK PROPERTY CONSTRAINT
 * ============================================================================
 */

networkPropertyConstraint
    : CONSTRAIN
      qualifiedName
      (
          LPAREN
          argumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 37. NETWORK PROPERTY PREFERENCE
 * ============================================================================
 */

networkPropertyPreference
    : PREFER
      qualifiedName
      (
          LPAREN
          argumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 38. NETWORK EFFECT SET
 * ============================================================================
 *
 * Composes directly with the canonical Effects grammar.
 */

networkEffectSet
    : LBRACE
      effectReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 39. NETWORK HANDLER
 * ============================================================================
 *
 * Network effects can be handled by the canonical effect-handler system.
 *
 * This rule deliberately does not redefine handler semantics.
 */

networkHandler
    : HANDLE
      expression
      effectHandlerBody
    ;


/*
 * ============================================================================
 * 40. NETWORK RESOURCE-NEUTRALITY
 * ============================================================================
 *
 * There is intentionally no grammar production for:
 *
 *     device_count
 *     node_count
 *     interface_count
 *     fixed_bandwidth
 *     fixed_latency
 *     fixed_topology
 *     fixed_port
 *     fixed_address
 *
 * Such values, when semantically meaningful, must be represented as:
 *
 *     expression values;
 *     requirements;
 *     constraints;
 *     preferences;
 *     capabilities;
 *     target descriptions;
 *     deployment configuration;
 *     runtime-discovered resources.
 *
 * ============================================================================
 * 41. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Lexer:
 *
 *     ZamaniLexer / ZamaniTokens
 *
 * must provide the keyword tokens used above.
 *
 * Core:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     parameterList
 *     returnType
 *     whereClause
 *
 * Types:
 *
 *     typeExpression
 *
 * Expressions:
 *
 *     expression
 *     argumentList
 *     blockExpression
 *
 * Effects:
 *
 *     effectReferenceList
 *     effectReference
 *     effectHandlerBody
 *
 * Security:
 *
 *     Semantic integration only.
 *
 * This parser MUST NOT import Security directly because the network/security
 * relationship is semantic composition, not a grammar ownership dependency.
 *
 * ============================================================================
 * 42. AST INTEGRATION
 * ============================================================================
 *
 * AST lowering must produce syntax-preserving nodes corresponding to:
 *
 *     NetworkDeclaration
 *     NetworkEffectDeclaration
 *     NetworkOperationDeclaration
 *     NetworkEndpointDeclaration
 *     NetworkChannelDeclaration
 *     NetworkMessageDeclaration
 *     NetworkProtocolDeclaration
 *     NetworkServiceDeclaration
 *     NetworkConnectionDeclaration
 *     NetworkOperation
 *     NetworkRequirement
 *     NetworkConstraint
 *     NetworkPreference
 *     NetworkCapability
 *     NetworkCommunication
 *     NetworkPolicy
 *     NetworkLifecycle
 *
 * The AST must retain:
 *
 *     source span;
 *     source name;
 *     qualified-name segments;
 *     arguments;
 *     type expressions;
 *     attributes;
 *     explicit requirement/constraint/preference classification.
 *
 * ============================================================================
 * 43. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     endpoint identities;
 *     channel identities;
 *     service identities;
 *     protocol identities;
 *     effect identities;
 *     capability identities;
 *     security references;
 *     resource requirements;
 *     network constraints;
 *     preferences.
 *
 * Semantic analysis MUST reject invalid combinations.
 *
 * The parser itself MUST NOT perform those checks.
 *
 * ============================================================================
 * 44. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Network operations involving ordinary classical values lower through the
 * canonical classical semantic/IR path.
 *
 * This grammar must never define a second classical value representation.
 *
 * ============================================================================
 * 45. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Network operations surrounding quantum programs lower through the existing
 * quantum semantic pipeline.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar does not create:
 *
 *     QuantumGate
 *     QubitId
 *     PhysicalQubitId
 *     QuantumCircuit
 *
 * ============================================================================
 * 46. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Network syntax may be consumed by the distributed subsystem for:
 *
 *     communication;
 *     service interaction;
 *     message exchange;
 *     endpoint relationships.
 *
 * Distributed placement, replication, consistency and fault tolerance remain
 * outside this grammar.
 *
 * ============================================================================
 * 47. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware analysis may map network requirements to available capabilities.
 *
 * This grammar does not select:
 *
 *     NIC;
 *     bus;
 *     interconnect;
 *     accelerator;
 *     physical interface;
 *     device.
 *
 * ============================================================================
 * 48. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis may interpret network requirements involving:
 *
 *     bandwidth;
 *     latency;
 *     energy;
 *     reliability;
 *     availability;
 *     throughput;
 *     scalability.
 *
 * The grammar imposes no finite numerical bounds.
 *
 * ============================================================================
 * 49. SECURITY INTEGRATION
 * ============================================================================
 *
 * Security analysis may combine network intent with:
 *
 *     authentication;
 *     authorization;
 *     confidentiality;
 *     integrity;
 *     privacy;
 *     trust;
 *     cryptographic requirements.
 *
 * The security grammar remains the owner of security declarations.
 *
 * ============================================================================
 * 50. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Network operations may become scheduling dependencies downstream.
 *
 * This grammar does not determine:
 *
 *     execution order;
 *     timing;
 *     retry timing;
 *     packet scheduling;
 *     resource scheduling.
 *
 * ============================================================================
 * 51. RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Network failures may become resilience incidents downstream.
 *
 * This grammar does not implement:
 *
 *     retry;
 *     restart;
 *     rollback;
 *     reroute;
 *     backend switching;
 *     recovery.
 *
 * ============================================================================
 * 52. VALIDATION REQUIREMENTS
 * ============================================================================
 *
 * Production validation must verify:
 *
 *     - no network grammar rule defines a physical machine;
 *     - no finite resource maximum exists;
 *     - no provider-specific implementation is required;
 *     - requirements remain distinct from preferences;
 *     - constraints remain distinct from requirements;
 *     - capabilities remain distinct from resources;
 *     - network and security ownership remain separate;
 *     - network and distributed ownership remain separate;
 *     - network and hardware ownership remain separate;
 *     - network and runtime ownership remain separate.
 *
 * ============================================================================
 * 53. DETERMINISM REQUIREMENTS
 * ============================================================================
 *
 * Parsing must be deterministic for deterministic token streams.
 *
 * No semantic predicate or target-language action may be added to this file.
 *
 * ============================================================================
 * 54. COMPATIBILITY REQUIREMENTS
 * ============================================================================
 *
 * Adding a new network keyword requires:
 *
 *     1. lexical specification update;
 *     2. language-version assessment;
 *     3. compatibility assessment;
 *     4. parser tests;
 *     5. documentation update.
 *
 * New protocol/service/device names MUST NOT require grammar modification.
 *
 * ============================================================================
 * 55. NO-HARD-CODING REQUIREMENT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_MESSAGE_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_NETWORKS
 *     MAX_SERVICES
 *     MAX_PROTOCOLS
 *
 * Also forbidden:
 *
 *     device-specific names;
 *     fixed addresses;
 *     fixed topology;
 *     fixed node counts;
 *     fixed interface counts.
 *
 * ============================================================================
 * 56. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] It parses all supported network syntax.
 *     [ ] It imports shared syntax rather than duplicating it.
 *     [ ] It integrates with Effects.
 *     [ ] It integrates semantically with Security.
 *     [ ] It integrates semantically with Distributed.
 *     [ ] It integrates semantically with Hardware.
 *     [ ] It integrates semantically with Resources.
 *     [ ] It integrates semantically with Runtime.
 *     [ ] It introduces no second IR.
 *     [ ] It introduces no physical-resource limits.
 *     [ ] It contains no embedded actions.
 *     [ ] It contains no semantic predicates.
 *     [ ] It contains no unsafe code.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Compatibility tests exist.
 *     [ ] Round-trip tests exist where printer support exists.
 *
 * ============================================================================
 */