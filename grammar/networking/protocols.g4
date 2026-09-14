/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Networking Protocol Grammar
 * ============================================================================
 *
 * File:
 *     grammar/networking/protocols.g4
 *
 * Grammar:
 *     Protocols
 *
 * Purpose:
 *     Source-level declaration of logical communication protocols and their
 *     contracts.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * ANTLR:
 *     ANTLR4 parser grammar
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
 *     - No target-specific assumptions.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL SYNTAX of a logical communication
 * protocol.
 *
 * A protocol describes communication semantics and contracts.
 *
 * It does NOT describe a particular implementation.
 *
 * The same protocol may be realized through:
 *
 *     - in-process communication;
 *     - shared memory;
 *     - IPC;
 *     - local channels;
 *     - distributed channels;
 *     - network transports;
 *     - accelerator fabrics;
 *     - hardware communication interfaces;
 *     - quantum communication infrastructure;
 *     - future communication substrates.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - protocol declarations;
 *     - protocol names;
 *     - protocol generic parameters;
 *     - protocol inheritance/refinement;
 *     - protocol composition;
 *     - protocol roles;
 *     - protocol message references;
 *     - protocol operation contracts;
 *     - protocol properties;
 *     - protocol requirements;
 *     - protocol capability references;
 *     - protocol constraints;
 *     - protocol preferences;
 *     - protocol metadata boundaries;
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - lexical literals;
 *     - types;
 *     - general expressions;
 *     - endpoint declarations;
 *     - channel declarations;
 *     - message schemas;
 *     - service declarations;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - resource discovery;
 *     - hardware discovery;
 *     - serialization;
 *     - compression;
 *     - encryption;
 *     - authentication;
 *     - authorization;
 *     - transport implementation;
 *     - sockets;
 *     - ports;
 *     - IP/MAC addresses;
 *     - network topology;
 *     - physical devices;
 *     - cluster topology;
 *     - runtime queues;
 *     - retry implementation;
 *     - distributed consensus;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - hardware calibration;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Protocol syntax describes WHAT communication contract exists.
 *
 * It does not permanently determine WHERE or HOW the contract is realized.
 *
 * Therefore a protocol declaration MUST NOT require:
 *
 *     a particular machine;
 *     a particular device;
 *     a particular node;
 *     a particular network;
 *     a particular address;
 *     a particular port;
 *     a particular transport;
 *     a particular number of participants;
 *     a particular bandwidth;
 *     a particular latency;
 *     a particular topology;
 *     a particular provider.
 *
 * Such properties belong to target/resource/deployment/runtime layers.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Protocol names are not enumerated.
 *
 * This grammar deliberately does NOT define closed protocol sets such as:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *
 * Those may be represented as qualified names, dialect declarations, or
 * target-specific capabilities elsewhere.
 *
 * A future protocol therefore does not require this grammar to change merely
 * because its name or implementation is new.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar constants for:
 *
 *     MAX_PROTOCOLS
 *     MAX_ROLES
 *     MAX_MESSAGES
 *     MAX_OPERATIONS
 *     MAX_FIELDS
 *     MAX_PARAMETERS
 *     MAX_REQUIREMENTS
 *     MAX_CAPABILITIES
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_PARTICIPANTS
 *     MAX_PROTOCOL_DEPTH
 *
 * All collections use unbounded ANTLR repetition.
 *
 * Physical/resource limits are intentionally downstream.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parser:
 *
 *     source
 *       -> lexer
 *       -> Protocols
 *       -> frontend AST
 *
 * Semantic analysis:
 *
 *       -> name resolution
 *       -> type checking
 *       -> protocol validation
 *       -> capability checking
 *       -> effect checking
 *       -> resource analysis
 *       -> security analysis
 *
 * Lowering:
 *
 *       -> canonical semantic representation
 *       -> distributed/networking representation
 *       -> classical IR where appropriate
 *       -> quantum::ir where quantum computation is involved
 *
 * Then:
 *
 *       -> optimization
 *       -> routing
 *       -> scheduling
 *       -> placement
 *       -> resilience
 *       -> hardware/runtime realization
 *
 * THIS GRAMMAR NEVER CREATES IR.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A protocol may carry or coordinate quantum-related values through ordinary
 * canonical types and expressions.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumState
 *     QuantumTopology
 *     Calibration
 *     QEC codes
 *     ZQN fault models
 *
 * If protocol semantics interact with quantum computation, semantic lowering
 * integrates with the canonical `quantum::ir` boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Protocols may describe logical interfaces used by hardware/software
 * co-design.
 *
 * This grammar does not define:
 *
 *     pins;
 *     FPGA routing;
 *     ASIC wiring;
 *     physical buses;
 *     fixed bus widths;
 *     fixed device counts;
 *     physical clock topology.
 *
 * HDL and hardware grammars own those concerns.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Protocols may express logical requirements such as:
 *
 *     requires security::confidentiality;
 *     requires security::authentication;
 *
 * through canonical qualified names.
 *
 * This grammar does not define cryptographic algorithms or security policy
 * semantics.
 *
 * Security remains owned by the security/effects/security grammar and
 * corresponding semantic subsystem.
 *
 * ============================================================================
 * MESSAGE INTEGRATION
 * ============================================================================
 *
 * Message schemas are owned by:
 *
 *     grammar/networking/messages.g4
 *
 * and, during migration, existing distributed messaging grammar.
 *
 * This file only references message declarations.
 *
 * It MUST NOT create a second message-schema language.
 *
 * ============================================================================
 * ENDPOINT INTEGRATION
 * ============================================================================
 *
 * Endpoint declarations are owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * Protocols may reference endpoint roles/types symbolically.
 *
 * This grammar does not define physical endpoint addressing.
 *
 * ============================================================================
 * CHANNEL INTEGRATION
 * ============================================================================
 *
 * Channels are owned by:
 *
 *     grammar/networking/channels.g4
 *
 * A protocol may constrain or describe channel semantics but does not create
 * channel runtime objects.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser consumes the canonical ZamaniLexer.
 *
 * Structural protocol keywords are:
 *
 *     protocol
 *     extends
 *     uses
 *     role
 *     message
 *     operation
 *     requires
 *     provides
 *     property
 *     constraint
 *     preference
 *     capability
 *
 * The lexer MUST expose these as canonical keyword tokens or compatible
 * literal tokens.
 *
 * Protocol names, role names, message names, operation names, capability names,
 * and future extension names remain canonical identifiers.
 *
 * No transport-specific lexer vocabulary is required here.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Core:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
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
 *
 * Attributes are imported from the canonical attribute grammar rather than
 * redefined.
 *
 * ============================================================================
 */

parser grammar Protocols;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable parser-composition boundary.
 *
 * Higher-level networking grammars should consume `protocolConstruct` rather
 * than duplicating protocol rules.
 */
protocolConstruct
    : protocolDeclaration
    | protocolReference
    ;


/*
 * ============================================================================
 * PROTOCOL DECLARATION
 * ============================================================================
 *
 * Canonical examples:
 *
 *     protocol ReliableTransport;
 *
 *     protocol ReliableTransport {
 *         role sender;
 *         role receiver;
 *     }
 *
 *     protocol ApplicationProtocol<T> extends BaseProtocol {
 *         ...
 *     }
 *
 * The declaration describes a logical contract, not an implementation.
 */
protocolDeclaration
    : attribute*
      visibility?
      PROTOCOL
      identifier
      genericParameters?
      protocolInheritanceClause?
      protocolWhereClause?
      protocolBody?
      SEMI?
    ;


/*
 * ============================================================================
 * PROTOCOL INHERITANCE / REFINEMENT
 * ============================================================================
 *
 * Protocol refinement is semantic composition.
 *
 * It does not imply implementation inheritance, class inheritance, or runtime
 * dispatch.
 */
protocolInheritanceClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * PROTOCOL CONSTRAINTS
 * ============================================================================
 *
 * Generic protocol constraints remain associated with the canonical type/core
 * system.
 *
 * The rule is intentionally a delegation boundary.
 */
protocolWhereClause
    : WHERE
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL BODY
 * ============================================================================
 *
 * The body contains zero or more protocol members.
 *
 * No fixed member count is encoded.
 */
protocolBody
    : LBRACE
      protocolMember*
      RBRACE
    ;


/*
 * ============================================================================
 * PROTOCOL MEMBER
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
    | protocolNestedDeclaration
    ;


/*
 * ============================================================================
 * PROTOCOL ROLE
 * ============================================================================
 *
 * A role is a logical participant in the protocol.
 *
 * It does NOT identify:
 *
 *     a machine;
 *     a process;
 *     a node;
 *     a device;
 *     an address;
 *     a network interface.
 */
protocolRoleDeclaration
    : attribute*
      ROLE
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
 * This references a message schema owned elsewhere.
 *
 * It does not define the message fields here.
 */
protocolMessageReference
    : attribute*
      MESSAGE
      qualifiedName
      protocolMessageDirectionClause?
      SEMI
    ;

protocolMessageDirectionClause
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * PROTOCOL OPERATIONS
 * ============================================================================
 *
 * Operations describe logical communication actions/contracts.
 *
 * They do NOT prescribe:
 *
 *     transport;
 *     scheduling;
 *     retries;
 *     sockets;
 *     threads;
 *     machines;
 *     devices.
 */
protocolOperationDeclaration
    : attribute*
      OPERATION
      identifier
      genericParameters?
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
 *
 * Parameters use canonical types.
 *
 * There is no fixed parameter count.
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
 * OPERATION RETURN
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
 *
 * These are semantic references, not implementations.
 */
protocolOperationClause
    : protocolRequiresClause
    | protocolProvidesClause
    | protocolUsesClause
    ;

protocolRequiresClause
    : REQUIRES
      qualifiedName
    ;

protocolProvidesClause
    : PROVIDES
      qualifiedName
    ;

protocolUsesClause
    : USES
      qualifiedName
    ;


/*
 * ============================================================================
 * PROTOCOL COMPOSITION
 * ============================================================================
 *
 * `uses` expresses logical dependence/composition.
 *
 * It does not force a particular implementation library.
 */
protocolUseDeclaration
    : attribute*
      USES
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL PROVIDED CONTRACT
 * ============================================================================
 */
protocolProvideDeclaration
    : attribute*
      PROVIDES
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL REQUIREMENTS
 * ============================================================================
 *
 * A requirement describes a semantic need.
 *
 * It is not a hardware/resource allocation.
 */
protocolRequirementDeclaration
    : attribute*
      REQUIRES
      qualifiedName
      protocolRequirementValue?
      SEMI
    ;

protocolRequirementValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * PROTOCOL CAPABILITIES
 * ============================================================================
 *
 * Capability references describe capabilities required/provided by a protocol.
 *
 * Capability discovery remains downstream.
 */
protocolCapabilityDeclaration
    : attribute*
      CAPABILITY
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
 * PROTOCOL CONSTRAINTS
 * ============================================================================
 *
 * Constraints express conditions on valid realization.
 *
 * They do not become machine constants.
 */
protocolConstraintDeclaration
    : attribute*
      CONSTRAINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * A preference MUST NOT be interpreted as a mandatory machine-selection rule.
 */
protocolPreferenceDeclaration
    : attribute*
      PREFERENCE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL PROPERTIES
 * ============================================================================
 *
 * Generic properties provide extensibility without adding transport-specific
 * grammar rules.
 *
 * Example:
 *
 *     property ordering: Ordering;
 *     property semantics: Ordering::Total;
 *     property mode = expression;
 *
 * The semantic layer determines the property's meaning.
 */
protocolPropertyDeclaration
    : attribute*
      PROPERTY
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
 * NESTED PROTOCOL DECLARATIONS
 * ============================================================================
 *
 * Nested protocols are allowed for namespace/scoping purposes.
 *
 * Their semantic legality is checked downstream.
 */
protocolNestedDeclaration
    : protocolDeclaration
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCE
 * ============================================================================
 *
 * A reference is simply a canonical qualified name.
 *
 * It does not resolve the referenced protocol.
 */
protocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCE LIST
 * ============================================================================
 *
 * Unbounded protocol references.
 */
protocolReferenceList
    : protocolReference
      (COMMA protocolReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL PROTOCOL REFERENCE LIST
 * ============================================================================
 */
optionalProtocolReferenceList
    : protocolReferenceList?
    ;


/*
 * ============================================================================
 * PROTOCOL ROLE REFERENCE
 * ============================================================================
 */
protocolRoleReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * PROTOCOL ROLE REFERENCE LIST
 * ============================================================================
 */
protocolRoleReferenceList
    : protocolRoleReference
      (COMMA protocolRoleReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * PROTOCOL MESSAGE REFERENCE LIST
 * ============================================================================
 */
protocolMessageReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;


/*
 * ============================================================================
 * PROTOCOL OPERATION REFERENCE
 * ============================================================================
 */
protocolOperationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * PROTOCOL OPERATION REFERENCE LIST
 * ============================================================================
 */
protocolOperationReferenceList
    : protocolOperationReference
      (COMMA protocolOperationReference)*
      COMMA?
    ;