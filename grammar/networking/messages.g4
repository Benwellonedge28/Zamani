/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/networking/messages.g4
 *
 * Grammar:
 *     Messages
 *
 * Status:
 *     CANONICAL PRODUCTION NETWORKING MESSAGE GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the SINGLE CANONICAL SOURCE-LEVEL OWNER of logical message
 * schema syntax in Zamani.
 *
 * A message is a typed logical value/schema that may participate in:
 *
 *     - local computation;
 *     - functions;
 *     - tasks;
 *     - actors;
 *     - concurrency;
 *     - distributed computation;
 *     - networking;
 *     - IPC;
 *     - shared-memory communication;
 *     - accelerator communication;
 *     - hardware communication;
 *     - classical/quantum hybrid computation;
 *     - AI/data pipelines;
 *     - future computational substrates.
 *
 * This grammar describes the logical message.
 *
 * It does NOT define how the message is:
 *
 *     - allocated;
 *     - copied;
 *     - moved;
 *     - serialized;
 *     - compressed;
 *     - encrypted;
 *     - authenticated;
 *     - routed;
 *     - scheduled;
 *     - replicated;
 *     - transmitted;
 *     - stored;
 *     - placed;
 *     - executed.
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     messageConstruct
 *     messageDeclaration
 *     messageGenericParameters
 *     messageGenericParameterList
 *     messageGenericParameter
 *     messageGenericParameterConstraint
 *     messageMember
 *     messageField
 *     messageFieldInitializer
 *     messageValue
 *     messageTypeReference
 *     messageArgumentList
 *     optionalMessageArgumentList
 *     messagePayload
 *     messagePayloadList
 *     optionalMessagePayloadList
 *     messageFieldReference
 *     messageTypeReferenceList
 *     optionalMessageTypeReferenceList
 *     messageFieldReferenceList
 *     optionalMessageFieldReferenceList
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexical definitions
 *     identifiers
 *     qualified names
 *     types
 *     generic type-system semantics
 *     expressions
 *     functions
 *     modules
 *     concurrency
 *     actors
 *     channels
 *     endpoints
 *     sockets
 *     protocols
 *     routing
 *     topology
 *     service discovery
 *     serialization
 *     compression
 *     encryption
 *     authentication
 *     authorization
 *     identity
 *     replication
 *     consistency
 *     fault tolerance
 *     scheduling
 *     placement
 *     resource allocation
 *     capability discovery
 *     hardware selection
 *     quantum operations
 *     quantum topology
 *     QEC
 *     HDL implementation
 *     classical IR
 *     quantum::ir
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * Logical message schema syntax has exactly ONE active canonical owner:
 *
 *     grammar/networking/messages.g4
 *
 * Distributed integration is provided by:
 *
 *     grammar/distributed/messages.g4
 *
 * The distributed adapter MUST delegate to this grammar.
 *
 * The historical:
 *
 *     grammar/distributed/messaging.g4
 *
 * MUST NOT remain an independent implementation of message schema syntax.
 *
 * It is outside the active distributed composition root and should be retired
 * or converted into a compatibility/delegation layer.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/types/types.g4
 *     grammar/expressions/expressions.g4
 *     grammar/core/attributes.g4
 *
 * IMPORTS:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * EXPORTS:
 *
 *     messageConstruct
 *     messageDeclaration
 *     messageValue
 *     messageTypeReference
 *     messageArgumentList
 *     optionalMessageArgumentList
 *     messagePayload
 *     messagePayloadList
 *     optionalMessagePayloadList
 *     messageFieldReference
 *     messageTypeReferenceList
 *     optionalMessageTypeReferenceList
 *     messageFieldReferenceList
 *     optionalMessageFieldReferenceList
 *
 * CONSUMED_BY:
 *
 *     grammar/networking/networking.g4
 *     grammar/distributed/messages.g4
 *     networking semantic analysis
 *     distributed semantic analysis
 *     message/schema AST construction
 *     communication semantic analysis
 *     channel semantic analysis
 *
 * AST_OWNER:
 *
 *     frontend AST subsystem
 *
 * SEMANTIC_OWNER:
 *
 *     networking/distributed semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR subsystems
 *
 * TEST_OWNER:
 *
 *     grammar/tests/networking/
 *     grammar/tests/distributed/
 *     shared frontend conformance tests
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/networking.md
 *     grammar/spec/distributed.md
 *     applicable language specifications
 *
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     ANTLR parser
 *          |
 *          v
 *     Messages
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> schema validation
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          |
 *          v
 *     semantic message model
 *          |
 *          +--> classical semantics
 *          +--> distributed semantics
 *          +--> networking semantics
 *          +--> data/schema semantics
 *          +--> hybrid semantics
 *          +--> quantum::ir when quantum semantics actually participate
 *          +--> HDL/hardware semantics where applicable
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     runtime / ZQN / HAL
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A message describes logical information.
 *
 * It MUST NOT encode physical realization.
 *
 * This grammar therefore contains no universal limits for:
 *
 *     machines
 *     nodes
 *     processes
 *     actors
 *     tasks
 *     threads
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     memory
 *     storage
 *     network size
 *     bandwidth
 *     topology
 *     connections
 *     participants
 *
 * There are deliberately no source-level capacity constants.
 *
 * The same message declaration can therefore participate in deployments
 * ranging from a very small execution environment to very large available
 * computational infrastructure, subject only to actual implementation,
 * semantic, capability and resource feasibility.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * All source-level collections are open-ended.
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     ?
 *
 * where appropriate.
 *
 * No grammar rule establishes a universal maximum for:
 *
 *     message declarations
 *     fields
 *     arguments
 *     payloads
 *     message types
 *     qualified-name depth
 *     generic parameters
 *
 * Practical limits imposed by:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler configuration;
 *     - operating system;
 *     - runtime;
 *     - deployment;
 *     - target hardware
 *
 * are implementation/resource constraints rather than language-level limits.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate communication operations.
 *
 * The following remain ordinary names or constructs owned elsewhere:
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
 * Likewise, this grammar does not enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     vendor transports
 *
 * New transports, protocols, serialization formats, communication operations,
 * vendors and deployment models must not require changes to this grammar merely
 * because they introduce new implementations.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser consumes the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The structural declaration keyword is:
 *
 *     MESSAGE
 *
 * supplied by the canonical lexical layer.
 *
 * No lexical rules are declared here.
 *
 * Message names, field names, schema names, payload names, protocol names and
 * extension names remain canonical identifiers/qualified names.
 *
 *
 * ============================================================================
 * IMPORTANT LEXER TOKEN ALIGNMENT
 * ============================================================================
 *
 * The canonical lexer defines:
 *
 *     LESS       : '<'
 *     GREATER    : '>'
 *     SEMICOLON  : ';'
 *
 * Therefore this grammar MUST use those exact token names.
 *
 * It MUST NOT use legacy aliases such as:
 *
 *     LT
 *     GT
 *     SEMI
 *
 * This keeps parser grammar and lexical authority synchronized.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Message fields consume:
 *
 *     typeExpression
 *
 * from the canonical type subsystem.
 *
 * This grammar does NOT define another type language.
 *
 * Consequently message fields may use any type supported by the canonical
 * Zamani type system, including future types, generic types, classical types,
 * quantum-related types, hardware-related types, tensor/data types, resource
 * types, capability types and other registered type families.
 *
 * Whether a particular type is legal as a message field is a semantic
 * question, not a parser question.
 *
 *
 * ============================================================================
 * GENERIC MESSAGE CONTRACT
 * ============================================================================
 *
 * Generic message declarations provide source-level parameterization.
 *
 * Example:
 *
 *     message Envelope<T> {
 *         payload: T;
 *     }
 *
 * Generic parameters are syntax only.
 *
 * Generic substitution, bounds, unification, type checking and instantiation
 * belong to the canonical type semantic system.
 *
 * This grammar does not introduce machine-resource parameters.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Message initializers, construction arguments and payloads consume the
 * canonical:
 *
 *     expression
 *
 * grammar.
 *
 * This file does not redefine:
 *
 *     arithmetic
 *     calls
 *     indexing
 *     operators
 *     conditionals
 *     lambdas
 *     closures
 *     patterns
 *     reasoning
 *     learning
 *     uncertainty
 *     metaprogramming
 *     quantum expressions
 *
 * Those remain owned by their canonical expression/domain grammars.
 *
 *
 * ============================================================================
 * ATTRIBUTE CONTRACT
 * ============================================================================
 *
 * Message declarations and fields may consume canonical attributes.
 *
 * Attribute syntax is owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Attribute meaning is semantic.
 *
 * This allows message schemas to participate in:
 *
 *     contracts
 *     policies
 *     capabilities
 *     resources
 *     provenance
 *     security
 *     interoperability
 *     dialect metadata
 *     compatibility
 *
 * without creating message-specific copies of those systems.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Message declaration syntax has no inherent runtime effect.
 *
 * Constructing a message value does not automatically imply:
 *
 *     network I/O
 *     distributed execution
 *     serialization
 *     mutation
 *     foreign execution
 *     native execution
 *
 * A separate communication operation may introduce those effects.
 *
 * Effect classification belongs to semantic analysis.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Message syntax itself does not select physical capabilities.
 *
 * A later communication operation may require capabilities such as:
 *
 *     network communication
 *     distributed communication
 *     shared memory
 *     accelerator communication
 *     quantum/classical communication
 *
 * Capability requirements are semantic data.
 *
 * This grammar does not discover, enumerate or select capabilities.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Message syntax does not impose physical resource requirements.
 *
 * Resource requirements, when applicable, are represented through canonical
 * resource/requirement mechanisms and interpreted after parsing.
 *
 * This file must not introduce:
 *
 *     payload-size ceilings
 *     field-count ceilings
 *     participant-count ceilings
 *     network-size ceilings
 *     memory ceilings
 *     device ceilings
 *
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Message declarations may participate in generic:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     policy
 *
 * semantics through the canonical language validation/policy systems.
 *
 * This grammar does not create a message-specific contract or policy language.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source spans and declaration structure must remain available to the frontend
 * so semantic analysis can attach provenance to:
 *
 *     schema definitions
 *     field definitions
 *     defaults
 *     message constructions
 *     transformations
 *     generated representations
 *     compatibility migrations
 *     serialization/lowering decisions
 *
 * The grammar itself does not create provenance records.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A message may contain a canonical quantum-related value or type.
 *
 * This grammar does NOT define:
 *
 *     qubits
 *     quantum operations
 *     gates
 *     measurement operations
 *     circuit topology
 *     calibration
 *     QEC
 *     QPU placement
 *
 * If a message participates in quantum computation, downstream semantic
 * analysis determines whether and how the corresponding operation/data enters:
 *
 *     quantum::ir
 *
 * No physical quantum capacity is encoded here.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A message may represent logical data crossing a hardware/software boundary.
 *
 * This grammar does not define:
 *
 *     wires
 *     ports
 *     buses
 *     physical addresses
 *     register widths
 *     device identifiers
 *     clock domains
 *     physical topology
 *
 * Those remain owned by HDL/hardware subsystems.
 *
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * A message is not:
 *
 *     a packet
 *     an endpoint
 *     a socket
 *     a route
 *     a transport
 *     a network address
 *     a network topology
 *
 * Those concerns belong to other networking grammars.
 *
 * A message can be referenced by:
 *
 *     channels
 *     requests
 *     responses
 *     services
 *     streams
 *     protocols
 *
 * without acquiring those constructs' physical semantics.
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Distributed messaging consumes this grammar through:
 *
 *     grammar/distributed/messages.g4
 *
 * That adapter owns distributed-specific names such as:
 *
 *     distributedMessageConstruct
 *     distributedMessageDeclaration
 *     distributedMessageValue
 *
 * It MUST delegate to the canonical message rules rather than redefine them.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at least:
 *
 * MessageDeclaration:
 *
 *     attributes
 *     qualified name
 *     generic parameters
 *     ordered members
 *     source span
 *
 * MessageField:
 *
 *     attributes
 *     name
 *     type syntax
 *     optional initializer
 *     source span
 *
 * MessageValue:
 *
 *     message type reference
 *     ordered arguments
 *     source span
 *
 * Message references:
 *
 *     qualified name
 *     source span
 *
 * The exact Rust AST type names belong to the frontend AST subsystem.
 *
 * This grammar MUST NOT depend on Rust implementation details.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving message names;
 *     - resolving field names;
 *     - resolving types;
 *     - validating generic parameters;
 *     - validating generic constraints;
 *     - detecting duplicate fields;
 *     - checking default expressions;
 *     - checking argument correspondence;
 *     - checking argument types;
 *     - checking recursive schemas;
 *     - checking visibility;
 *     - checking ownership;
 *     - checking serialization legality;
 *     - checking transfer/move semantics;
 *     - checking capabilities;
 *     - checking resources;
 *     - checking effects;
 *     - checking contracts;
 *     - checking policies;
 *     - preserving provenance;
 *     - checking interoperability constraints.
 *
 * None of these checks are parser actions.
 *
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should cover structural failures such as:
 *
 *     missing message name
 *     missing field name
 *     missing field type
 *     missing colon
 *     missing semicolon
 *     malformed generic parameter list
 *     malformed message argument list
 *     malformed delimiters
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unknown message
 *     duplicate field
 *     invalid field type
 *     invalid generic substitution
 *     invalid default
 *     incompatible argument
 *     unsupported capability
 *     unsatisfied resource requirement
 *     invalid policy
 *     invalid serialization
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime callbacks;
 *     - no randomness.
 *
 * Parsing depends only on:
 *
 *     token stream
 *     grammar version
 *     imported grammar versions
 *     language compatibility configuration
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical message declaration remains:
 *
 *     message Name {
 *         field: Type;
 *     }
 *
 * Generic declarations are supported:
 *
 *     message Envelope<T> {
 *         payload: T;
 *     }
 *
 * Message construction remains:
 *
 *     Name(value)
 *
 * and:
 *
 *     namespace::Name(value)
 *
 * A trailing declaration semicolon is accepted:
 *
 *     message Name {
 *         field: Type;
 *     };
 *
 * This is intentionally compatible with other Zamani declaration grammars
 * that permit an optional terminator after a braced declaration.
 *
 *
 * ============================================================================
 * IMPLEMENTATION SAFETY
 * ============================================================================
 *
 * This grammar contains no embedded implementation code.
 *
 * The Rust frontend/parser integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * The grammar itself performs no resource discovery, allocation or execution.
 *
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * messageConstruct
 *     Standalone message composition boundary.
 *
 * messageDeclaration
 *     Canonical logical message schema declaration.
 *
 * messageValue
 *     Canonical message construction boundary.
 *
 * messageTypeReference
 *     Canonical message type/name reference.
 *
 * messageArgumentList
 *     Canonical message argument boundary.
 *
 * messagePayload
 *     Generic expression payload boundary.
 *
 * messageFieldReference
 *     Canonical message-field reference boundary.
 *
 * messageTypeReferenceList
 *     Open-ended message-type list.
 *
 * messageFieldReferenceList
 *     Open-ended message-field list.
 *
 *
 * ============================================================================
 * PRIVATE / SUPPORT RULES
 * ============================================================================
 *
 * The following rules support the public interface:
 *
 *     messageGenericParameters
 *     messageGenericParameterList
 *     messageGenericParameter
 *     messageGenericParameterConstraint
 *     messageMember
 *     messageField
 *     messageFieldInitializer
 *     optionalMessageArgumentList
 *     messagePayloadList
 *     optionalMessagePayloadList
 *     optionalMessageTypeReferenceList
 *     optionalMessageFieldReferenceList
 *
 *
 * ============================================================================
 * GRAMMAR IMPLEMENTATION
 * ============================================================================
 */

parser grammar Messages;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC MESSAGE COMPOSITION
 * ============================================================================
 *
 * This boundary is intentionally useful to standalone message grammar tests
 * and compatibility tooling.
 *
 * IMPORTANT:
 *
 * Networking's top-level declaration boundary should normally consume
 * `messageDeclaration`, not this mixed declaration/value boundary.
 *
 * Message values remain available to distributed/communication/expression
 * consumers through `messageValue`.
 */
messageConstruct
    : messageDeclaration
    | messageValue
    ;


/*
 * ============================================================================
 * MESSAGE DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 *     message Envelope<T> {
 *         payload: T;
 *     };
 */
messageDeclaration
    : attribute*
      MESSAGE
      qualifiedName
      messageGenericParameters?
      LBRACE
      messageMember*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * GENERIC MESSAGE PARAMETERS
 * ============================================================================
 *
 * Example:
 *
 *     message Envelope<T> {
 *         payload: T;
 *     }
 *
 * The syntax is intentionally small and delegates semantic interpretation to
 * the canonical type system.
 */
messageGenericParameters
    : LESS
      messageGenericParameterList
      GREATER
    ;


messageGenericParameterList
    : messageGenericParameter
      (
          COMMA
          messageGenericParameter
      )*
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


/*
 * ============================================================================
 * MESSAGE MEMBERS
 * ============================================================================
 *
 * Only schema fields are accepted.
 *
 * Executable statements do not belong inside a message schema.
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
 * Optional initializer:
 *
 *     fieldName: FieldType = expression;
 *
 * Optionality and collection semantics belong to the canonical type system.
 */
messageField
    : identifier
      COLON
      typeExpression
      messageFieldInitializer?
      SEMICOLON
    ;


messageFieldInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * MESSAGE VALUE
 * ============================================================================
 *
 * Canonical forms:
 *
 *     UserCreated(id, name)
 *
 *     events::UserCreated(id, name)
 *
 * This syntax intentionally resembles ordinary call syntax.
 *
 * Semantic analysis MUST determine whether the referenced name denotes a
 * message type.
 *
 * This grammar therefore does not pretend that syntax alone proves the
 * semantic category of the qualified name.
 */
messageValue
    : messageTypeReference
      LPAREN
      optionalMessageArgumentList
      RPAREN
    ;


/*
 * ============================================================================
 * MESSAGE TYPE REFERENCE
 * ============================================================================
 *
 * Message type names use the canonical qualified-name grammar.
 */
messageTypeReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * MESSAGE ARGUMENT LIST
 * ============================================================================
 *
 * The canonical expression grammar already owns expression-list syntax.
 *
 * This rule exists as the stable message-specific semantic boundary while
 * delegating the actual list structure to `expressionList`.
 */
messageArgumentList
    : expressionList
    ;


optionalMessageArgumentList
    : messageArgumentList?
    ;


/*
 * ============================================================================
 * MESSAGE PAYLOAD
 * ============================================================================
 *
 * A payload is a canonical expression.
 *
 * It does not imply:
 *
 *     serialization
 *     transmission
 *     allocation
 *     copying
 *     ownership transfer
 *     network I/O
 */
messagePayload
    : expression
    ;


messagePayloadList
    : messagePayload
      (
          COMMA
          messagePayload
      )*
      COMMA?
    ;


optionalMessagePayloadList
    : messagePayloadList?
    ;


/*
 * ============================================================================
 * MESSAGE FIELD REFERENCE
 * ============================================================================
 *
 * Semantic analysis determines whether the qualified name resolves to a
 * message field in the relevant scope.
 */
messageFieldReference
    : qualifiedName
    ;


messageFieldReferenceList
    : messageFieldReference
      (
          COMMA
          messageFieldReference
      )*
      COMMA?
    ;


optionalMessageFieldReferenceList
    : messageFieldReferenceList?
    ;


/*
 * ============================================================================
 * MESSAGE TYPE REFERENCE LIST
 * ============================================================================
 *
 * No finite cardinality is encoded.
 */
messageTypeReferenceList
    : messageTypeReference
      (
          COMMA
          messageTypeReference
      )*
      COMMA?
    ;


optionalMessageTypeReferenceList
    : messageTypeReferenceList?
    ;


/*
 * ============================================================================
 * END OF GRAMMAR
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * -------------------
 *
 * [x] Single canonical networking message-schema owner.
 *
 * [x] Canonical ZamaniLexer consumed.
 *
 * [x] Canonical Names grammar consumed.
 *
 * [x] Canonical Types grammar consumed.
 *
 * [x] Canonical Expressions grammar consumed.
 *
 * [x] Canonical Attributes grammar consumed.
 *
 * [x] No duplicated lexical rules.
 *
 * [x] No duplicated type system.
 *
 * [x] No duplicated expression system.
 *
 * [x] No communication operation syntax.
 *
 * [x] No transport syntax.
 *
 * [x] No serialization syntax.
 *
 * [x] No routing syntax.
 *
 * [x] No topology syntax.
 *
 * [x] No endpoint syntax.
 *
 * [x] No socket syntax.
 *
 * [x] No actor syntax.
 *
 * [x] No scheduler syntax.
 *
 * [x] No hardware selection.
 *
 * [x] No quantum operation selection.
 *
 * [x] No physical resource ceilings.
 *
 * [x] No parser actions.
 *
 * [x] No semantic predicates.
 *
 * [x] No runtime behavior.
 *
 * [x] Open-ended declarations.
 *
 * [x] Open-ended fields.
 *
 * [x] Open-ended arguments.
 *
 * [x] Open-ended generic parameters.
 *
 * [x] Open-ended reference lists.
 *
 * [x] Correct canonical lexer punctuation names.
 *
 * [x] Generic messages preserved.
 *
 * [x] Message initializers preserved.
 *
 * [x] Message payload boundary available.
 *
 * [x] Distributed adapter compatibility preserved.
 *
 * [x] Quantum/classical data can flow through canonical types.
 *
 * [x] HDL/hardware data can flow through canonical types.
 *
 * [x] Effects/capabilities/resources remain downstream.
 *
 * [x] Contracts/policies remain shared language mechanisms.
 *
 * [x] Provenance remains downstream/frontend-owned.
 *
 * [x] Rust implementation remains compatible with Rust 1.97+.
 *
 * [x] Safe Rust integration remains sufficient.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This grammar defines WHAT a logical message is.
 *
 * It does not define WHERE the message executes or HOW it is realized.
 *
 * Therefore:
 *
 *     message schema
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     semantic message model
 *          |
 *          +--> classical
 *          +--> distributed
 *          +--> networking
 *          +--> hybrid
 *          +--> HDL/hardware
 *          +--> quantum::ir where applicable
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     lowering / routing / scheduling
 *          |
 *          v
 *     target realization
 *
 * One logical message schema can therefore participate in any valid target
 * realization supported by the rest of the Zamani compiler/runtime stack.
 *
 * ============================================================================
 */