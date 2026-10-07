/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/messages.g4
 *
 * Grammar:
 *     DistributedMessages
 *
 * Status:
 *     Production distributed-message integration boundary
 *
 * Language:
 *     Zamani
 *
 * ANTLR:
 *     ANTLR4 parser grammar
 *
 * Rust:
 *     Rust 1.97 or later
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the DISTRIBUTED MESSAGE INTEGRATION BOUNDARY.
 *
 * It does NOT define a second message-schema language.
 *
 * The canonical source-level message schema is owned exclusively by:
 *
 *     grammar/networking/messages.g4
 *
 *     parser grammar Messages;
 *
 * That grammar owns:
 *
 *     messageConstruct
 *     messageDeclaration
 *     messageMember
 *     messageField
 *     messageFieldInitializer
 *     messageValue
 *     messageTypeReference
 *     messageArgumentList
 *     messageFieldReference
 *     messageTypeReferenceList
 *     messageFieldReferenceList
 *
 * This grammar adapts those canonical productions into the distributed
 * domain without redefining them.
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
 *     Zamani parser composition
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Networking Messages       DistributedMessages
 *          |                             |
 *          |                    distributed context
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 domain-neutral AST
 *                        |
 *                        v
 *                 semantic analysis
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *        types        effects      ownership
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                 capabilities
 *                        |
 *                 resources
 *                        |
 *                  policies
 *                        |
 *                  provenance
 *                        |
 *              distributed semantics
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *      classical     quantum::ir    HDL/hardware
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                 optimization
 *                        |
 *               routing / placement
 *                        |
 *                    scheduling
 *                        |
 *                 resilience/recovery
 *                        |
 *                  target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     distributedMessageConstruct
 *     distributedMessageDeclaration
 *     distributedMessageValue
 *     distributedMessageTypeReference
 *     distributedMessagePayload
 *     distributedMessagePayloadList
 *     distributedMessageArgumentList
 *     distributedMessageFieldReference
 *     distributedMessageTypeReferenceList
 *     distributedMessageFieldReferenceList
 *
 * These are DISTRIBUTED INTEGRATION ADAPTERS.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     messageDeclaration
 *     messageMember
 *     messageField
 *     messageFieldInitializer
 *     messageValue
 *     messageTypeReference
 *     messageArgumentList
 *     messageFieldReference
 *
 * Those remain owned by:
 *
 *     grammar/networking/messages.g4
 *
 * THIS FILE ALSO DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     channels
 *     actors
 *     communication operations
 *     endpoints
 *     protocols
 *     routing
 *     placement
 *     scheduling
 *     serialization
 *     encryption
 *     authentication
 *     authorization
 *     replication
 *     consistency
 *     fault tolerance
 *     hardware discovery
 *     quantum topology
 *     quantum gates
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one source grammar owner for logical message schemas.
 *
 * Canonical owner:
 *
 *     grammar/networking/messages.g4
 *
 * Distributed integration:
 *
 *     grammar/distributed/messages.g4
 *
 * Legacy compatibility:
 *
 *     grammar/distributed/messaging.g4
 *
 * if retained, MUST delegate to the canonical networking message grammar and
 * MUST NOT define an independent message schema.
 *
 * A future change to message-field syntax must therefore be made in:
 *
 *     grammar/networking/messages.g4
 *
 * and not duplicated here.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Imported grammar:
 *
 *     Messages
 *
 * Source:
 *
 *     grammar/networking/messages.g4
 *
 * Canonical Messages dependencies:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * This file deliberately imports only Messages.
 *
 * The canonical Messages grammar already owns and imports the lower-level
 * syntax required by its exported rules.
 *
 * This prevents this adapter from recreating the same dependency graph.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file does not define lexer rules.
 *
 * It does not introduce distributed-message-specific tokens.
 *
 * In particular, it does not define:
 *
 *     SEND
 *     RECEIVE
 *     REQUEST
 *     REPLY
 *     BROADCAST
 *     MULTICAST
 *     PUBLISH
 *     SUBSCRIBE
 *     MESSAGE_ID
 *     PAYLOAD
 *     SOURCE
 *     DESTINATION
 *
 * Communication operation names remain owned by the communication layer and
 * may be represented through the canonical identifier/qualified-name model.
 *
 * The `message` declaration keyword remains owned by the canonical lexer and
 * canonical networking message grammar.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Distributed messaging is deliberately open-ended.
 *
 * This file does not enumerate:
 *
 *     transports
 *     protocols
 *     communication operations
 *     node kinds
 *     device kinds
 *     machine kinds
 *     providers
 *     topology kinds
 *     serialization formats
 *     message categories
 *
 * New distributed communication semantics must not require this file to be
 * modified merely because a new implementation or library is introduced.
 *
 * For example, semantic operations such as:
 *
 *     distributed::send(...)
 *     distributed::receive(...)
 *     distributed::broadcast(...)
 *     distributed::stream(...)
 *     vendor::message_exchange(...)
 *
 * are not message-schema syntax.
 *
 * They belong to the communication/channel/distributed semantic layers.
 *
 * ============================================================================
 * MESSAGE MODEL
 * ============================================================================
 *
 * A message is a logical typed value.
 *
 * Canonical declaration:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 * Canonical construction:
 *
 *     UserCreated(id, name)
 *
 * The distributed layer does not decide whether construction:
 *
 *     allocates memory;
 *     copies data;
 *     moves ownership;
 *     serializes data;
 *     compresses data;
 *     encrypts data;
 *     transmits data;
 *     queues data;
 *     schedules execution.
 *
 * Those decisions belong to semantic analysis and downstream realization.
 *
 * ============================================================================
 * DISTRIBUTED MESSAGE CONSTRUCT
 * ============================================================================
 *
 * This is the primary public integration boundary.
 *
 * IMPORTANT:
 *
 * Do NOT include a generic `expression` alternative here.
 *
 * A generic expression would overlap with:
 *
 *     messageValue
 *     messageTypeReference
 *
 * and would make this supposedly message-specific entry point excessively
 * permissive and unnecessarily ambiguous.
 *
 * The public construct therefore contains only actual message constructs.
 */
distributedMessageConstruct
    : distributedMessageDeclaration
    | distributedMessageValue
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE DECLARATION
 * ============================================================================
 *
 * Delegates to the canonical networking message declaration.
 *
 * The declaration schema itself remains owned by:
 *
 *     grammar/networking/messages.g4
 *
 * No distributed-specific field grammar is introduced.
 */
distributedMessageDeclaration
    : messageDeclaration
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE VALUE
 * ============================================================================
 *
 * Delegates to the canonical message construction syntax.
 *
 * Canonical examples:
 *
 *     UserCreated(id, name)
 *
 *     events::UserCreated(id, name)
 *
 * Semantic analysis determines whether the value is:
 *
 *     local;
 *     copied;
 *     moved;
 *     shared;
 *     transferred;
 *     serialized;
 *     transmitted.
 *
 * The parser makes none of those decisions.
 */
distributedMessageValue
    : messageValue
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE TYPE REFERENCE
 * ============================================================================
 *
 * A distributed semantic consumer may need an explicit message-type boundary.
 *
 * The underlying syntax remains canonical.
 */
distributedMessageTypeReference
    : messageTypeReference
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE PAYLOAD
 * ============================================================================
 *
 * A payload is a normal Zamani expression.
 *
 * This rule exists solely as a named integration boundary for distributed
 * consumers.
 *
 * It does NOT mean:
 *
 *     serialized payload;
 *     network packet;
 *     wire data;
 *     copied memory;
 *     owned memory;
 *     transmitted value.
 *
 * Those are semantic/runtime properties.
 */
distributedMessagePayload
    : expression
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE PAYLOAD LIST
 * ============================================================================
 *
 * An unbounded sequence of payload expressions.
 *
 * No finite payload count is encoded.
 *
 * This rule is intentionally useful to communication/channel grammars while
 * remaining independent of any transport.
 */
distributedMessagePayloadList
    : distributedMessagePayload
      (COMMA distributedMessagePayload)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL DISTRIBUTED MESSAGE PAYLOAD LIST
 * ============================================================================
 */
optionalDistributedMessagePayloadList
    : distributedMessagePayloadList?
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE ARGUMENT LIST
 * ============================================================================
 *
 * The canonical message grammar owns message argument syntax.
 *
 * This adapter gives distributed semantic consumers an explicit boundary
 * without creating another argument grammar.
 */
distributedMessageArgumentList
    : messageArgumentList
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE FIELD REFERENCE
 * ============================================================================
 *
 * Delegates to the canonical networking message field reference.
 *
 * Semantic analysis determines whether the referenced symbol is a valid
 * message field in the surrounding scope.
 */
distributedMessageFieldReference
    : messageFieldReference
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE FIELD REFERENCE LIST
 * ============================================================================
 *
 * Unbounded list.
 *
 * No fixed field count is encoded.
 */
distributedMessageFieldReferenceList
    : messageFieldReferenceList
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE TYPE REFERENCE LIST
 * ============================================================================
 *
 * Delegates to the canonical networking message type-reference list.
 */
distributedMessageTypeReferenceList
    : messageTypeReferenceList
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE FRAGMENT
 * ============================================================================
 *
 * This is intentionally a sequence of actual distributed message constructs,
 * not arbitrary expressions.
 *
 * The empty sequence is valid so that composition grammars can use this
 * boundary inside optional distributed regions without introducing a second
 * block grammar.
 */
distributedMessageFragment
    : distributedMessageConstruct*
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE VALUE FRAGMENT
 * ============================================================================
 */
distributedMessageValueFragment
    : distributedMessageValue
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE DECLARATION FRAGMENT
 * ============================================================================
 */
distributedMessageDeclarationFragment
    : distributedMessageDeclaration
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE TYPE FRAGMENT
 * ============================================================================
 */
distributedMessageTypeFragment
    : distributedMessageTypeReference
    ;


/*
 * ============================================================================
 * DISTRIBUTED MESSAGE PAYLOAD FRAGMENT
 * ============================================================================
 */
distributedMessagePayloadFragment
    : distributedMessagePayload
    ;


/*
 * ============================================================================
 * OPTIONAL MESSAGE TYPE
 * ============================================================================
 *
 * This wrapper is useful where a distributed grammar needs an optional
 * message-type boundary.
 */
optionalDistributedMessageTypeReference
    : distributedMessageTypeReference?
    ;


/*
 * ============================================================================
 * OPTIONAL MESSAGE FIELD REFERENCE LIST
 * ============================================================================
 */
optionalDistributedMessageFieldReferenceList
    : distributedMessageFieldReferenceList?
    ;


/*
 * ============================================================================
 * OPTIONAL MESSAGE ARGUMENT LIST
 * ============================================================================
 *
 * Delegates to the canonical message argument grammar through the canonical
 * message value representation.
 *
 * This rule is provided only for consumers that require a named distributed
 * boundary. It does not redefine argument syntax.
 */
optionalDistributedMessageArgumentList
    : messageArgumentList?
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no Rust AST implementation.
 *
 * Parser output is mapped by the frontend into the domain-neutral AST.
 *
 * Expected semantic mapping:
 *
 *     distributedMessageDeclaration
 *         -> canonical message declaration AST
 *
 *     distributedMessageValue
 *         -> canonical message value AST
 *
 *     distributedMessageTypeReference
 *         -> canonical type/name reference AST
 *
 *     distributedMessagePayload
 *         -> canonical expression AST
 *
 * The AST MUST preserve source spans.
 *
 * The AST MUST NOT contain physical:
 *
 *     node IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     addresses;
 *     ports;
 *     routes;
 *     transport handles.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution;
 *     message declaration resolution;
 *     type resolution;
 *     field validation;
 *     generic substitution;
 *     argument matching;
 *     ownership analysis;
 *     lifetime analysis;
 *     transfer semantics;
 *     serialization legality;
 *     capability checking;
 *     effect checking;
 *     resource checking;
 *     security checking;
 *     policy checking;
 *     provenance;
 *     distributed execution semantics.
 *
 * Parser rules must not perform these operations.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring or constructing a message does not automatically imply a network
 * effect.
 *
 * In particular:
 *
 *     message declaration
 *
 * is not itself:
 *
 *     network I/O
 *
 * and:
 *
 *     message value construction
 *
 * is not itself:
 *
 *     message transmission.
 *
 * A communication operation that sends a message may acquire effects such as:
 *
 *     network
 *     distributed
 *     mutation
 *     foreign
 *     native
 *
 * according to the semantic effect system.
 *
 * This grammar does not assign those effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Message syntax does not require a physical capability by itself.
 *
 * A downstream communication operation may require capabilities such as:
 *
 *     capability("distributed.communication")
 *     capability("network.communication")
 *     capability("shared_memory")
 *     capability("ipc")
 *
 * Capability resolution remains outside this grammar.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file introduces no physical resource requirements.
 *
 * Message schema syntax does not contain:
 *
 *     memory limits;
 *     bandwidth limits;
 *     node limits;
 *     queue limits;
 *     device limits;
 *     network limits.
 *
 * If a particular execution requires resources, those requirements are
 * expressed and analyzed by the resource subsystem.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Distributed message semantics may be constrained by policies covering:
 *
 *     communication;
 *     data movement;
 *     locality;
 *     security;
 *     privacy;
 *     provenance;
 *     serialization;
 *     reliability;
 *     deployment.
 *
 * Policy evaluation is downstream.
 *
 * No policy implementation belongs in this grammar.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Message declarations and values must remain source-traceable.
 *
 * Downstream provenance may record:
 *
 *     source declaration;
 *     source value;
 *     transformation;
 *     serialization decision;
 *     routing decision;
 *     placement decision;
 *     execution decision;
 *     verification result.
 *
 * This grammar only preserves the syntactic structure required to establish
 * those records.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Message fields and payloads may contain ordinary Zamani classical values.
 *
 * No special classical-message grammar is required.
 *
 * Canonical path:
 *
 *     message
 *        |
 *        v
 *     canonical AST
 *        |
 *        v
 *     semantic type checking
 *        |
 *        v
 *     classical semantic representation
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Message fields may use canonical quantum-related types where the type system
 * permits them.
 *
 * This grammar does NOT define:
 *
 *     physical qubits;
 *     logical qubits;
 *     quantum gates;
 *     coupling maps;
 *     calibration;
 *     QEC;
 *     physical topology.
 *
 * Where distributed source participates in quantum computation, the semantic
 * pipeline remains:
 *
 *     distributed message
 *          |
 *          v
 *     domain-neutral semantic model
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * There is no distributed-message quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Message values may connect logical computation with HDL/hardware semantics.
 *
 * This grammar does not encode:
 *
 *     bus width;
 *     register width;
 *     physical address;
 *     device number;
 *     link count;
 *     hardware topology.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CONCURRENCY / ACTOR INTEGRATION
 * ============================================================================
 *
 * Distributed messages may be consumed by:
 *
 *     grammar/concurrency/actors.g4
 *
 * and distributed actor semantics.
 *
 * This file does NOT define:
 *
 *     actor lifecycle;
 *     actor spawning;
 *     mailboxes;
 *     supervision;
 *     scheduling;
 *     actor state.
 *
 * The existing actor grammar remains the owner of actor syntax.
 *
 * Message values provide the logical data exchanged by those constructs.
 *
 * ============================================================================
 * CHANNEL INTEGRATION
 * ============================================================================
 *
 * `grammar/distributed/channels.g4` owns distributed channel syntax.
 *
 * Channel operations may consume:
 *
 *     distributedMessageTypeReference
 *     distributedMessageValue
 *     distributedMessagePayload
 *
 * as appropriate to their semantic contracts.
 *
 * This file does not define:
 *
 *     channel capacity;
 *     buffering;
 *     ordering;
 *     delivery guarantees;
 *     transport.
 *
 * ============================================================================
 * COMMUNICATION INTEGRATION
 * ============================================================================
 *
 * `grammar/distributed/communication.g4` owns communication operations.
 *
 * A communication operation may consume a message value or payload.
 *
 * Conceptual flow:
 *
 *     distributed communication
 *             |
 *             v
 *     message value/payload
 *             |
 *             v
 *     semantic operation
 *             |
 *             v
 *     effects/capabilities/resources/policies
 *             |
 *             v
 *     distributed execution representation
 *
 * Communication syntax must not be copied into this file.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * `grammar/networking/messages.g4` remains the canonical logical message
 * schema owner.
 *
 * Networking realization may later select:
 *
 *     local memory;
 *     IPC;
 *     shared memory;
 *     transport protocol;
 *     network fabric;
 *     accelerator fabric;
 *     hardware link;
 *     future communication substrate.
 *
 * This file remains independent of that selection.
 *
 * ============================================================================
 * SERIALIZATION INTEGRATION
 * ============================================================================
 *
 * No serialization format is selected here.
 *
 * A logical message may later be represented through an implementation-defined
 * or negotiated format.
 *
 * The semantic system must preserve the distinction between:
 *
 *     logical message
 *
 * and:
 *
 *     physical representation.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO language-level finite limits in this grammar for:
 *
 *     message declarations;
 *     message fields;
 *     message arguments;
 *     message payloads;
 *     message types;
 *     distributed participants;
 *     channels;
 *     nodes;
 *     processes;
 *     actors;
 *     workers;
 *     machines;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     clusters;
 *     regions;
 *     networks;
 *     memory;
 *     bandwidth.
 *
 * This grammar contains no:
 *
 *     MAX_MESSAGES
 *     MAX_MESSAGE_FIELDS
 *     MAX_MESSAGE_SIZE
 *     MAX_PAYLOAD_SIZE
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_ACTORS
 *     MAX_CHANNELS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * Repeated grammar structures use ANTLR repetition.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-level ceiling.
 *
 * It does not mean an implementation can exceed the physical or computational
 * resources actually available.
 *
 * Resource exhaustion belongs to compiler/runtime/deployment semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     token stream;
 *     grammar;
 *     selected language/compatibility configuration.
 *
 * Parsing must not depend on:
 *
 *     hardware;
 *     network state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     runtime state;
 *     scheduler state;
 *     target availability.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no unsafe code;
 *     no I/O;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no runtime callbacks.
 *
 * Generated Rust must therefore remain compatible with the repository's
 * safe-Rust requirement.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical message syntax remains valid:
 *
 *     message Name {
 *         field: Type;
 *     }
 *
 * and:
 *
 *     Name(value)
 *
 * Compatibility handling belongs to:
 *
 *     grammar/compatibility/
 *
 * This file must not introduce duplicate lexical aliases or alternate message
 * schema syntax.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * PARENT:
 *
 *     grammar/distributed/distributed.g4
 *
 * must import:
 *
 *     DistributedMessages
 *
 * and expose:
 *
 *     distributedMessageConstruct
 *
 * as the distributed message boundary.
 *
 * The parent must NOT simultaneously import an independent message-schema
 * grammar for the same source constructs.
 *
 * LEGACY:
 *
 *     grammar/distributed/messaging.g4
 *
 * should be reduced to a compatibility/delegation layer or retired after
 * repository-wide migration.
 *
 * It must not remain an independent implementation of:
 *
 *     messageDeclaration
 *     messageField
 *     messageValue
 *     messageTypeReference
 *
 * COMMUNICATION:
 *
 *     grammar/distributed/communication.g4
 *
 * remains the owner of communication operations.
 *
 * CHANNELS:
 *
 *     grammar/distributed/channels.g4
 *
 * remains the owner of distributed channel syntax.
 *
 * NETWORKING:
 *
 *     grammar/networking/messages.g4
 *
 * remains the canonical message-schema authority.
 *
 * AST:
 *
 *     frontend AST subsystem
 *
 * owns actual Rust AST data structures.
 *
 * SEMANTICS:
 *
 *     semantic analysis
 *
 * owns resolution, validation, effects, capabilities, resources, policies and
 * distributed message meaning.
 *
 * IR:
 *
 *     canonical classical IR
 *     quantum::ir
 *
 * own downstream representations.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Correct direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names / Types / Expressions / Attributes
 *          |
 *          v
 *     networking::Messages
 *          |
 *          v
 *     distributed::DistributedMessages
 *          |
 *          v
 *     distributed::Distributed
 *          |
 *          v
 *     Zamani parser composition
 *
 * This file MUST NOT import:
 *
 *     Distributed
 *     Communication
 *     DistributedChannels
 *     runtime grammars
 *     compiler grammars
 *     quantum::ir
 *     HAL grammars
 *
 * Doing so would invert ownership and risk dependency cycles.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * LEXICAL:
 *
 *     message keyword is recognized by the canonical lexer.
 *
 * PARSER POSITIVE:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 *     UserCreated(id, name)
 *
 *     events::UserCreated(id, name)
 *
 *     message Envelope<T> {
 *         payload: T;
 *     }
 *
 * PARSER NEGATIVE:
 *
 *     message {
 *     }
 *
 *     message Name {
 *         : Type;
 *     }
 *
 *     message Name {
 *         field Type;
 *     }
 *
 *     message Name {
 *         field: Type
 *     }
 *
 *     message Name {
 *         field: Type;;
 *     }
 *
 * Semantic validation, rather than parser syntax, owns:
 *
 *     duplicate fields;
 *     unknown message types;
 *     invalid generic substitutions;
 *     invalid default values;
 *     invalid argument types;
 *     ownership violations.
 *
 * CROSS-DOMAIN:
 *
 *     classical message payload;
 *     tensor message payload;
 *     quantum-related typed message;
 *     measurement result message;
 *     HDL control/data message;
 *     actor message;
 *     distributed channel message;
 *     networking message.
 *
 * SCALABILITY:
 *
 *     arbitrarily many declarations;
 *     arbitrarily many fields;
 *     arbitrarily many arguments;
 *     arbitrarily deep qualified names supported by the canonical name grammar;
 *     arbitrarily large source programs subject only to implementation
 *     resources.
 *
 * DETERMINISM:
 *
 * Identical token streams and language configuration produce identical parse
 * structure.
 *
 * PORTABILITY:
 *
 * The grammar contains no physical target selection.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [x] Grammar name is DistributedMessages.
 *
 *     [x] Canonical ZamaniLexer is used.
 *
 *     [x] Canonical Messages grammar is the only message-schema dependency.
 *
 *     [x] No message-schema rules are redefined.
 *
 *     [x] No communication operations are defined.
 *
 *     [x] No channel operations are defined.
 *
 *     [x] No transport syntax is defined.
 *
 *     [x] No serialization format is defined.
 *
 *     [x] No physical topology is defined.
 *
 *     [x] No target-specific capacity is defined.
 *
 *     [x] No finite hardware limits are defined.
 *
 *     [x] No semantic actions exist.
 *
 *     [x] No semantic predicates exist.
 *
 *     [x] No unsafe Rust is required.
 *
 *     [x] Distributed message entry points are explicit.
 *
 *     [x] Generic expression alternatives are not used as a message root.
 *
 *     [x] Canonical message construction remains authoritative.
 *
 *     [x] AST ownership is documented.
 *
 *     [x] semantic ownership is documented.
 *
 *     [x] IR ownership is documented.
 *
 *     [x] communication integration is documented.
 *
 *     [x] channel integration is documented.
 *
 *     [x] networking integration is documented.
 *
 *     [x] quantum integration is documented.
 *
 *     [x] classical integration is documented.
 *
 *     [x] HDL/hardware integration is documented.
 *
 *     [x] scalability contract is documented.
 *
 *     [x] determinism contract is documented.
 *
 *     [x] safety contract is documented.
 *
 *     [x] compatibility contract is documented.
 *
 * Repository-level completion additionally requires:
 *
 *     [ ] grammar/distributed/distributed.g4 imports DistributedMessages.
 *
 *     [ ] distributed.g4 uses distributedMessageConstruct.
 *
 *     [ ] duplicate message-schema ownership in messaging.g4 is removed or
 *         converted to delegation.
 *
 *     [ ] ANTLR generation succeeds.
 *
 *     [ ] Rust 1.97+ frontend generation/build succeeds.
 *
 *     [ ] positive parser tests pass.
 *
 *     [ ] negative parser tests pass.
 *
 *     [ ] semantic tests pass.
 *
 *     [ ] cross-domain tests pass.
 *
 *     [ ] scalability tests pass within available resources.
 *
 *     [ ] deterministic parsing tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How does a canonical logical message enter distributed grammar
 *      composition?"
 *
 * It does NOT answer:
 *
 *     "How is the message transported?"
 *
 *     "Where is it placed?"
 *
 *     "How is it serialized?"
 *
 *     "Which node executes it?"
 *
 *     "Which CPU/GPU/FPGA/ASIC/QPU executes it?"
 *
 *     "Which network carries it?"
 *
 *     "How is it routed?"
 *
 *     "How is it replicated?"
 *
 *     "How is it scheduled?"
 *
 *     "How is it recovered?"
 *
 * Those decisions belong downstream.
 *
 * The resulting dependency is:
 *
 *     canonical logical message
 *             |
 *             v
 *     distributed integration boundary
 *             |
 *             v
 *     semantic analysis
 *             |
 *       +-----+-----+-----+-----+
 *       |     |     |     |     |
 *      type effect resource policy provenance
 *             |
 *             v
 *     distributed execution semantics
 *             |
 *             +--> classical
 *             +--> quantum::ir
 *             +--> HDL/hardware
 *             +--> networking
 *             +--> future substrates
 *
 * This preserves target independence, open-ended scalability, deterministic
 * parsing, safe Rust integration and POCO-REAF.
 *
 * ============================================================================
 * END OF grammar/distributed/messages.g4
 * ============================================================================
 */

parser grammar DistributedMessages;

options {
    tokenVocab = ZamaniLexer;
}

import Messages;


/*
 * ============================================================================
 * PUBLIC DISTRIBUTED MESSAGE BOUNDARY
 * ============================================================================
 *
 * Only actual message constructs belong here.
 *
 * Generic expressions deliberately do not appear as alternatives because they
 * would make the message boundary overlap with the entire expression grammar.
 */
distributedMessageConstruct
    : distributedMessageDeclaration
    | distributedMessageValue
    ;


/*
 * ============================================================================
 * CANONICAL MESSAGE DECLARATION ADAPTER
 * ============================================================================
 */
distributedMessageDeclaration
    : messageDeclaration
    ;


/*
 * ============================================================================
 * CANONICAL MESSAGE VALUE ADAPTER
 * ============================================================================
 */
distributedMessageValue
    : messageValue
    ;


/*
 * ============================================================================
 * CANONICAL MESSAGE TYPE REFERENCE ADAPTER
 * ============================================================================
 *
 * Used by distributed constructs that need to refer to a message type without
 * constructing a value.
 */
distributedMessageTypeReference
    : messageTypeReference
    ;


/*
 * ============================================================================
 * PAYLOAD ADAPTER
 * ============================================================================
 *
 * A payload is a normal Zamani expression.
 *
 * No transmission, ownership, serialization or allocation semantics are
 * implied by parsing this rule.
 */
distributedMessagePayload
    : expression
    ;


/*
 * ============================================================================
 * PAYLOAD LIST ADAPTER
 * ============================================================================
 *
 * Open-ended cardinality; no finite payload limit is encoded.
 */
distributedMessagePayloadList
    : distributedMessagePayload
      (COMMA distributedMessagePayload)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL PAYLOAD LIST
 * ============================================================================
 */
optionalDistributedMessagePayloadList
    : distributedMessagePayloadList?
    ;


/*
 * ============================================================================
 * ARGUMENT LIST ADAPTER
 * ============================================================================
 *
 * Canonical argument syntax remains owned by Messages.
 */
distributedMessageArgumentList
    : messageArgumentList
    ;


/*
 * ============================================================================
 * FIELD REFERENCE ADAPTER
 * ============================================================================
 */
distributedMessageFieldReference
    : messageFieldReference
    ;


/*
 * ============================================================================
 * FIELD REFERENCE LIST ADAPTER
 * ============================================================================
 */
distributedMessageFieldReferenceList
    : messageFieldReferenceList
    ;


/*
 * ============================================================================
 * TYPE REFERENCE LIST ADAPTER
 * ============================================================================
 */
distributedMessageTypeReferenceList
    : messageTypeReferenceList
    ;


/*
 * ============================================================================
 * MESSAGE FRAGMENT
 * ============================================================================
 *
 * A fragment is a sequence of actual distributed message constructs.
 */
distributedMessageFragment
    : distributedMessageConstruct*
    ;


/*
 * ============================================================================
 * SINGLE-VALUE FRAGMENTS
 * ============================================================================
 */
distributedMessageValueFragment
    : distributedMessageValue
    ;


distributedMessageDeclarationFragment
    : distributedMessageDeclaration
    ;


distributedMessageTypeFragment
    : distributedMessageTypeReference
    ;


distributedMessagePayloadFragment
    : distributedMessagePayload
    ;


/*
 * ============================================================================
 * OPTIONAL TYPE REFERENCE
 * ============================================================================
 */
optionalDistributedMessageTypeReference
    : distributedMessageTypeReference?
    ;


/*
 * ============================================================================
 * OPTIONAL FIELD REFERENCE LIST
 * ============================================================================
 */
optionalDistributedMessageFieldReferenceList
    : distributedMessageFieldReferenceList?
    ;


/*
 * ============================================================================
 * OPTIONAL ARGUMENT LIST
 * ============================================================================
 *
 * This is an integration adapter only. The canonical argument syntax remains
 * owned by Messages.
 */
optionalDistributedMessageArgumentList
    : messageArgumentList?
    ;