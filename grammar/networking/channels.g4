/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Networking Channel Grammar
 * ============================================================================
 *
 * File:
 *     grammar/networking/channels.g4
 *
 * Grammar:
 *     NetworkingChannels
 *
 * Purpose:
 *     Canonical parser grammar for backend-independent logical networking
 *     channels.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No environment-dependent parsing.
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
 *     composed parser
 *          |
 *          v
 *     NetworkingChannels
 *          |
 *          v
 *     frontend syntax AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> capability checking
 *          +--> effect checking
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> distributed analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir where quantum semantics are involved
 *          +--> networking model
 *          +--> distributed model
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / placement
 *          |
 *          v
 *     runtime / hardware realization
 *
 * THIS FILE MUST NEVER directly construct an IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical networking-channel declarations;
 *     - logical channel references;
 *     - source/destination relationships;
 *     - channel message/schema references;
 *     - channel protocol references;
 *     - channel direction;
 *     - channel delivery semantics;
 *     - channel ordering semantics;
 *     - channel reliability intent;
 *     - channel communication requirements;
 *     - channel communication constraints;
 *     - channel communication preferences;
 *     - channel metadata;
 *     - channel-scoped properties;
 *     - channel-scoped capability requirements;
 *     - channel-scoped expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic concurrency channels;
 *     - send/receive/select operations;
 *     - actor mailboxes;
 *     - task channels;
 *     - queue implementation;
 *     - buffering implementation;
 *     - channel memory allocation;
 *     - network endpoints;
 *     - network protocols themselves;
 *     - protocol implementation;
 *     - message schema definition;
 *     - serialization;
 *     - compression;
 *     - encryption;
 *     - authentication;
 *     - authorization;
 *     - routing algorithms;
 *     - physical topology;
 *     - network discovery;
 *     - placement;
 *     - scheduling;
 *     - hardware discovery;
 *     - device discovery;
 *     - CPU/GPU/FPGA/QPU topology;
 *     - quantum topology;
 *     - quantum gates;
 *     - quantum states;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime queues;
 *     - sockets;
 *     - IP addresses;
 *     - ports;
 *     - vendor APIs;
 *     - cloud-provider APIs.
 *
 * Generic concurrency channel semantics remain owned by:
 *
 *     grammar/concurrency/channels.g4
 *
 * Networking protocol syntax remains owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * Networking message schema syntax remains owned by:
 *
 *     grammar/networking/messages.g4
 *
 * Networking endpoints remain owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * Networking services remain owned by:
 *
 *     grammar/networking/services.g4
 *
 * ============================================================================
 * CRITICAL CHANNEL OWNERSHIP DISTINCTION
 * ============================================================================
 *
 * Zamani has two related but different abstractions:
 *
 * 1. CONCURRENCY CHANNEL
 *
 *     A language-level communication primitive used by concurrent computation.
 *
 *     Example:
 *
 *         channel values: Channel<T>;
 *
 *     It owns operations such as:
 *
 *         send
 *         receive
 *         select
 *         close
 *
 *     That is grammar/concurrency/channels.g4.
 *
 *
 * 2. NETWORKING CHANNEL
 *
 *     A logical communication relationship between networking participants.
 *
 *     Example:
 *
 *         channel telemetry {
 *             source: sensor;
 *             destination: collector;
 *             message: telemetry::Measurement;
 *         }
 *
 *     It does NOT itself perform send/receive.
 *
 * This distinction prevents networking from becoming a duplicate concurrency
 * system and prevents concurrency from becoming a physical network model.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A networking channel describes WHAT communication relationship is required.
 *
 * It does not permanently encode WHERE or HOW that relationship is realized.
 *
 * Therefore this grammar MUST NOT require:
 *
 *     - a machine;
 *     - a node;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - a QPU;
 *     - a process;
 *     - a thread;
 *     - a socket;
 *     - an IP address;
 *     - a port;
 *     - a router;
 *     - a switch;
 *     - a subnet;
 *     - a network provider;
 *     - a cloud provider;
 *     - a geographic region;
 *     - a fixed topology.
 *
 * The same logical channel can therefore be lowered to:
 *
 *     - in-process communication;
 *     - local IPC;
 *     - shared memory;
 *     - accelerator communication;
 *     - hardware fabrics;
 *     - local networks;
 *     - wide-area networks;
 *     - distributed systems;
 *     - cloud systems;
 *     - quantum/classical networks;
 *     - future communication substrates.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar constants such as:
 *
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS_PER_CHANNEL
 *     MAX_MESSAGES
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_CHANNEL_DEPTH
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_PAYLOAD_SIZE
 *
 * All collections use unbounded parser repetition.
 *
 * Any practical limit comes from:
 *
 *     - available memory;
 *     - compiler implementation;
 *     - parser implementation;
 *     - resource policy;
 *     - runtime;
 *     - deployment;
 *     - hardware.
 *
 * Such limits are NOT source-language grammar limits.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar deliberately does not enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     RDMA
 *     InfiniBand
 *     MPI
 *     HTTP
 *     custom_transport
 *
 * as channel grammar concepts.
 *
 * Protocols are referenced symbolically and resolved by semantic/capability
 * analysis.
 *
 * Likewise, delivery and ordering semantics are represented through
 * expressions/names rather than an exhaustive closed enumeration.
 *
 * This allows future networking technologies to be introduced without
 * modifying the fundamental channel grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The current canonical lexer intentionally does not provide a dedicated
 * networking CHANNEL keyword.
 *
 * Consequently the networking channel introducer is parsed as a contextual
 * identifier marker:
 *
 *     channel ...
 *
 * Semantic analysis validates that the contextual spelling is being used in
 * the correct networking declaration position.
 *
 * This avoids introducing a second lexer authority.
 *
 * IMPORTANT:
 *
 * `channel` here MUST NOT be confused with the CHANNEL token used by
 * concurrency/channels.g4 if/when the lexer evolves to reserve that spelling.
 *
 * If the canonical language specification eventually reserves `channel`,
 * this parser file should be migrated to the canonical CHANNEL token in the
 * same compatibility change as the lexer update.
 *
 * No other networking-specific keyword is required.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Canonical shared grammar concepts are consumed from the existing grammar
 * architecture:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     attribute
 *
 * This file does NOT redefine those concepts.
 *
 * ============================================================================
 */

parser grammar NetworkingChannels;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable integration boundary for:
 *
 *     grammar/networking/networking.g4
 *
 * and any future networking parser composition layer.
 *
 * ============================================================================
 */

networkChannelConstruct
    : networkChannelDeclaration
    | networkChannelReference
    ;


/*
 * ============================================================================
 * NETWORK CHANNEL DECLARATION
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     channel telemetry {
 *         source: sensor;
 *         destination: collector;
 *         message: telemetry::Measurement;
 *         protocol: reliable_transport;
 *     }
 *
 * The declaration describes a logical communication relationship.
 *
 * It does not instantiate anything.
 *
 * ============================================================================
 */

networkChannelDeclaration
    : attribute*
      networkChannelMarker
      identifier
      networkChannelBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL CHANNEL MARKER
 * ============================================================================
 *
 * `channel` is intentionally represented as a contextual identifier rather
 * than introducing another lexer rule.
 *
 * Semantic analysis MUST validate the exact spelling.
 * ============================================================================
 */

networkChannelMarker
    : identifier
    ;


/*
 * ============================================================================
 * CHANNEL BODY
 * ============================================================================
 */

networkChannelBody
    : LBRACE
      networkChannelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * CHANNEL MEMBERS
 * ============================================================================
 *
 * The member grammar is intentionally property-oriented.
 *
 * This keeps the channel grammar extensible without embedding an exhaustive
 * list of transport technologies.
 *
 * ============================================================================
 */

networkChannelMember
    : attribute*
      networkChannelSource
    | attribute*
      networkChannelDestination
    | attribute*
      networkChannelMessage
    | attribute*
      networkChannelProtocol
    | attribute*
      networkChannelDirection
    | attribute*
      networkChannelDelivery
    | attribute*
      networkChannelOrdering
    | attribute*
      networkChannelReliability
    | attribute*
      networkChannelRequirement
    | attribute*
      networkChannelConstraint
    | attribute*
      networkChannelPreference
    | attribute*
      networkChannelCapability
    | attribute*
      networkChannelProperty
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * Identifies the logical source of communication.
 *
 * It is NOT:
 *
 *     - a host;
 *     - a socket;
 *     - a physical device;
 *     - a process;
 *     - a network address.
 *
 * ============================================================================
 */

networkChannelSource
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * DESTINATION
 * ============================================================================
 */

networkChannelDestination
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * MESSAGE / SCHEMA REFERENCE
 * ============================================================================
 *
 * The referenced value is intentionally an expression rather than a second
 * message-schema grammar.
 *
 * Canonical message schemas belong to:
 *
 *     grammar/networking/messages.g4
 *
 * Data schemas remain owned by:
 *
 *     grammar/data/schemas.g4
 *
 * ============================================================================
 */

networkChannelMessage
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCE
 * ============================================================================
 *
 * Protocol syntax belongs to:
 *
 *     grammar/networking/protocols.g4
 *
 * This rule merely provides the channel-level reference boundary.
 *
 * ============================================================================
 */

networkChannelProtocol
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * DIRECTION
 * ============================================================================
 *
 * Direction describes communication intent.
 *
 * It does not imply:
 *
 *     - network topology;
 *     - transport implementation;
 *     - socket direction;
 *     - physical link direction.
 *
 * The value remains an expression so future communication models can be
 * introduced without changing this grammar.
 *
 * ============================================================================
 */

networkChannelDirection
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * DELIVERY SEMANTICS
 * ============================================================================
 *
 * Examples that may be represented semantically include:
 *
 *     reliable
 *     best_effort
 *     at_most_once
 *     at_least_once
 *     exactly_once
 *     ordered
 *     unordered
 *     custom::delivery
 *
 * The grammar deliberately does not enumerate these.
 *
 * Their legality and meaning are semantic contracts.
 *
 * ============================================================================
 */

networkChannelDelivery
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * ORDERING
 * ============================================================================
 */

networkChannelOrdering
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * RELIABILITY
 * ============================================================================
 */

networkChannelReliability
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory for a legal realization.
 *
 * It is NOT an allocation request.
 *
 * ============================================================================
 */

networkChannelRequirement
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts legal realizations.
 *
 * It does not necessarily select one realization.
 *
 * ============================================================================
 */

networkChannelConstraint
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * A preference is weaker than a requirement.
 *
 * The compiler/runtime may choose another realization if required for
 * correctness, feasibility, availability or stronger constraints.
 *
 * ============================================================================
 */

networkChannelPreference
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * A capability expresses a property required from the eventual realization.
 *
 * It does not identify a specific machine.
 *
 * ============================================================================
 */

networkChannelCapability
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * OPEN PROPERTY
 * ============================================================================
 *
 * This is the extensibility boundary.
 *
 * Example:
 *
 *     latency: requirement::bounded;
 *     bandwidth: preference::high;
 *     security: secure;
 *     locality: near(source);
 *
 * Unknown properties are NOT automatically semantically valid.
 *
 * The semantic layer must resolve:
 *
 *     - ownership;
 *     - type;
 *     - capability;
 *     - legality;
 *     - version;
 *     - namespace.
 *
 * The grammar merely preserves the syntax.
 *
 * ============================================================================
 */

networkChannelProperty
    : networkChannelPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 *
 * Property names are ordinary identifiers.
 *
 * Qualified names are allowed so extensions can be namespaced without adding
 * new keywords.
 *
 * ============================================================================
 */

networkChannelPropertyName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCE
 * ============================================================================
 *
 * A reference identifies a previously declared logical networking channel.
 *
 * ============================================================================
 */

networkChannelReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCE LIST
 * ============================================================================
 *
 * Unbounded by design.
 * ============================================================================
 */

networkChannelReferenceList
    : networkChannelReference
      (COMMA networkChannelReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL CHANNEL REFERENCE LIST
 * ============================================================================
 */

optionalNetworkChannelReferenceList
    : networkChannelReferenceList?
    ;


/*
 * ============================================================================
 * CHANNEL PROPERTY VALUE
 * ============================================================================
 *
 * Dedicated boundary for semantic consumers that need to identify a channel
 * property expression without inventing another expression language.
 * ============================================================================
 */

networkChannelPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * CHANNEL TYPE REFERENCE
 * ============================================================================
 *
 * A channel may optionally expose a logical payload type.
 *
 * This is deliberately only a reference to the canonical type system.
 *
 * ============================================================================
 */

networkChannelTypeReference
    : typeExpression
    ;


/*
 * ============================================================================
 * CHANNEL TYPE ANNOTATION
 * ============================================================================
 *
 * Example:
 *
 *     payload_type: SomeType;
 *
 * ============================================================================
 */

networkChannelTypeAnnotation
    : networkChannelPropertyName
      COLON
      networkChannelTypeReference
      SEMI
    ;


/*
 * ============================================================================
 * CHANNEL EXPRESSION REFERENCE
 * ============================================================================
 *
 * A logical networking channel can participate in ordinary Zamani expressions
 * without creating a networking-specific expression language.
 *
 * ============================================================================
 */

networkChannelExpression
    : networkChannelReference
    ;


/*
 * ============================================================================
 * CHANNEL PROPERTY LIST
 * ============================================================================
 *
 * Explicit list boundary for tooling and semantic analysis.
 * ============================================================================
 */

networkChannelPropertyList
    : networkChannelProperty*
    ;


/*
 * ============================================================================
 * CHANNEL DECLARATION LIST
 * ============================================================================
 *
 * Unbounded.
 * ============================================================================
 */

networkChannelDeclarationList
    : networkChannelDeclaration*
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCE OR DECLARATION
 * ============================================================================
 */

networkChannelItem
    : networkChannelDeclaration
    | networkChannelReference
    ;