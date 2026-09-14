/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Networking Message Grammar
 * ============================================================================
 *
 * File:
 *     grammar/networking/messages.g4
 *
 * Grammar:
 *     Messages
 *
 * Status:
 *     Production-target canonical networking message grammar.
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
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL STRUCTURAL SYNTAX of logical
 * messages in Zamani.
 *
 * A message is a typed, named semantic value that may be exchanged between
 * logical computational entities.
 *
 * A message may ultimately be realized through:
 *
 *     - a function boundary;
 *     - an in-process channel;
 *     - a task;
 *     - an actor;
 *     - shared memory;
 *     - IPC;
 *     - a distributed service;
 *     - a network;
 *     - a hardware communication fabric;
 *     - a classical/quantum boundary;
 *     - an accelerator;
 *     - a future computational substrate.
 *
 * This grammar describes the logical message.
 *
 * It does NOT prescribe how the message is transported, represented,
 * serialized, scheduled, encrypted, routed, stored, or executed.
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
 *     - message field defaults;
 *     - message construction syntax;
 *     - message type references;
 *     - message argument syntax;
 *     - message schema attributes;
 *     - message declaration extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical syntax;
 *     - identifiers;
 *     - qualified names;
 *     - types;
 *     - general expressions;
 *     - functions;
 *     - modules;
 *     - channels;
 *     - endpoints;
 *     - network protocols;
 *     - services;
 *     - routing;
 *     - network topology;
 *     - placement;
 *     - scheduling;
 *     - serialization;
 *     - compression;
 *     - encryption;
 *     - authentication;
 *     - authorization;
 *     - identity;
 *     - replication;
 *     - consistency;
 *     - distributed fault tolerance;
 *     - runtime queues;
 *     - runtime transport;
 *     - hardware;
 *     - quantum topology;
 *     - quantum gates;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - canonical IR.
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
 *     parser
 *          |
 *          v
 *     Messages
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> schema validation
 *          +--> capability analysis
 *          +--> effect analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical semantic IR
 *          +--> distributed semantic model
 *          +--> networking semantic model
 *          +--> data/schema model
 *          +--> quantum::ir where quantum semantics are actually involved
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
 * Messages MUST NOT directly construct or mutate an IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A message declaration expresses WHAT information exists and may be
 * exchanged, not WHERE or HOW it is transported.
 *
 * Therefore this grammar intentionally contains no:
 *
 *     machine count;
 *     node count;
 *     process count;
 *     thread count;
 *     CPU count;
 *     GPU count;
 *     QPU count;
 *     FPGA count;
 *     memory capacity;
 *     network capacity;
 *     network address;
 *     network port;
 *     topology;
 *     provider;
 *     region;
 *     device identifier;
 *     transport;
 *     packet size;
 *     wire offset;
 *     serialization format.
 *
 * The same message schema can therefore be used on:
 *
 *     one machine;
 *     many machines;
 *     an embedded system;
 *     a cluster;
 *     a supercomputer;
 *     a cloud;
 *     a quantum/classical system;
 *     future computational substrates.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite source-language limits are imposed on:
 *
 *     - number of message declarations;
 *     - number of fields;
 *     - number of message arguments;
 *     - number of message types;
 *     - number of nested type structures;
 *     - number of modules;
 *     - number of communication participants;
 *     - payload size;
 *     - deployment size.
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * Actual limits arising from:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler policy;
 *     - operating-system resources;
 *     - runtime resources;
 *     - deployment resources
 *
 * remain implementation/resource constraints and MUST NOT become grammar
 * constants.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar intentionally does NOT enumerate communication operations.
 *
 * It therefore does not reserve:
 *
 *     send
 *     receive
 *     request
 *     reply
 *     publish
 *     subscribe
 *     broadcast
 *     multicast
 *
 * as message-language primitives.
 *
 * Such operations belong to channels, protocols, services, distributed
 * execution, or user/library abstractions.
 *
 * A future communication model must not require modification of this grammar
 * merely because a new transport or communication operation is introduced.
 *
 * ============================================================================
 * TYPE SYSTEM BOUNDARY
 * ============================================================================
 *
 * Message fields consume canonical `typeExpression`.
 *
 * This grammar therefore does NOT define:
 *
 *     Optional<T>
 *     List<T>
 *     Map<K,V>
 *     Result<T,E>
 *     tuple types
 *     quantum types
 *     hardware types
 *     resource types
 *     tensor types
 *
 * Those remain owned by the canonical type system.
 *
 * Optionality, multiplicity, resource semantics, quantum semantics, and
 * hardware semantics are therefore represented by the type system and
 * semantic layers rather than by a second messaging-specific type language.
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Message construction arguments and field defaults consume canonical
 * `expression`.
 *
 * This grammar does not redefine:
 *
 *     arithmetic;
 *     calls;
 *     indexing;
 *     conditionals;
 *     literals;
 *     operators;
 *     lambdas;
 *     compile-time expressions.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A message may contain a canonical quantum value if the type system permits
 * it.
 *
 * This grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     LogicalQubitId
 *     GateKind
 *     QuantumState IR
 *     circuit IR
 *     topology
 *     calibration
 *     QEC
 *     ZQN
 *     resilience.
 *
 * If message data participates in quantum computation, semantic lowering is
 * responsible for integrating the resulting operation/data semantics with
 * the canonical `quantum::ir`.
 *
 * The grammar remains independent of physical quantum resources.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Message fields use canonical types and expressions.
 *
 * Consequently messages may participate in:
 *
 *     classical computation;
 *     quantum/classical computation;
 *     HDL control;
 *     hardware/software co-design;
 *     accelerator workflows;
 *     distributed computation;
 *     AI/data pipelines;
 *     embedded systems;
 *     scientific computing.
 *
 * This file introduces no hardware-specific syntax.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * A message is NOT a packet.
 *
 * A message does not contain:
 *
 *     IP address;
 *     port;
 *     socket;
 *     MAC address;
 *     route;
 *     transport;
 *     packet header;
 *     MTU;
 *     network topology.
 *
 * Those belong to:
 *
 *     networking/endpoints.g4
 *     networking/protocols.g4
 *     networking/channels.g4
 *     networking/services.g4
 *     networking/network-capabilities.g4
 *
 * and their semantic/runtime subsystems.
 *
 * ============================================================================
 * SERIALIZATION BOUNDARY
 * ============================================================================
 *
 * Message declarations are logical schemas.
 *
 * This grammar does NOT choose:
 *
 *     JSON;
 *     CBOR;
 *     protobuf;
 *     custom binary;
 *     textual encoding;
 *     zero-copy layout;
 *     ABI layout;
 *     packet layout.
 *
 * The same message schema may have many physical representations.
 *
 * Serialization belongs downstream, principally to the data/interoperability
 * layers.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Security is not encoded as transport behavior here.
 *
 * Message declarations may carry generic language attributes, which semantic
 * analysis may interpret through the security subsystem.
 *
 * This grammar does not implement:
 *
 *     encryption;
 *     signatures;
 *     authentication;
 *     authorization;
 *     identity;
 *     trust;
 *     key management.
 *
 * ============================================================================
 * DISTRIBUTED-SYSTEM INTEGRATION
 * ============================================================================
 *
 * This file becomes the canonical owner of MESSAGE SCHEMA SYNTAX.
 *
 * Existing:
 *
 *     grammar/distributed/messaging.g4
 *
 * MUST NOT remain a second independent message-schema grammar.
 *
 * It should become a compatibility/delegation grammar for distributed
 * constructs that consume the canonical rules from this file.
 *
 * Distributed semantics such as:
 *
 *     replication;
 *     consistency;
 *     delivery;
 *     retry;
 *     actor behavior;
 *     node placement;
 *     remote execution;
 *
 * remain outside this file.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime callbacks;
 *     - no hardware discovery;
 *     - no randomness.
 *
 * Parsing is therefore a deterministic function of the token stream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * A message declaration should lower into an AST representation containing
 * at least:
 *
 *     declaration attributes;
 *     qualified message name;
 *     ordered field declarations;
 *     field names;
 *     field type syntax;
 *     optional default expressions;
 *     source spans.
 *
 * A message value should preserve:
 *
 *     message type reference;
 *     ordered arguments;
 *     source spans.
 *
 * The parser MUST NOT resolve:
 *
 *     names;
 *     types;
 *     transports;
 *     endpoints;
 *     devices;
 *     resources;
 *     capabilities.
 *
 * Those belong to later semantic phases.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for validating:
 *
 *     - duplicate field names;
 *     - unresolved message references;
 *     - invalid field types;
 *     - invalid default values;
 *     - argument/type correspondence;
 *     - required/optional semantics;
 *     - recursive schema legality;
 *     - visibility;
 *     - module ownership;
 *     - capabilities;
 *     - effects;
 *     - resource requirements;
 *     - serialization compatibility;
 *     - security constraints.
 *
 * None of those checks should be implemented as parser actions.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar deliberately preserves the core source shape of the existing
 * distributed messaging grammar:
 *
 *     message Name {
 *         field: Type;
 *     }
 *
 * and:
 *
 *     Name(value1, value2)
 *
 * Existing message syntax should therefore migrate without changing its
 * semantic meaning.
 *
 * Deprecated distributed-message rules should delegate to these canonical
 * rules rather than maintain duplicate definitions.
 *
 * ============================================================================
 */

parser grammar Messages;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable parser-composition boundary for networking messages.
 *
 * A higher-level networking grammar may consume:
 *
 *     messageConstruct
 *
 * without depending on implementation details of the individual productions.
 */
messageConstruct
    : messageDeclaration
    | messageValue
    ;


/* ============================================================================
 * MESSAGE DECLARATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 * Attributes are delegated to the canonical attribute grammar.
 *
 * The message name is a qualified source name. Whether a qualified message
 * declaration is legal in a particular scope is a semantic/module concern.
 */
messageDeclaration
    : attribute*
      MESSAGE
      qualifiedName
      messageGenericParameters?
      LBRACE
      messageMember*
      RBRACE
    ;


/* ============================================================================
 * MESSAGE GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic message schemas are source-level abstractions.
 *
 * They do not represent machine resources.
 *
 * Example:
 *
 *     message Envelope<T> {
 *         payload: T;
 *     }
 *
 * Generic parameter validity and substitution are semantic/type-system
 * responsibilities.
 *
 * The rule is deliberately local so this grammar does not invent a second
 * generic type system.
 */
messageGenericParameters
    : LT
      messageGenericParameterList
      GT
    ;


messageGenericParameterList
    : messageGenericParameter
      (COMMA messageGenericParameter)*
      COMMA?
    ;


messageGenericParameter
    : identifier
      messageGenericParameterConstraint?
    ;


messageGenericParameterConstraint
    : COLON
      typeExpression
    ;


/* ============================================================================
 * MESSAGE MEMBERS
 * ============================================================================
 *
 * The initial production surface deliberately contains fields only.
 *
 * This prevents arbitrary executable statements from becoming legal inside a
 * message schema.
 *
 * Future schema members must be introduced deliberately and assigned explicit
 * ownership.
 */
messageMember
    : attribute*
      messageField
    ;


/* ============================================================================
 * MESSAGE FIELD
 * ============================================================================
 *
 * Canonical form:
 *
 *     fieldName: FieldType;
 *
 * Optional defaults:
 *
 *     fieldName: FieldType = expression;
 *
 * Optionality itself is represented by the canonical type system, for example:
 *
 *     fieldName: Option<T>;
 *
 * rather than a messaging-specific `optional` type construct.
 */
messageField
    : identifier
      COLON
      typeExpression
      messageFieldInitializer?
      SEMI
    ;


messageFieldInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * MESSAGE VALUE
 * ============================================================================
 *
 * Canonical positional construction:
 *
 *     UserCreated(id, name)
 *
 *     events::UserCreated(id, name)
 *
 * This grammar does not decide whether constructing the value:
 *
 *     allocates;
 *     copies;
 *     serializes;
 *     sends;
 *     queues;
 *     encrypts;
 *     schedules;
 *     transmits.
 *
 * Those are downstream semantic/runtime concerns.
 */
messageValue
    : messageTypeReference
      LPAREN
      messageArgumentList?
      RPAREN
    ;


/* ============================================================================
 * MESSAGE TYPE REFERENCE
 * ============================================================================
 *
 * A message type reference is a canonical qualified name.
 *
 * Semantic resolution determines whether the referenced declaration is
 * actually a message type.
 */
messageTypeReference
    : qualifiedName
    ;


/* ============================================================================
 * MESSAGE ARGUMENTS
 * ============================================================================
 *
 * No finite argument count is encoded.
 *
 * Every argument is delegated to the canonical expression grammar.
 */
messageArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * MESSAGE FIELD REFERENCE
 * ============================================================================
 *
 * This is intentionally only a qualified name.
 *
 * Semantic analysis determines whether it denotes a field and whether the
 * reference is legal in its surrounding context.
 */
messageFieldReference
    : qualifiedName
    ;


/* ============================================================================
 * MESSAGE TYPE REFERENCE LIST
 * ============================================================================
 *
 * Useful to networking/distributed consumers that need to declare a set of
 * accepted/related message types.
 *
 * The list has no artificial finite size.
 */
messageTypeReferenceList
    : messageTypeReference
      (COMMA messageTypeReference)*
      COMMA?
    ;


/* ============================================================================
 * MESSAGE FIELD REFERENCE LIST
 * ============================================================================
 *
 * Reusable semantic boundary for consumers such as protocol/service/schema
 * analysis.
 */
messageFieldReferenceList
    : messageFieldReference
      (COMMA messageFieldReference)*
      COMMA?
    ;