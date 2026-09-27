/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/networking/protocols.g4
 *
 * GRAMMAR IDENTITY
 * ----------------
 * Protocols
 *
 * STATUS
 * ------
 * Production networking protocol grammar
 *
 * PURPOSE
 * -------
 * Defines the SOURCE-LEVEL SYNTAX for logical communication protocols.
 *
 * A protocol is a portable communication contract.  It may describe:
 *
 *   - logical participants;
 *   - roles;
 *   - message references;
 *   - operations;
 *   - protocol refinement;
 *   - protocol composition;
 *   - capabilities;
 *   - requirements;
 *   - constraints;
 *   - preferences;
 *   - protocol properties;
 *   - protocol states;
 *   - protocol transitions;
 *   - protocol interaction intent.
 *
 * It does NOT implement communication.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *   Rust 2021
 *   Safe Rust only
 *
 * ANTLR:
 *
 *   ANTLR4 parser grammar
 *
 * This file contains no Rust actions.
 *
 * Therefore:
 *
 *   - no unsafe Rust;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime callbacks;
 *   - no randomness;
 *   - no environment inspection;
 *   - no backend discovery;
 *   - no semantic predicates.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                    Zamani source
 *                          |
 *                          v
 *                    ZamaniLexer
 *                          |
 *                          v
 *                    ZamaniParser
 *                          |
 *                          v
 *                     Networking
 *                          |
 *                          v
 *                       Protocols
 *                          |
 *                          v
 *                   Frontend AST
 *                          |
 *                          v
 *                 Semantic analysis
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *          Networking   Distributed   Security
 *           semantics    semantics    semantics
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                 Canonical semantic model
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        Classical     quantum::ir    HDL/Hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                   optimization
 *                          |
 *                  routing / placement
 *                          |
 *                      scheduling
 *                          |
 *                      resilience
 *                          |
 *                         ZQN
 *                          |
 *                         HAL
 *                          |
 *                 target realization
 *
 * THIS FILE DOES NOT CREATE IR.
 *
 * ============================================================================
 * SINGLE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - protocol declaration syntax;
 *   - protocol identity;
 *   - protocol refinement syntax;
 *   - protocol composition references;
 *   - protocol roles;
 *   - message references;
 *   - operation contracts;
 *   - protocol requirements;
 *   - protocol capabilities;
 *   - protocol constraints;
 *   - protocol preferences;
 *   - protocol properties;
 *   - protocol states;
 *   - protocol transitions;
 *   - protocol interaction structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - qualified names;
 *   - types;
 *   - expressions;
 *   - attributes;
 *   - modules;
 *   - endpoints;
 *   - channels;
 *   - message schemas;
 *   - services;
 *   - sockets;
 *   - ports;
 *   - IP addresses;
 *   - MAC addresses;
 *   - DNS;
 *   - transport implementation;
 *   - serialization;
 *   - compression;
 *   - cryptography;
 *   - authentication implementation;
 *   - authorization implementation;
 *   - routing;
 *   - scheduling;
 *   - placement;
 *   - distributed execution;
 *   - hardware discovery;
 *   - hardware topology;
 *   - resource allocation;
 *   - runtime queues;
 *   - QEC;
 *   - ZQN;
 *   - quantum::ir;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A protocol describes WHAT communication contract exists.
 *
 * It does not permanently determine:
 *
 *   - WHERE it executes;
 *   - WHICH machine executes it;
 *   - WHICH node executes it;
 *   - WHICH CPU executes it;
 *   - WHICH GPU executes it;
 *   - WHICH FPGA executes it;
 *   - WHICH QPU executes it;
 *   - WHICH network interface is used;
 *   - WHICH address is used;
 *   - WHICH socket is used;
 *   - WHICH port is used;
 *   - WHICH transport is used;
 *   - WHICH provider is used;
 *   - WHICH topology is used;
 *   - HOW MANY participants physically exist.
 *
 * This is required for:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar intentionally DOES NOT enumerate:
 *
 *   TCP
 *   UDP
 *   QUIC
 *   HTTP
 *   HTTPS
 *   MQTT
 *   gRPC
 *   MPI
 *   RDMA
 *   InfiniBand
 *   Ethernet
 *   Bluetooth
 *   Wi-Fi
 *   quantum-network protocols
 *   vendor transports
 *   cloud transports
 *
 * A protocol implementation may be referred to through:
 *
 *   - qualified names;
 *   - capabilities;
 *   - dialects;
 *   - libraries;
 *   - semantic registries;
 *   - target descriptions.
 *
 * Adding a future protocol therefore does not require changing this grammar
 * merely because its name is new.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexical boundary remains:
 *
 *   grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *   tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT define lexer rules.
 *
 * IMPORTANT:
 *
 * The currently inspected canonical keyword vocabulary contains:
 *
 *   EXTENDS
 *   WHERE
 *   REQUIRES
 *   CONSTRAINT
 *   CAPABILITY
 *   PREFER
 *   PROPERTY
 *
 * but does not establish dedicated canonical tokens for every protocol word.
 *
 * Therefore the following remain CONTEXTUAL protocol markers:
 *
 *   protocol
 *   role
 *   message
 *   operation
 *   uses
 *   provides
 *   state
 *   transition
 *   send
 *   receive
 *   emit
 *   on
 *   from
 *   to
 *   flow
 *   sequence
 *   choice
 *   parallel
 *   repeat
 *   optional
 *   wait
 *
 * They are structurally recognized as identifiers and semantically validated
 * by the frontend.
 *
 * This avoids introducing a second lexical authority and avoids requiring a
 * keyword-registry edit merely to compile this independent grammar.
 *
 * ============================================================================
 * IMPORT AUTHORITY
 * ============================================================================
 *
 * This grammar consumes the canonical shared parser rules:
 *
 *   Names
 *   Types
 *   Expressions
 *
 * Those provide:
 *
 *   identifier
 *   qualifiedName
 *   typeExpression
 *   expression
 *
 * This file does NOT redefine those rules.
 *
 * ============================================================================
 * MESSAGE INTEGRATION
 * ============================================================================
 *
 * Message schemas are owned by:
 *
 *   grammar/networking/messages.g4
 *
 * Protocol syntax only references messages.
 *
 * It does NOT duplicate:
 *
 *   - message fields;
 *   - message schemas;
 *   - serialization;
 *   - wire representation.
 *
 * ============================================================================
 * ENDPOINT INTEGRATION
 * ============================================================================
 *
 * Endpoints are owned by:
 *
 *   grammar/networking/endpoints.g4
 *
 * Protocol roles are logical roles.
 *
 * They do not identify physical endpoints.
 *
 * ============================================================================
 * CHANNEL INTEGRATION
 * ============================================================================
 *
 * Networking channels are owned by:
 *
 *   grammar/networking/channels.g4
 *
 * Protocols may reference logical channels through expressions or names.
 *
 * Protocol syntax does not create runtime channels.
 *
 * ============================================================================
 * SERVICE INTEGRATION
 * ============================================================================
 *
 * Services are owned by:
 *
 *   grammar/networking/services.g4
 *
 * A protocol can be referenced by a service contract without defining service
 * deployment or runtime dispatch.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Networking capability semantics are owned by:
 *
 *   grammar/networking/network-capabilities.g4
 *
 * This file may reference capabilities but does not perform capability
 * discovery.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Protocol requirements must distinguish:
 *
 *   requirement
 *   constraint
 *   preference
 *   capability
 *   hint
 *
 * They must NOT become:
 *
 *   physical allocation
 *   device selection
 *   machine selection
 *   topology selection
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Protocol semantics may participate in distributed computation.
 *
 * However:
 *
 *   networking != distributed execution
 *
 * Distributed execution remains owned by:
 *
 *   grammar/distributed/
 *
 * That subsystem owns:
 *
 *   - nodes;
 *   - placement;
 *   - replication;
 *   - distributed scheduling;
 *   - consistency;
 *   - distributed recovery.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * A protocol may express requirements such as:
 *
 *   requires security::confidentiality;
 *   requires security::authentication;
 *   requires security::integrity;
 *
 * through canonical names.
 *
 * This grammar does not define cryptographic algorithms or security engines.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Protocols may coordinate:
 *
 *   - distributed quantum computation;
 *   - quantum-classical computation;
 *   - remote quantum execution;
 *   - quantum networking;
 *   - distributed QEC workflows;
 *   - quantum services.
 *
 * This grammar MUST NOT define:
 *
 *   QubitId
 *   PhysicalQubitId
 *   LogicalQubitId
 *   GateKind
 *   QuantumState
 *   QuantumTopology
 *   Calibration
 *   Pulse
 *   QEC implementation
 *   ZQN implementation
 *
 * If protocol semantics involve quantum computation:
 *
 *   protocol AST
 *       ->
 *   semantic analysis
 *       ->
 *   quantum semantic model
 *       ->
 *   quantum::ir
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Protocols may describe logical communication interfaces used by:
 *
 *   - hardware/software co-design;
 *   - accelerators;
 *   - FPGA designs;
 *   - ASIC designs;
 *   - embedded systems.
 *
 * This grammar does NOT define:
 *
 *   - wires;
 *   - pins;
 *   - clocks;
 *   - buses;
 *   - registers;
 *   - physical links;
 *   - FPGA routing;
 *   - ASIC routing;
 *   - physical topology.
 *
 * Those remain owned by:
 *
 *   grammar/hdl/
 *   grammar/hardware/
 *
 * ============================================================================
 * SCALABILITY / NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * There are deliberately NO grammar constants for:
 *
 *   MAX_PROTOCOLS
 *   MAX_ROLES
 *   MAX_MESSAGES
 *   MAX_OPERATIONS
 *   MAX_STATES
 *   MAX_TRANSITIONS
 *   MAX_PARAMETERS
 *   MAX_REQUIREMENTS
 *   MAX_CAPABILITIES
 *   MAX_PARTICIPANTS
 *   MAX_ENDPOINTS
 *   MAX_CHANNELS
 *   MAX_NETWORKS
 *   MAX_NODES
 *   MAX_CONNECTIONS
 *   MAX_PROTOCOL_DEPTH
 *   MAX_MESSAGE_SIZE
 *   MAX_BANDWIDTH
 *   MAX_LATENCY
 *
 * Repetition is unbounded at the language level.
 *
 * Practical limits may arise from:
 *
 *   - available memory;
 *   - compiler implementation;
 *   - parser implementation;
 *   - resource policy;
 *   - operating-system limits;
 *   - runtime resources;
 *   - deployment resources;
 *   - target capabilities.
 *
 * Those are NOT language-level protocol limits.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *   - source;
 *   - selected grammar version;
 *   - canonical token stream.
 *
 * Parsing MUST NOT depend on:
 *
 *   - network state;
 *   - DNS;
 *   - filesystem state;
 *   - hardware;
 *   - runtime state;
 *   - environment variables;
 *   - wall-clock time;
 *   - randomness;
 *   - target availability.
 *
 * ============================================================================
 * SEMANTIC RESPONSIBILITIES
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *   - contextual marker validation;
 *   - protocol name resolution;
 *   - role resolution;
 *   - message resolution;
 *   - operation resolution;
 *   - type checking;
 *   - protocol refinement validation;
 *   - state/transition validation;
 *   - capability validation;
 *   - requirement checking;
 *   - constraint checking;
 *   - preference handling;
 *   - security checking;
 *   - distributed compatibility;
 *   - endpoint compatibility;
 *   - channel compatibility;
 *   - protocol compatibility;
 *   - resource feasibility.
 *
 * The parser does none of these.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces parse-tree contexts.
 *
 * Frontend AST lowering should map:
 *
 *   protocolDeclaration
 *       ->
 *   domain-neutral protocol declaration node
 *
 *   protocolRoleDeclaration
 *       ->
 *   protocol role node
 *
 *   protocolMessageReference
 *       ->
 *   protocol message-reference node
 *
 *   protocolOperationDeclaration
 *       ->
 *   protocol operation-contract node
 *
 *   protocolStateDeclaration
 *       ->
 *   protocol state node
 *
 *   protocolTransitionDeclaration
 *       ->
 *   protocol transition node
 *
 *   protocolRequirementDeclaration
 *       ->
 *   requirement node
 *
 *   protocolCapabilityDeclaration
 *       ->
 *   capability-reference node
 *
 *   protocolConstraintDeclaration
 *       ->
 *   constraint node
 *
 *   protocolPreferenceDeclaration
 *       ->
 *   preference node
 *
 *   protocolPropertyDeclaration
 *       ->
 *   property node
 *
 * No networking-specific backend object should be created merely because a
 * protocol was parsed.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Protocol semantics may later lower into:
 *
 *   - networking semantic representation;
 *   - distributed semantic representation;
 *   - classical IR;
 *   - hardware communication intent;
 *   - deployment representation;
 *   - quantum-related semantic metadata.
 *
 * If quantum computation is involved, quantum semantics continue through:
 *
 *   quantum::ir
 *
 * rather than a protocol-specific quantum IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may consume protocol semantics for:
 *
 *   - capability negotiation;
 *   - protocol compatibility;
 *   - communication planning;
 *   - routing;
 *   - placement;
 *   - scheduling;
 *   - serialization selection;
 *   - security obligations;
 *   - distributed execution planning;
 *   - target lowering.
 *
 * None of these operations occur in this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may eventually:
 *
 *   - resolve protocol implementations;
 *   - bind endpoints;
 *   - select channels;
 *   - negotiate capabilities;
 *   - establish communication;
 *   - monitor communication;
 *   - recover communication;
 *   - migrate communication;
 *   - select another valid realization.
 *
 * This grammar performs none of these operations.
 *
 * ============================================================================
 */

parser grammar Protocols;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions;


/*
 * ============================================================================
 * PUBLIC COMPOSITION ENTRY POINT
 * ============================================================================
 *
 * networking.g4 consumes:
 *
 *     protocolConstruct
 *
 * No other grammar should copy protocolDeclaration directly when
 * protocolConstruct is sufficient.
 * ============================================================================
 */

protocolConstruct
    : protocolDeclaration
    | protocolReference
    ;


/*
 * ============================================================================
 * CONTEXTUAL MARKER HELPERS
 * ============================================================================
 *
 * These rules intentionally consume canonical identifiers.
 *
 * Semantic analysis MUST validate their source spelling.
 *
 * This keeps protocol vocabulary open without adding a second lexer.
 * ============================================================================
 */

protocolMarker
    : identifier
    ;

roleMarker
    : identifier
    ;

messageMarker
    : identifier
    ;

operationMarker
    : identifier
    ;

usesMarker
    : identifier
    ;

providesMarker
    : identifier
    ;

stateMarker
    : identifier
    ;

transitionMarker
    : identifier
    ;

sendMarker
    : identifier
    ;

receiveMarker
    : identifier
    ;

emitMarker
    : identifier
    ;

onMarker
    : identifier
    ;

fromMarker
    : identifier
    ;

toMarker
    : identifier
    ;

flowMarker
    : identifier
    ;

sequenceMarker
    : identifier
    ;

choiceMarker
    : identifier
    ;

parallelMarker
    : identifier
    ;

repeatMarker
    : identifier
    ;

optionalMarker
    : identifier
    ;

waitMarker
    : identifier
    ;


/*
 * ============================================================================
 * PROTOCOL DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     protocol Reliable;
 *
 *     protocol Reliable {
 *         ...
 *     }
 *
 *     protocol Application<T> extends Base {
 *         ...
 *     }
 *
 * Generic parameters are declared locally as protocol contract parameters.
 *
 * Their types and bounds use the canonical type system.
 * ============================================================================
 */

protocolDeclaration
    : protocolMarker
      identifier
      protocolTypeParameterList?
      protocolInheritanceClause?
      protocolWhereClause?
      protocolBody?
      SEMI?
    ;


/*
 * ============================================================================
 * PROTOCOL TYPE PARAMETERS
 * ============================================================================
 *
 * This is declaration syntax, not a second type system.
 *
 * Example:
 *
 *     protocol Transport<T>;
 *
 *     protocol Transport<T: Message>;
 *
 *     protocol Transport<T: Serializable, M: Message>;
 *
 * The type expressions used as bounds are owned by Types.
 *
 * ============================================================================
 */

protocolTypeParameterList
    : LESS_THAN
      protocolTypeParameter
      (COMMA protocolTypeParameter)*
      COMMA?
      GREATER_THAN
    ;

protocolTypeParameter
    : identifier
      protocolTypeParameterBound?
    ;

protocolTypeParameterBound
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * PROTOCOL INHERITANCE / REFINEMENT
 * ============================================================================
 *
 * `extends` is a canonical lexical token.
 *
 * Refinement is semantic composition.
 *
 * It does not imply implementation inheritance.
 * ============================================================================
 */

protocolInheritanceClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * PROTOCOL WHERE CLAUSE
 * ============================================================================
 */

protocolWhereClause
    : WHERE
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL BODY
 * ============================================================================
 */

protocolBody
    : LBRACE
      protocolMember*
      RBRACE
    ;


/*
 * ============================================================================
 * PROTOCOL MEMBER DISPATCH
 * ============================================================================
 */

protocolMember
    : protocolRoleDeclaration
    | protocolMessageReference
    | protocolOperationDeclaration
    | protocolUseDeclaration
    | protocolProvideDeclaration
    | protocolRequirementDeclaration
    | protocolCapabilityDeclaration
    | protocolConstraintDeclaration
    | protocolPreferenceDeclaration
    | protocolPropertyDeclaration
    | protocolStateDeclaration
    | protocolTransitionDeclaration
    | protocolFlowDeclaration
    | protocolNestedDeclaration
    ;


/*
 * ============================================================================
 * ROLE
 * ============================================================================
 *
 * A role is a LOGICAL participant.
 *
 * It is not:
 *
 *   - a machine;
 *   - a node;
 *   - a process;
 *   - a thread;
 *   - a device;
 *   - an address;
 *   - an interface.
 * ============================================================================
 */

protocolRoleDeclaration
    : roleMarker
      identifier
      protocolRoleTypeClause?
      SEMI
    ;

protocolRoleTypeClause
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * MESSAGE REFERENCE
 * ============================================================================
 *
 * Message schemas remain owned by messages.g4.
 *
 * Example:
 *
 *     message telemetry::Measurement;
 *
 * Optional direction metadata is preserved as an expression/name.
 * ============================================================================
 */

protocolMessageReference
    : messageMarker
      qualifiedName
      protocolMessageDirectionClause?
      SEMI
    ;

protocolMessageDirectionClause
    : COLON
      expression
    ;


/*
 * ============================================================================
 * OPERATION CONTRACT
 * ============================================================================
 *
 * Example:
 *
 *     operation send<T>(
 *         value: T
 *     ) -> Result<T>
 *         requires networking::reliable
 *         provides networking::delivery
 *         uses channel::results;
 *
 * The operation describes a contract.
 *
 * It does not specify:
 *
 *   - socket;
 *   - port;
 *   - thread;
 *   - process;
 *   - node;
 *   - machine;
 *   - transport implementation.
 * ============================================================================
 */

protocolOperationDeclaration
    : operationMarker
      identifier
      protocolTypeParameterList?
      LPAREN
      protocolParameterList?
      RPAREN
      protocolReturnClause?
      protocolOperationClause*
      protocolWhereClause?
      SEMI
    ;


/*
 * ============================================================================
 * OPERATION PARAMETERS
 * ============================================================================
 */

protocolParameterList
    : protocolParameter
      (COMMA protocolParameter)*
      COMMA?
    ;

protocolParameter
    : identifier
      COLON
      typeExpression
      protocolParameterDefault?
    ;

protocolParameterDefault
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * OPERATION RETURN TYPE
 * ============================================================================
 */

protocolReturnClause
    : ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * OPERATION CLAUSES
 * ============================================================================
 */

protocolOperationClause
    : protocolRequiresClause
    | protocolProvidesClause
    | protocolUsesClause
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * `requires` is a canonical lexical token.
 *
 * A requirement is semantic intent, not allocation.
 * ============================================================================
 */

protocolRequiresClause
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * PROVIDES
 * ============================================================================
 *
 * `provides` is intentionally contextual because the inspected canonical
 * keyword registry does not currently define a PROVIDES token.
 * ============================================================================
 */

protocolProvidesClause
    : providesMarker
      expression
    ;


/*
 * ============================================================================
 * USES
 * ============================================================================
 */

protocolUsesClause
    : usesMarker
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL USE / COMPOSITION
 * ============================================================================
 */

protocolUseDeclaration
    : usesMarker
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL PROVIDED CONTRACT
 * ============================================================================
 */

protocolProvideDeclaration
    : providesMarker
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL REQUIREMENT DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     requires networking::reliable;
 *
 *     requires networking::latency < bound;
 *
 *     requires capability("network.reliable");
 *
 *     requires expression;
 * ============================================================================
 */

protocolRequirementDeclaration
    : REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL CAPABILITY DECLARATION
 * ============================================================================
 *
 * `capability` is a canonical lexical token.
 *
 * Capability discovery is downstream.
 * ============================================================================
 */

protocolCapabilityDeclaration
    : CAPABILITY
      qualifiedName
      protocolCapabilityValue?
      SEMI
    ;

protocolCapabilityValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL CONSTRAINT
 * ============================================================================
 *
 * A constraint limits legal realizations.
 *
 * It does not select a machine.
 * ============================================================================
 */

protocolConstraintDeclaration
    : CONSTRAINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL PREFERENCE
 * ============================================================================
 *
 * The canonical lexer currently exposes `PREFER`.
 *
 * The source form therefore uses:
 *
 *     prefer expression;
 *
 * rather than introducing a new PREFERENCE token.
 * ============================================================================
 */

protocolPreferenceDeclaration
    : PREFER
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL PROPERTY
 * ============================================================================
 *
 * `property` is a canonical lexical token.
 *
 * Property names are open-world identifiers.
 * ============================================================================
 */

protocolPropertyDeclaration
    : PROPERTY
      identifier
      protocolPropertyTypeClause?
      protocolPropertyInitializer?
      SEMI
    ;

protocolPropertyTypeClause
    : COLON
      typeExpression
    ;

protocolPropertyInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL STATES
 * ============================================================================
 *
 * States describe the abstract state machine of a protocol.
 *
 * They are logical protocol states, NOT runtime process states.
 *
 * Example:
 *
 *     state idle;
 *     state established;
 *     state closed;
 * ============================================================================
 */

protocolStateDeclaration
    : stateMarker
      identifier
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL TRANSITIONS
 * ============================================================================
 *
 * A transition describes a legal protocol-state change.
 *
 * Canonical conceptual forms:
 *
 *     transition idle -> established;
 *
 *     transition idle -> established on connect;
 *
 *     transition established -> closed
 *         on disconnect
 *         requires security::authenticated;
 *
 * Source spellings such as `from`, `to`, and `on` remain contextual.
 *
 * ============================================================================
 */

protocolTransitionDeclaration
    : transitionMarker
      identifier
      ARROW
      identifier
      protocolTransitionEventClause?
      protocolTransitionConditionClause*
      SEMI
    ;

protocolTransitionEventClause
    : onMarker
      expression
    ;

protocolTransitionConditionClause
    : REQUIRES
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL FLOW
 * ============================================================================
 *
 * A flow provides an explicit interaction contract without tying the protocol
 * to a transport implementation.
 *
 * Example:
 *
 *     flow request_response {
 *         send request;
 *         receive response;
 *     }
 *
 * The flow syntax is intentionally generic.
 *
 * ============================================================================
 */

protocolFlowDeclaration
    : flowMarker
      identifier?
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMI?
    ;


/*
 * ============================================================================
 * FLOW STEP
 * ============================================================================
 */

protocolFlowStep
    : protocolSendStep
    | protocolReceiveStep
    | protocolEmitStep
    | protocolFlowCallStep
    | protocolSequenceStep
    | protocolChoiceStep
    | protocolParallelStep
    | protocolRepeatStep
    | protocolOptionalStep
    | protocolWaitStep
    | protocolFlowExpressionStep
    ;


/*
 * ============================================================================
 * SEND
 * ============================================================================
 *
 * Send intent only.
 *
 * It does not open a socket or choose a transport.
 * ============================================================================
 */

protocolSendStep
    : sendMarker
      expression
      SEMI
    ;


/*
 * ============================================================================
 * RECEIVE
 * ============================================================================
 */

protocolReceiveStep
    : receiveMarker
      expression
      SEMI
    ;


/*
 * ============================================================================
 * EMIT
 * ============================================================================
 *
 * Logical event emission.
 * ============================================================================
 */

protocolEmitStep
    : emitMarker
      expression
      SEMI
    ;


/*
 * ============================================================================
 * FLOW CALL
 * ============================================================================
 *
 * A protocol flow may reference another named protocol operation or flow.
 * ============================================================================
 */

protocolFlowCallStep
    : qualifiedName
      LPAREN
      protocolFlowArgumentList?
      RPAREN
      SEMI
    ;

protocolFlowArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * SEQUENCE
 * ============================================================================
 */

protocolSequenceStep
    : sequenceMarker
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMI?
    ;


/*
 * ============================================================================
 * CHOICE
 * ============================================================================
 */

protocolChoiceStep
    : choiceMarker
      LBRACE
      protocolChoiceBranch+
      RBRACE
      SEMI?
    ;

protocolChoiceBranch
    : protocolFlowStep+
    ;


/*
 * ============================================================================
 * PARALLEL
 * ============================================================================
 *
 * Parallel protocol interactions are semantic intent.
 *
 * The runtime/compiler decides whether the realization is:
 *
 *   - concurrent;
 *   - pipelined;
 *   - multiplexed;
 *   - serialized;
 *   - distributed.
 * ============================================================================
 */

protocolParallelStep
    : parallelMarker
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMI?
    ;


/*
 * ============================================================================
 * REPEAT
 * ============================================================================
 *
 * The repeat expression is semantic.
 *
 * It may be:
 *
 *   - finite;
 *   - symbolic;
 *   - resource dependent;
 *   - condition controlled.
 *
 * The grammar does not impose a maximum iteration count.
 * ============================================================================
 */

protocolRepeatStep
    : repeatMarker
      protocolRepeatCondition?
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMI?
    ;

protocolRepeatCondition
    : expression
    ;


/*
 * ============================================================================
 * OPTIONAL
 * ============================================================================
 */

protocolOptionalStep
    : optionalMarker
      LBRACE
      protocolFlowStep*
      RBRACE
      SEMI?
    ;


/*
 * ============================================================================
 * WAIT
 * ============================================================================
 *
 * Wait describes logical protocol waiting.
 *
 * Timing interpretation is semantic/runtime behavior.
 * ============================================================================
 */

protocolWaitStep
    : waitMarker
      expression
      SEMI
    ;


/*
 * ============================================================================
 * GENERIC FLOW EXPRESSION
 * ============================================================================
 *
 * This provides forward compatibility for protocol constructs that can be
 * represented as ordinary Zamani expressions without introducing another
 * expression language.
 * ============================================================================
 */

protocolFlowExpressionStep
    : expression
      SEMI
    ;


/*
 * ============================================================================
 * NESTED PROTOCOL
 * ============================================================================
 *
 * Nested declarations are namespace/scoping syntax only.
 *
 * Semantic analysis determines whether a particular nesting is legal.
 * ============================================================================
 */

protocolNestedDeclaration
    : protocolDeclaration
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCE
 * ============================================================================
 *
 * A protocol reference is an ordinary qualified name.
 *
 * Resolution is semantic.
 * ============================================================================
 */

protocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REFERENCE LISTS
 * ============================================================================
 */

protocolReferenceList
    : protocolReference
      (COMMA protocolReference)*
      COMMA?
    ;

protocolRoleReference
    : qualifiedName
    ;

protocolRoleReferenceList
    : protocolRoleReference
      (COMMA protocolRoleReference)*
      COMMA?
    ;

protocolMessageReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;

protocolOperationReference
    : qualifiedName
    ;

protocolOperationReferenceList
    : protocolOperationReference
      (COMMA protocolOperationReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * REUSABLE EXPRESSION CONTRACTS
 * ============================================================================
 *
 * These are adapter rules, not new expression languages.
 * ============================================================================
 */

protocolValue
    : expression
    ;

protocolCondition
    : expression
    ;

protocolPredicate
    : expression
    ;


/*
 * ============================================================================
 * COMPLETION / CONFORMANCE CONTRACT
 * ============================================================================
 *
 * This grammar is complete when:
 *
 * [x] It has one canonical parser grammar identity: Protocols.
 *
 * [x] It consumes the canonical ZamaniLexer.
 *
 * [x] It imports canonical Names.
 *
 * [x] It imports canonical Types.
 *
 * [x] It imports canonical Expressions.
 *
 * [x] It does not define a lexer.
 *
 * [x] It does not define a second identifier system.
 *
 * [x] It does not define a second type system.
 *
 * [x] It does not define a second expression language.
 *
 * [x] It does not enumerate today's network protocols.
 *
 * [x] It does not encode physical addresses.
 *
 * [x] It does not encode sockets or ports.
 *
 * [x] It does not encode network topology.
 *
 * [x] It does not encode hardware topology.
 *
 * [x] It does not encode machine counts.
 *
 * [x] It does not encode node limits.
 *
 * [x] It does not encode bandwidth limits.
 *
 * [x] It does not encode latency limits.
 *
 * [x] It does not encode message-size limits.
 *
 * [x] It supports open-world protocol names.
 *
 * [x] It supports protocol refinement.
 *
 * [x] It supports protocol roles.
 *
 * [x] It references external message schemas.
 *
 * [x] It supports operation contracts.
 *
 * [x] It supports requirements.
 *
 * [x] It supports capabilities.
 *
 * [x] It supports constraints.
 *
 * [x] It supports preferences.
 *
 * [x] It supports properties.
 *
 * [x] It supports protocol state machines.
 *
 * [x] It supports protocol transitions.
 *
 * [x] It supports explicit interaction flows.
 *
 * [x] It supports sequence composition.
 *
 * [x] It supports choice composition.
 *
 * [x] It supports parallel composition.
 *
 * [x] It supports repetition.
 *
 * [x] It supports optional interactions.
 *
 * [x] It supports logical waiting.
 *
 * [x] It remains parser-only.
 *
 * [x] It creates no IR.
 *
 * [x] It creates no runtime object.
 *
 * [x] It performs no discovery.
 *
 * [x] It performs no routing.
 *
 * [x] It performs no scheduling.
 *
 * [x] It performs no hardware access.
 *
 * [x] It introduces no unsafe Rust.
 *
 * [x] It has no embedded Rust actions.
 *
 * [x] It has no semantic predicates.
 *
 * [x] It has no artificial scalability ceiling.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * ============================================================================
 * REQUIRED INTEGRATION
 * ============================================================================
 *
 * networking.g4
 *     |
 *     +--> protocolConstruct
 *
 * grammar/antlr/ZamaniParser.g4
 *     |
 *     +--> networkingElement
 *             |
 *             +--> networkingConstruct
 *                     |
 *                     +--> protocolConstruct
 *
 * Frontend AST:
 *
 *     protocolDeclaration
 *         -> protocol declaration AST
 *
 *     protocolRoleDeclaration
 *         -> protocol role AST
 *
 *     protocolMessageReference
 *         -> protocol message-reference AST
 *
 *     protocolOperationDeclaration
 *         -> protocol operation-contract AST
 *
 *     protocolStateDeclaration
 *         -> protocol state AST
 *
 *     protocolTransitionDeclaration
 *         -> protocol transition AST
 *
 *     protocolFlowDeclaration
 *         -> protocol flow AST
 *
 * Semantic analysis:
 *
 *     resolve names
 *     resolve messages
 *     resolve endpoints
 *     resolve channels
 *     resolve services
 *     validate protocol states
 *     validate transitions
 *     validate operation contracts
 *     validate capabilities
 *     validate requirements
 *     validate constraints
 *     validate preferences
 *     validate security obligations
 *     validate distributed compatibility
 *
 * Lowering:
 *
 *     protocol semantics
 *         ->
 *     canonical networking/distributed representation
 *         ->
 *     target-independent optimization
 *         ->
 *     routing / placement / scheduling
 *         ->
 *     runtime realization
 *
 * Quantum:
 *
 *     protocol semantics
 *         ->
 *     semantic quantum integration
 *         ->
 *     quantum::ir
 *
 * NOT:
 *
 *     protocol grammar
 *         ->
 *     protocol-specific quantum IR
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal constants are absent:
 *
 *   MAX_PROTOCOLS
 *   MAX_ROLES
 *   MAX_MESSAGES
 *   MAX_OPERATIONS
 *   MAX_STATES
 *   MAX_TRANSITIONS
 *   MAX_ENDPOINTS
 *   MAX_CHANNELS
 *   MAX_NODES
 *   MAX_DEVICES
 *   MAX_NETWORK_SIZE
 *   MAX_BANDWIDTH
 *   MAX_LATENCY
 *   MAX_MESSAGE_SIZE
 *
 * There are no:
 *
 *   CPU limits
 *   GPU limits
 *   FPGA limits
 *   QPU limits
 *   qubit limits
 *   memory limits
 *   register-width limits
 *   tensor-rank limits
 *   topology limits
 *
 * A numeric literal appearing in an expression remains PROGRAM SEMANTICS.
 *
 * For example:
 *
 *     requires networking::latency <= 100
 *
 * does not create a universal 100-unit language limit.
 *
 * ============================================================================
 * NEGATIVE CONFORMANCE CASES
 * ============================================================================
 *
 * These are expected to be rejected syntactically:
 *
 *     protocol;
 *
 *     protocol Network {
 *
 *     role;
 *
 *     message;
 *
 *     operation();
 *
 *     protocol Network {
 *         state;
 *     }
 *
 *     protocol Network {
 *         transition idle;
 *     }
 *
 *     protocol Network {
 *         operation send(value);
 *     }
 *
 *     protocol Network {
 *         requires;
 *     }
 *
 *     protocol Network {
 *         capability;
 *     }
 *
 *     protocol Network {
 *         constraint;
 *     }
 *
 * ============================================================================
 * POSITIVE CONFORMANCE CASES
 * ============================================================================
 *
 * Minimal protocol:
 *
 *     protocol Reliable;
 *
 * Protocol with roles:
 *
 *     protocol Reliable {
 *         role sender;
 *         role receiver;
 *     }
 *
 * Protocol with messages:
 *
 *     protocol Reliable {
 *         message transport::Request;
 *         message transport::Response;
 *     }
 *
 * Protocol with operation:
 *
 *     protocol Reliable {
 *         operation send(
 *             value: Message
 *         ) -> Result;
 *     }
 *
 * Protocol with refinement:
 *
 *     protocol Secure extends Reliable {
 *         requires security::authentication;
 *         capability networking::reliable_delivery;
 *         prefer networking::low_latency;
 *     }
 *
 * Protocol state machine:
 *
 *     protocol Session {
 *         state idle;
 *         state established;
 *         state closed;
 *
 *         transition idle -> established;
 *         transition established -> closed;
 *     }
 *
 * Protocol transition with event:
 *
 *     protocol Session {
 *         transition idle -> established on connect;
 *     }
 *
 * Protocol flow:
 *
 *     protocol RequestResponse {
 *         flow request_response {
 *             send request;
 *             receive response;
 *         }
 *     }
 *
 * Protocol composition:
 *
 *     protocol SecureRequest extends RequestResponse {
 *         uses security::authentication;
 *         provides security::confidentiality;
 *     }
 *
 * Protocol with parameterization:
 *
 *     protocol Transport<T: Message>;
 *
 * ============================================================================
 * SCALABILITY CONFORMANCE
 * ============================================================================
 *
 * Implementations MUST test protocols containing:
 *
 *   - many roles;
 *   - many messages;
 *   - many operations;
 *   - many states;
 *   - many transitions;
 *   - many requirements;
 *   - many capabilities;
 *   - many properties;
 *   - deeply qualified names;
 *   - deeply nested flow structures;
 *   - large expressions;
 *   - large type expressions;
 *   - arbitrarily many protocol declarations.
 *
 * No test may establish a language-level maximum.
 *
 * The practical upper bound is determined by implementation resources and
 * explicitly declared semantic requirements.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What is the source-level syntax for a logical communication protocol?"
 *
 * It does NOT answer:
 *
 *     "Which machine runs it?"
 *     "Which interface is used?"
 *     "Which socket is opened?"
 *     "Which route is selected?"
 *     "Which network provider is selected?"
 *     "How many physical nodes exist?"
 *     "How is communication scheduled?"
 *     "How is communication encrypted?"
 *     "How is QEC performed?"
 *     "How is ZQN evaluated?"
 *
 * Those decisions belong downstream.
 *
 * The resulting architecture preserves:
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
 * subject to actual program semantics, declared requirements, available
 * resources, and target capabilities, without artificial grammar-level
 * hardware ceilings.
 *
 * ============================================================================
 */