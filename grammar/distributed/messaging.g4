/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Distributed Messaging Grammar
 * ============================================================================
 *
 * File:
 *     grammar/distributed/messaging.g4
 *
 * Grammar:
 *     Messaging
 *
 * Status:
 *     Production-target distributed messaging grammar.
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
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
 *     - No backend discovery.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL MESSAGE STRUCTURE.
 *
 * Messaging is the semantic abstraction for information exchanged between
 * logical computational entities.
 *
 * A message may ultimately be transported through:
 *
 *     - an in-process channel;
 *     - a task channel;
 *     - an actor mailbox;
 *     - shared memory;
 *     - a local IPC mechanism;
 *     - a network;
 *     - a distributed service;
 *     - a cluster;
 *     - a cloud deployment;
 *     - a hardware communication fabric;
 *     - a quantum/classical hybrid system;
 *     - a future computational substrate.
 *
 * The grammar does not choose the realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - message declarations;
 *     - message schema structure;
 *     - message fields;
 *     - message field initializers;
 *     - message type references;
 *     - message construction syntax;
 *     - message payload syntax;
 *     - message envelope references;
 *     - message schema metadata boundaries;
 *     - message extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - lexical tokens;
 *     - network protocols;
 *     - network endpoints;
 *     - channels;
 *     - routing;
 *     - service discovery;
 *     - placement;
 *     - scheduling;
 *     - serialization algorithms;
 *     - compression algorithms;
 *     - encryption algorithms;
 *     - authentication;
 *     - authorization;
 *     - replication;
 *     - consistency;
 *     - fault tolerance;
 *     - retry policy;
 *     - distributed execution;
 *     - actor runtime;
 *     - runtime queues;
 *     - mailbox implementation;
 *     - hardware;
 *     - quantum topology;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * Those concerns remain owned by their respective repository subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     Messaging
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> schema validation
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> security analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> distributed semantic model
 *          +--> networking model
 *          +--> data/schema model
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / placement
 *          |
 *          v
 *     runtime realization
 *
 * Messaging MUST NOT directly construct or modify any IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Message syntax describes the logical information being exchanged.
 *
 * It does NOT encode:
 *
 *     - node count;
 *     - process count;
 *     - thread count;
 *     - CPU count;
 *     - GPU count;
 *     - QPU count;
 *     - memory capacity;
 *     - network capacity;
 *     - network address;
 *     - network port;
 *     - transport protocol;
 *     - machine identifier;
 *     - device identifier;
 *     - cluster size;
 *     - topology;
 *     - provider;
 *     - region.
 *
 * Consequently the same message schema may be realized on arbitrarily small
 * or arbitrarily large available infrastructure.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar constants such as:
 *
 *     MAX_MESSAGES
 *     MAX_FIELDS
 *     MAX_PAYLOAD_SIZE
 *     MAX_MESSAGE_DEPTH
 *     MAX_MESSAGE_TYPES
 *     MAX_NODES
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_REPLICAS
 *     MAX_SCHEMA_SIZE
 *
 * Repetition is represented through ANTLR's unbounded repetition operators.
 *
 * Practical limits imposed by:
 *
 *     - memory;
 *     - parser implementation;
 *     - compiler policy;
 *     - operating system;
 *     - deployment;
 *     - runtime;
 *     - available hardware
 *
 * are resource/implementation limits and MUST NOT become source-language
 * grammar limits.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Message operation names are intentionally NOT enumerated here.
 *
 * This grammar therefore does not hard-code:
 *
 *     send
 *     receive
 *     request
 *     reply
 *     broadcast
 *     multicast
 *     publish
 *     subscribe
 *
 * Those are communication/runtime concepts.
 *
 * A communication grammar may use message references and arbitrary expression
 * values without requiring messaging.g4 to change when a new communication
 * operation is introduced.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser consumes the canonical ZamaniLexer.
 *
 * The `message` declaration introducer is a LANGUAGE STRUCTURAL KEYWORD.
 *
 * It therefore requires the canonical lexer to expose:
 *
 *     MESSAGE : 'message' ;
 *
 * in grammar/antlr/ZamaniLexer.g4.
 *
 * `MESSAGE` is intentionally the ONLY messaging-specific structural keyword
 * required by this grammar.
 *
 * Message names, field names, operation names, schema names and future
 * extension names remain canonical identifiers.
 *
 * The grammar does NOT introduce:
 *
 *     SEND
 *     RECEIVE
 *     REQUEST
 *     REPLY
 *     BROADCAST
 *     MULTICAST
 *     PUBLISH
 *     SUBSCRIBE
 *
 * as lexer tokens.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Names:
 *
 *     identifier
 *     qualifiedName
 *
 * are owned by grammar/core/names.g4.
 *
 * Types:
 *
 *     typeExpression
 *
 * is owned by the canonical type grammar.
 *
 * Expressions:
 *
 *     expression
 *     expressionList
 *
 * are owned by grammar/expressions/expressions.g4.
 *
 * Attributes:
 *
 *     attribute
 *
 * is owned by grammar/core/attributes.g4.
 *
 * This file MUST NOT redefine those concepts.
 *
 * ============================================================================
 * MESSAGE MODEL
 * ============================================================================
 *
 * A message declaration describes a logical schema:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 * It does NOT describe:
 *
 *     - memory layout;
 *     - wire layout;
 *     - ABI;
 *     - serialization format;
 *     - network packet structure;
 *     - transport protocol.
 *
 * Those are downstream representations.
 *
 * ============================================================================
 * MESSAGE FIELD MODEL
 * ============================================================================
 *
 * A field has:
 *
 *     name
 *     type
 *     optional initializer
 *
 * Example:
 *
 *     message Measurement {
 *         value: Float;
 *         basis: Basis;
 *     }
 *
 * Optionality, multiplicity and richer algebraic structure should normally be
 * represented by the canonical type system:
 *
 *     Option<T>
 *     List<T>
 *     Map<K,V>
 *     Result<T,E>
 *     domain-specific types
 *
 * rather than creating another messaging-specific type system.
 *
 * ============================================================================
 * MESSAGE VALUE MODEL
 * ============================================================================
 *
 * A message value is represented structurally as:
 *
 *     QualifiedMessageName(arguments...)
 *
 * This grammar provides a dedicated parse boundary for semantic consumers.
 *
 * It does not determine whether construction:
 *
 *     allocates;
 *     copies;
 *     serializes;
 *     transmits;
 *     encrypts;
 *     queues;
 *     schedules.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Messages may contain quantum-related values because their payload is an
 * ordinary Zamani expression and their fields may use canonical types.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumState IR
 *     quantum topology
 *     calibration
 *     QEC
 *     ZQN.
 *
 * If a message contains a quantum value, semantic lowering determines how
 * that value interacts with `quantum::ir`.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Message fields may reference arbitrary canonical types and expressions.
 *
 * Therefore messaging can participate in:
 *
 *     classical computation;
 *     quantum/classical computation;
 *     HDL control;
 *     hardware/software co-design;
 *     accelerator workflows;
 *     distributed execution;
 *     AI/data pipelines.
 *
 * No hardware-specific representation is introduced here.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * A message is NOT a network packet.
 *
 * This grammar does not own:
 *
 *     IP addresses
 *     ports
 *     TCP
 *     UDP
 *     QUIC
 *     RDMA
 *     InfiniBand
 *     MPI
 *     vendor transports
 *
 * Networking grammars and runtime layers may map messages to those mechanisms.
 *
 * ============================================================================
 * SERIALIZATION BOUNDARY
 * ============================================================================
 *
 * Message schema syntax does not select a serialization format.
 *
 * The same logical message may eventually be represented using:
 *
 *     an in-memory representation;
 *     a binary representation;
 *     a textual representation;
 *     a zero-copy representation;
 *     a hardware representation;
 *     a future representation.
 *
 * Serialization is therefore downstream.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     encryption;
 *     signatures;
 *     authentication;
 *     authorization;
 *     identity;
 *     trust;
 *     key management.
 *
 * Security metadata may be attached through canonical attributes or security
 * constructs, but the security model remains owned by grammar/security/* and
 * the corresponding semantic subsystem.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar has:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no I/O;
 *     - no runtime decisions;
 *     - no randomness;
 *     - no hardware discovery.
 *
 * Parsing depends only on the token stream.
 *
 * ============================================================================
 */

parser grammar Messaging;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC DOMAIN ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary for distributed and higher-level parser
 * composition.
 */
messagingConstruct
    : messageDeclaration
    | messageValue
    | messageTypeReference
    ;


/*
 * ============================================================================
 * MESSAGE DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 * Attributes are generic language metadata and are therefore delegated to
 * Attributes.
 */
messageDeclaration
    : attribute*
      MESSAGE
      qualifiedName
      LBRACE
      messageMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MESSAGE MEMBERS
 * ============================================================================
 *
 * The schema body currently owns fields.
 *
 * Future schema constructs must be added through an explicit compatibility
 * decision rather than silently making arbitrary expressions valid members.
 */
messageMember
    : attribute*
      messageField
    ;


/*
 * ============================================================================
 * MESSAGE FIELD
 * ============================================================================
 *
 * Canonical form:
 *
 *     fieldName: FieldType;
 *
 * An initializer is permitted because the initializer is part of the source
 * schema declaration and is represented by the canonical expression grammar.
 *
 * The semantic layer decides whether a particular field type permits a
 * default/initializer.
 */
messageField
    : identifier
      COLON
      typeExpression
      messageFieldInitializer?
      SEMI
    ;


/*
 * ============================================================================
 * FIELD INITIALIZER
 * ============================================================================
 */
messageFieldInitializer
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * MESSAGE TYPE REFERENCE
 * ============================================================================
 *
 * A message reference is simply a canonical qualified name.
 *
 * Examples:
 *
 *     UserCreated
 *     events::UserCreated
 *     telemetry::Measurement
 *
 * This rule assigns no semantic meaning to the name.
 */
messageTypeReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * MESSAGE VALUE
 * ============================================================================
 *
 * Canonical construction form:
 *
 *     UserCreated(value1, value2)
 *
 * or:
 *
 *     events::UserCreated(value1, value2)
 *
 * The semantic layer validates:
 *
 *     - field count;
 *     - argument correspondence;
 *     - field types;
 *     - defaults;
 *     - ownership;
 *     - effects;
 *     - resource requirements.
 *
 * The grammar itself imposes no fixed argument count.
 */
messageValue
    : messageTypeReference
      LPAREN
      optionalMessageArgumentList
      RPAREN
    ;


/*
 * ============================================================================
 * MESSAGE ARGUMENT LIST
 * ============================================================================
 *
 * This is intentionally local rather than redefining expressionList.
 *
 * The list delegates every value to canonical `expression`.
 */
optionalMessageArgumentList
    : expressionList?
    ;


/*
 * ============================================================================
 * MESSAGE PAYLOAD
 * ============================================================================
 *
 * A payload is a semantic expression.
 *
 * This rule exists as an explicit ownership boundary for communication and
 * distributed semantic consumers.
 *
 * It does not imply serialization or transmission.
 */
messagePayload
    : expression
    ;


/*
 * ============================================================================
 * MESSAGE PAYLOAD LIST
 * ============================================================================
 *
 * Unbounded payload expressions.
 */
messagePayloadList
    : messagePayload
      (COMMA messagePayload)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL MESSAGE PAYLOAD LIST
 * ============================================================================
 */
optionalMessagePayloadList
    : messagePayloadList?
    ;


/*
 * ============================================================================
 * MESSAGE FIELD REFERENCE
 * ============================================================================
 *
 * Used by semantic consumers that need to distinguish a field reference from
 * a complete message type reference.
 *
 * The grammar intentionally keeps the structure generic.
 */
messageFieldReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * MESSAGE TYPE LIST
 * ============================================================================
 *
 * No finite number of message types is encoded.
 */
messageTypeReferenceList
    : messageTypeReference
      (COMMA messageTypeReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL MESSAGE TYPE LIST
 * ============================================================================
 */
optionalMessageTypeReferenceList
    : messageTypeReferenceList?
    ;