/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Distributed Message Integration Grammar
 * ============================================================================
 *
 * File:
 *     grammar/distributed/messages.g4
 *
 * Grammar:
 *     DistributedMessages
 *
 * Status:
 *     Production distributed-message integration contract.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No environment queries.
 *     - No unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the DISTRIBUTED-DOMAIN INTEGRATION BOUNDARY for messages.
 *
 * IMPORTANT:
 *
 * This file does NOT become a second message-schema grammar.
 *
 * The canonical source-level message schema is owned by:
 *
 *     grammar/networking/messages.g4
 *
 * whose parser grammar name is:
 *
 *     Messages
 *
 * This file consumes that canonical message grammar and exposes distributed
 * semantic wrappers around it.
 *
 * The architecture is therefore:
 *
 *     grammar/networking/messages.g4
 *                 |
 *                 v
 *          canonical message
 *          schema/value syntax
 *                 |
 *                 v
 *     grammar/distributed/messages.g4
 *                 |
 *                 v
 *       distributed message
 *       integration boundary
 *                 |
 *                 v
 *       distributed semantic model
 *
 * This prevents:
 *
 *     networking message grammar
 *             +
 *     distributed message grammar
 *
 * from becoming two independent definitions of the same message language.
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * MESSAGE SCHEMA OWNERSHIP
 *
 *     grammar/networking/messages.g4
 *
 * owns:
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
 * DISTRIBUTED MESSAGE INTEGRATION
 *
 *     grammar/distributed/messages.g4
 *
 * owns:
 *
 *     distributedMessageConstruct
 *     distributedMessageDeclaration
 *     distributedMessageValue
 *     distributedMessageTypeReference
 *     distributedMessagePayload
 *     distributedMessagePayloadList
 *     distributedMessageFieldReference
 *     distributedMessageTypeReferenceList
 *     distributedMessageReference
 *     distributedMessageFragment
 *
 * COMMUNICATION
 *
 *     grammar/distributed/communication.g4
 *
 * owns:
 *
 *     distributedCommunication
 *     communicationOperation
 *     communicationName
 *     communicationArguments
 *
 * CHANNELS
 *
 *     grammar/distributed/channels.g4
 *
 * owns:
 *
 *     distributed channel declarations;
 *     channel semantics;
 *     channel policies;
 *     channel operations.
 *
 * NETWORKING
 *
 *     grammar/networking/
 *
 * owns:
 *
 *     network-specific realization;
 *     endpoints;
 *     protocols;
 *     transport;
 *     network channels;
 *     networking capabilities.
 *
 * Therefore this file MUST NOT redefine those concerns.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A distributed program needs a stable grammar-level boundary through which
 * message schemas and message values participate in distributed semantics.
 *
 * Without this boundary, parent grammars tend to directly consume generic
 * networking message rules everywhere.
 *
 * That causes:
 *
 *     distributed grammar
 *          |
 *          +--> networking message details
 *          +--> channel details
 *          +--> transport details
 *          +--> protocol details
 *          +--> runtime details
 *
 * and eventually creates domain coupling.
 *
 * This file prevents that.
 *
 * The intended architecture is:
 *
 *     message schema
 *          |
 *          v
 *     distributed message reference/value
 *          |
 *          v
 *     communication intent
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type checking
 *          +--> ownership analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> distributed analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical execution
 *          +--> distributed execution
 *          +--> networking
 *          +--> quantum::ir where quantum semantics participate
 *          +--> HDL/hardware representation where applicable
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     runtime realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed message syntax participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source describes:
 *
 *     WHAT message exists;
 *     WHAT message type is involved;
 *     WHAT value is being exchanged;
 *     WHAT logical message participates in distributed computation.
 *
 * The source does NOT describe:
 *
 *     WHERE the message physically exists;
 *     HOW the message is serialized;
 *     HOW the message is transported;
 *     WHICH machine carries the message;
 *     WHICH node carries the message;
 *     WHICH process carries the message;
 *     WHICH CPU carries the message;
 *     WHICH GPU carries the message;
 *     WHICH FPGA carries the message;
 *     WHICH QPU carries the message;
 *     WHICH network carries the message;
 *     WHICH transport protocol carries the message;
 *     WHICH provider hosts the message.
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately imposes NO language-level finite limits on:
 *
 *     message declarations;
 *     message fields;
 *     message arguments;
 *     message types;
 *     payload expressions;
 *     payload lists;
 *     qualified-name depth;
 *     distributed entities;
 *     distributed participants;
 *     communication operations;
 *     channels;
 *     nodes;
 *     processes;
 *     workers;
 *     services;
 *     actors;
 *     replicas;
 *     partitions;
 *     clusters;
 *     machines;
 *     devices;
 *     accelerators;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     QPUs;
 *     memory;
 *     network size.
 *
 * There is intentionally NO:
 *
 *     MAX_MESSAGES
 *     MAX_MESSAGE_FIELDS
 *     MAX_MESSAGE_SIZE
 *     MAX_PAYLOAD_SIZE
 *     MAX_MESSAGE_TYPES
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_CHANNELS
 *     MAX_REPLICAS
 *     MAX_PARTITIONS
 *     MAX_CLUSTERS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * The grammar uses recursive and repeated structures where cardinality is
 * semantically unbounded.
 *
 * Practical limits remain implementation/resource constraints.
 *
 * They may arise from:
 *
 *     parser memory;
 *     compiler memory;
 *     compiler time;
 *     runtime memory;
 *     runtime time;
 *     operating-system resources;
 *     deployment resources;
 *     target capabilities;
 *     network capacity;
 *     physical hardware.
 *
 * Those limits MUST NOT become universal language limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     node0
 *     node1
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     device0
 *     channel0
 *     physical_qubit0
 *     replica0
 *
 * as universal distributed-message constructs.
 *
 * Names such as those may be ordinary user identifiers where legal.
 *
 * Their semantic interpretation, if any, belongs downstream.
 *
 * Likewise this grammar MUST NOT contain fixed-size constructs such as:
 *
 *     messageField0
 *     messageField1
 *     messageField2
 *
 * or:
 *
 *     payload0
 *     payload1
 *     payload2
 *
 * Repetition must remain open-ended.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * This file does not enumerate message kinds.
 *
 * It does not create lexer tokens for:
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
 * Those operations belong to communication/channel/service semantics.
 *
 * A future operation such as:
 *
 *     distributed::stream
 *
 * or:
 *
 *     future::message_exchange
 *
 * remains syntactically representable through the canonical communication
 * grammar without modifying this file.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No lexer grammar is created here.
 *
 * This file MUST NOT introduce distributed-message lexer tokens.
 *
 * In particular, it must not introduce:
 *
 *     DISTRIBUTED_MESSAGE
 *     SEND
 *     RECEIVE
 *     MESSAGE_ID
 *     PAYLOAD
 *     DESTINATION
 *     SOURCE
 *
 * merely to support message semantics.
 *
 * Existing language tokens remain sufficient.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Canonical message syntax is imported from:
 *
 *     grammar/networking/messages.g4
 *
 * whose grammar name is:
 *
 *     Messages
 *
 * This file therefore consumes:
 *
 *     messageDeclaration
 *     messageValue
 *     messageTypeReference
 *     messageFieldReference
 *     messageArgumentList
 *
 * through that canonical grammar.
 *
 * Names and expressions are already part of the canonical message grammar,
 * but this file imports the foundational grammars explicitly where its own
 * wrappers need them.
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     typeExpression
 *     messageDeclaration
 *     messageField
 *     messageValue
 *     messageTypeReference
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 */

parser grammar DistributedMessages;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions,
    Attributes,
    Messages
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable distributed-domain message boundary.
 *
 * A parent distributed grammar should consume:
 *
 *     distributedMessageConstruct
 *
 * rather than directly depending on the internal message-schema grammar.
 *
 * This preserves a clean integration boundary.
 */
distributedMessageConstruct
    : distributedMessageDeclaration
    | distributedMessageValue
    | distributedMessageReference
    | distributedMessagePayload
    | distributedMessagePayloadList
    ;


/*
 * ============================================================================
 * 2. MESSAGE DECLARATION ADAPTER
 * ============================================================================
 *
 * This rule delegates completely to the canonical networking message
 * declaration.
 *
 * It does NOT redefine messageDeclaration.
 *
 * Example:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 * The canonical declaration remains owned by:
 *
 *     grammar/networking/messages.g4
 */
distributedMessageDeclaration
    : messageDeclaration
    ;


/*
 * ============================================================================
 * 3. MESSAGE VALUE ADAPTER
 * ============================================================================
 *
 * Example:
 *
 *     UserCreated(id, name)
 *
 *     events::UserCreated(id, name)
 *
 * Construction semantics remain downstream.
 */
distributedMessageValue
    : messageValue
    ;


/*
 * ============================================================================
 * 4. MESSAGE TYPE REFERENCE
 * ============================================================================
 *
 * This wrapper provides distributed semantic context without creating another
 * message type system.
 *
 * Example:
 *
 *     UserCreated
 *
 *     events::UserCreated
 *
 *     telemetry::Measurement
 */
distributedMessageTypeReference
    : messageTypeReference
    ;


/*
 * ============================================================================
 * 5. MESSAGE REFERENCE
 * ============================================================================
 *
 * A reference is intentionally an alias of the canonical message type
 * reference.
 *
 * Semantic analysis determines whether the referenced symbol is:
 *
 *     a message declaration;
 *     a message value;
 *     another compatible declaration;
 *     unresolved;
 *     inaccessible;
 *     invalid in context.
 *
 * The parser does not perform symbol resolution.
 */
distributedMessageReference
    : distributedMessageTypeReference
    ;


/*
 * ============================================================================
 * 6. MESSAGE PAYLOAD
 * ============================================================================
 *
 * A distributed message payload is a normal Zamani expression.
 *
 * Examples:
 *
 *     value
 *
 *     compute(x)
 *
 *     tensor[index]
 *
 *     measurement
 *
 *     classical_result
 *
 *     quantum_result
 *
 * The expression may ultimately participate in:
 *
 *     classical computation;
 *     quantum/classical computation;
 *     distributed computation;
 *     HDL/hardware interaction;
 *     AI/data computation.
 *
 * This rule does NOT imply:
 *
 *     serialization;
 *     copying;
 *     transmission;
 *     allocation;
 *     ownership transfer;
 *     network transport.
 */
distributedMessagePayload
    : expression
    ;


/*
 * ============================================================================
 * 7. MESSAGE PAYLOAD LIST
 * ============================================================================
 *
 * Unbounded list of payload expressions.
 *
 * This is deliberately separate from the canonical `expressionList` so that
 * the AST/semantic layer has an explicit distributed-message ownership
 * boundary.
 *
 * It remains structurally equivalent to a normal expression sequence.
 */
distributedMessagePayloadList
    : distributedMessagePayload
      (
          COMMA
          distributedMessagePayload
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 8. OPTIONAL MESSAGE PAYLOAD LIST
 * ============================================================================
 */
optionalDistributedMessagePayloadList
    : distributedMessagePayloadList?
    ;


/*
 * ============================================================================
 * 9. MESSAGE ARGUMENT LIST ADAPTER
 * ============================================================================
 *
 * This wrapper allows distributed semantic consumers to distinguish message
 * arguments from arbitrary argument lists without creating a second argument
 * grammar.
 *
 * The canonical message grammar owns the underlying syntax.
 */
distributedMessageArgumentList
    : messageArgumentList
    ;


/*
 * ============================================================================
 * 10. MESSAGE FIELD REFERENCE
 * ============================================================================
 *
 * Delegates to the canonical message field reference.
 */
distributedMessageFieldReference
    : messageFieldReference
    ;


/*
 * ============================================================================
 * 11. MESSAGE FIELD REFERENCE LIST
 * ============================================================================
 *
 * No finite number of fields is encoded.
 */
distributedMessageFieldReferenceList
    : distributedMessageFieldReference
      (
          COMMA
          distributedMessageFieldReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. MESSAGE TYPE REFERENCE LIST
 * ============================================================================
 *
 * A distributed semantic consumer may need a list of message types.
 *
 * This remains an ordinary unbounded source list.
 */
distributedMessageTypeReferenceList
    : distributedMessageTypeReference
      (
          COMMA
          distributedMessageTypeReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 13. OPTIONAL MESSAGE TYPE REFERENCE LIST
 * ============================================================================
 */
optionalDistributedMessageTypeReferenceList
    : distributedMessageTypeReferenceList?
    ;


/*
 * ============================================================================
 * 14. MESSAGE FRAGMENT
 * ============================================================================
 *
 * This is NOT a second program root.
 *
 * It is a compositional fragment for distributed grammar consumers.
 */
distributedMessageFragment
    : distributedMessageConstruct*
    ;


/*
 * ============================================================================
 * 15. MESSAGE VALUE FRAGMENT
 * ============================================================================
 *
 * Useful to parent grammars that need a message value but not a declaration.
 */
distributedMessageValueFragment
    : distributedMessageValue
    ;


/*
 * ============================================================================
 * 16. MESSAGE TYPE FRAGMENT
 * ============================================================================
 */
distributedMessageTypeFragment
    : distributedMessageTypeReference
    ;


/*
 * ============================================================================
 * 17. MESSAGE PAYLOAD FRAGMENT
 * ============================================================================
 */
distributedMessagePayloadFragment
    : distributedMessagePayload
    ;


/*
 * ============================================================================
 * 18. SOURCE-LEVEL DISTRIBUTED MESSAGE ROLE
 * ============================================================================
 *
 * This rule deliberately describes only a syntactic role.
 *
 * It does NOT classify the message as:
 *
 *     source;
 *     destination;
 *     request;
 *     response;
 *     event;
 *     command;
 *     reply;
 *     broadcast;
 *     multicast.
 *
 * Such classification belongs to semantic analysis or the communication
 * subsystem.
 *
 * This avoids embedding distributed runtime policy into the grammar.
 */
distributedMessageRole
    : distributedMessageRoleName
    ;


distributedMessageRoleName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 19. MESSAGE CONTEXT
 * ============================================================================
 *
 * A context name may be used by a semantic consumer to identify a logical
 * message context.
 *
 * The grammar does not resolve it.
 */
distributedMessageContext
    : qualifiedName
    ;


/*
 * ============================================================================
 * 20. MESSAGE METADATA
 * ============================================================================
 *
 * Distributed message metadata must use canonical attributes rather than
 * creating a distributed-message-specific annotation language.
 *
 * This rule provides the explicit integration boundary.
 */
distributedMessageMetadata
    : attribute*
    ;


/*
 * ============================================================================
 * 21. MESSAGE DECLARATION WITH DISTRIBUTED CONTEXT
 * ============================================================================
 *
 * This rule is intentionally an adapter rather than a new declaration syntax.
 *
 * It allows parent distributed grammar composition to preserve distributed
 * context while still delegating the actual schema to `Messages`.
 *
 * Example:
 *
 *     @distributed
 *     message Event {
 *         value: Data;
 *     }
 *
 * The exact meaning of the attribute is semantic.
 *
 * No distributed keyword is introduced here.
 */
distributedMessageDeclarationWithMetadata
    : distributedMessageMetadata
      distributedMessageDeclaration
    ;


/*
 * ============================================================================
 * 22. MESSAGE VALUE WITH DISTRIBUTED CONTEXT
 * ============================================================================
 */
distributedMessageValueWithMetadata
    : distributedMessageMetadata
      distributedMessageValue
    ;


/*
 * ============================================================================
 * 23. MESSAGE CONSTRUCTION REFERENCE
 * ============================================================================
 *
 * This rule keeps construction/reference semantics separate in the parse tree.
 */
distributedMessageConstruction
    : distributedMessageValue
    ;


/*
 * ============================================================================
 * 24. MESSAGE SCHEMA REFERENCE
 * ============================================================================
 */
distributedMessageSchemaReference
    : distributedMessageTypeReference
    ;


/*
 * ============================================================================
 * 25. MESSAGE PAYLOAD EXPRESSION
 * ============================================================================
 *
 * Explicit semantic wrapper around an ordinary expression.
 */
distributedMessagePayloadExpression
    : distributedMessagePayload
    ;


/*
 * ============================================================================
 * 26. MESSAGE PAYLOAD EXPRESSION LIST
 * ============================================================================
 */
distributedMessagePayloadExpressionList
    : distributedMessagePayloadList
    ;


/*
 * ============================================================================
 * 27. MESSAGE DECLARATION OR VALUE
 * ============================================================================
 *
 * This rule is useful for parent composition where the context accepts either
 * a schema declaration or a value.
 */
distributedMessageDeclarationOrValue
    : distributedMessageDeclaration
    | distributedMessageValue
    ;


/*
 * ============================================================================
 * 28. MESSAGE TYPE OR VALUE
 * ============================================================================
 */
distributedMessageTypeOrValue
    : distributedMessageTypeReference
    | distributedMessageValue
    ;


/*
 * ============================================================================
 * 29. MESSAGE DATA
 * ============================================================================
 *
 * A message datum can be either:
 *
 *     a message value;
 *     a payload expression.
 *
 * The semantic layer determines whether the expression is a valid message
 * payload for the relevant message type.
 */
distributedMessageData
    : distributedMessageValue
    | distributedMessagePayload
    ;


/*
 * ============================================================================
 * 30. MESSAGE DATA LIST
 * ============================================================================
 */
distributedMessageDataList
    : distributedMessageData
      (
          COMMA
          distributedMessageData
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 31. OPTIONAL MESSAGE DATA LIST
 * ============================================================================
 */
optionalDistributedMessageDataList
    : distributedMessageDataList?
    ;


/*
 * ============================================================================
 * 32. MESSAGE NAME
 * ============================================================================
 *
 * This is a semantic wrapper around canonical qualifiedName.
 *
 * No MessageName token is introduced.
 */
distributedMessageName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 33. MESSAGE TYPE NAME
 * ============================================================================
 */
distributedMessageTypeName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 34. MESSAGE REFERENCE LIST
 * ============================================================================
 */
distributedMessageReferenceList
    : distributedMessageReference
      (
          COMMA
          distributedMessageReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 35. OPTIONAL MESSAGE REFERENCE LIST
 * ============================================================================
 */
optionalDistributedMessageReferenceList
    : distributedMessageReferenceList?
    ;


/*
 * ============================================================================
 * 36. MESSAGE FIELD LIST
 * ============================================================================
 *
 * This rule intentionally references the canonical field representation
 * through the canonical declaration grammar.
 *
 * It does not redefine messageField.
 *
 * The declaration itself remains the authority for field ordering and
 * structure.
 *
 * This wrapper exists for semantic tooling and parent grammar composition.
 */
distributedMessageFieldList
    : messageField
      (
          messageField
      )*
    ;


/*
 * ============================================================================
 * 37. MESSAGE DECLARATION LIST
 * ============================================================================
 *
 * No finite declaration count is imposed.
 *
 * Parent grammar consumers should normally use this fragment rather than
 * inventing a fixed-size list.
 */
distributedMessageDeclarationList
    : distributedMessageDeclaration*
    ;


/*
 * ============================================================================
 * 38. MESSAGE VALUE LIST
 * ============================================================================
 */
distributedMessageValueList
    : distributedMessageValue*
    ;


/*
 * ============================================================================
 * 39. MESSAGE TYPE LIST
 * ============================================================================
 */
distributedMessageTypeList
    : distributedMessageTypeReference*
    ;


/*
 * ============================================================================
 * 40. MESSAGE PAYLOAD LIST FRAGMENT
 * ============================================================================
 */
distributedMessagePayloadListFragment
    : distributedMessagePayloadList?
    ;


/*
 * ============================================================================
 * 41. DISTRIBUTED MESSAGE COMPATIBILITY ADAPTER
 * ============================================================================
 *
 * This is the principal migration rule for the existing:
 *
 *     grammar/distributed/messaging.g4
 *
 * implementation.
 *
 * Existing distributed grammar consumers should eventually migrate from
 * directly consuming:
 *
 *     messagingConstruct
 *
 * to:
 *
 *     distributedMessageConstruct
 *
 * without changing the underlying message schema.
 */
distributedMessageCompatibilityConstruct
    : distributedMessageConstruct
    ;


/*
 * ============================================================================
 * 42. LEGACY MESSAGE SCHEMA ADAPTER
 * ============================================================================
 *
 * Existing distributed-message consumers may use this rule during migration.
 *
 * It deliberately delegates to the canonical message grammar.
 *
 * No legacy syntax is reimplemented here.
 */
distributedLegacyMessage
    : distributedMessageDeclaration
    | distributedMessageValue
    ;


/*
 * ============================================================================
 * 43. CANONICAL MESSAGE CONSTRUCT ADAPTER
 * ============================================================================
 *
 * The semantic intent is:
 *
 *     distributedMessageConstruct
 *          ->
 *     canonical Messages construct
 *
 * No independent AST hierarchy is implied.
 */
distributedCanonicalMessageConstruct
    : messageConstruct
    ;


/*
 * ============================================================================
 * 44. AST CONTRACT
 * ============================================================================
 *
 * This grammar must map to the EXISTING DOMAIN-NEUTRAL FRONTEND AST.
 *
 * It must not require:
 *
 *     DistributedMessageAst
 *     NetworkMessageAst
 *     MessageTransportAst
 *     MessagePacketAst
 *     MessageEnvelopeAst
 *
 * merely because a message occurs in distributed code.
 *
 * The preferred conceptual mapping is:
 *
 *     distributedMessageDeclaration
 *         |
 *         v
 *     canonical message declaration AST
 *
 *     distributedMessageValue
 *         |
 *         v
 *     canonical message/value AST
 *
 *     distributedMessagePayload
 *         |
 *         v
 *     canonical expression AST
 *
 *     distributedMessageTypeReference
 *         |
 *         v
 *     canonical type/name reference AST
 *
 * The distributed context is preserved by the enclosing distributed AST
 * construct, semantic context, attributes, or source span—not by duplicating
 * the underlying message node hierarchy.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve source spans for:
 *
 *     distributed message construct;
 *     message declaration;
 *     message name;
 *     message type reference;
 *     message value;
 *     message arguments;
 *     payload;
 *     payload list;
 *     field references;
 *     attributes;
 *     enclosing distributed construct.
 *
 * This grammar does not calculate source offsets.
 *
 * The canonical lexer/parser frontend owns source-span construction.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     message-name resolution;
 *     message-type resolution;
 *     field resolution;
 *     visibility;
 *     module ownership;
 *     generic substitution;
 *     type compatibility;
 *     payload compatibility;
 *     ownership;
 *     borrowing;
 *     effect checking;
 *     capability checking;
 *     resource requirements;
 *     security requirements;
 *     distributed participant validity;
 *     communication validity;
 *     serialization compatibility;
 *     delivery semantics;
 *     consistency semantics;
 *     retry semantics;
 *     fault semantics.
 *
 * The parser performs NONE of those operations.
 *
 * ============================================================================
 * COMMUNICATION CONTRACT
 * ============================================================================
 *
 * Message syntax is separate from communication syntax.
 *
 * Therefore:
 *
 *     distributedMessageConstruct
 *
 * describes the message side of communication.
 *
 *     distributedCommunication
 *
 * describes communication operation intent.
 *
 * The relationship is semantic:
 *
 *     message
 *        |
 *        v
 *     communication
 *        |
 *        v
 *     distributed execution
 *
 * The grammar must not silently turn:
 *
 *     message declaration
 *
 * into:
 *
 *     send operation.
 *
 * A message schema does not imply that it is transmitted.
 *
 * ============================================================================
 * CHANNEL CONTRACT
 * ============================================================================
 *
 * Channels belong to:
 *
 *     grammar/distributed/channels.g4
 *
 * A channel may carry a message type or message value.
 *
 * This file must not redefine:
 *
 *     channel;
 *     endpoint;
 *     queue;
 *     channel policy;
 *     channel capacity;
 *     channel ordering;
 *     channel delivery.
 *
 * The relationship is:
 *
 *     message
 *       |
 *       v
 *     channel payload type/value
 *       |
 *       v
 *     channel semantics
 *       |
 *       v
 *     runtime realization
 *
 * ============================================================================
 * NETWORKING CONTRACT
 * ============================================================================
 *
 * Networking remains downstream.
 *
 * A distributed message does NOT select:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     Ethernet;
 *     shared memory;
 *     vendor transport;
 *     future transport.
 *
 * The networking subsystem determines an appropriate realization after
 * semantic and capability analysis.
 *
 * ============================================================================
 * SERIALIZATION CONTRACT
 * ============================================================================
 *
 * A message schema is not a serialization format.
 *
 * This file does not define:
 *
 *     JSON;
 *     CBOR;
 *     protobuf;
 *     custom binary;
 *     wire layout;
 *     packet layout;
 *     ABI layout;
 *     byte order;
 *     memory offsets.
 *
 * Serialization belongs downstream to data/interoperability/networking
 * subsystems as appropriate.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar does not implement:
 *
 *     encryption;
 *     authentication;
 *     authorization;
 *     key management;
 *     identity;
 *     trust;
 *     signatures.
 *
 * Security requirements may be expressed through canonical attributes and
 * security-domain constructs.
 *
 * Their meaning is determined downstream.
 *
 * ============================================================================
 * OWNERSHIP / MEMORY CONTRACT
 * ============================================================================
 *
 * Constructing or referencing a message does not itself define:
 *
 *     copy;
 *     move;
 *     borrow;
 *     clone;
 *     allocation;
 *     deallocation;
 *     shared ownership;
 *     serialization ownership.
 *
 * These remain owned by the canonical type/ownership/effect systems.
 *
 * This is necessary so that the same message source can lower differently
 * depending on available target resources without changing program meaning.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Message fields and payloads may contain canonical quantum types or quantum
 * semantic values where permitted by the type and semantic systems.
 *
 * This file MUST NOT define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     LogicalQubitId;
 *     GateKind;
 *     quantum topology;
 *     calibration;
 *     pulse;
 *     QEC;
 *     ZQN.
 *
 * If a distributed message participates in quantum computation:
 *
 *     distributed message
 *          |
 *          v
 *     semantic quantum analysis
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This file must never create:
 *
 *     DistributedQuantumMessageIR
 *
 * or another competing quantum representation.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Messages may carry ordinary classical values:
 *
 *     scalar;
 *     vector;
 *     matrix;
 *     tensor;
 *     record;
 *     data;
 *     function results;
 *     symbolic values;
 *     AI/model values;
 *     scientific-computing values.
 *
 * The type system and semantic layer determine legality.
 *
 * This grammar imposes no machine-width assumptions.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Message values may participate in hardware/software co-design.
 *
 * This grammar does NOT define:
 *
 *     bus width;
 *     wire width;
 *     register width;
 *     physical address;
 *     device ID;
 *     FPGA resource count;
 *     ASIC topology;
 *     accelerator count.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Messages may carry canonical:
 *
 *     tensors;
 *     datasets;
 *     records;
 *     model values;
 *     inference results;
 *     training data;
 *     agent state;
 *     symbolic values.
 *
 * AI and data grammars remain responsible for their domain-specific syntax.
 *
 * This file provides only the message integration boundary.
 *
 * ============================================================================
 * DISTRIBUTED SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Distributed semantics may interpret a message as:
 *
 *     task input;
 *     task output;
 *     service request;
 *     service response;
 *     actor data;
 *     event;
 *     command;
 *     result;
 *     collective data;
 *     replicated state;
 *     migration data;
 *     checkpoint data;
 *     recovery data;
 *     quantum/classical result.
 *
 * This file intentionally does NOT encode those classifications.
 *
 * Classification belongs to semantic context.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Message syntax does not itself require a particular resource.
 *
 * A semantic analysis may determine requirements such as:
 *
 *     capability("distributed.communication")
 *
 *     capability("secure.communication")
 *
 *     capability("quantum.communication")
 *
 *     memory >= required_memory
 *
 *     network capability >= required_capability
 *
 * Such requirements are evaluated downstream.
 *
 * There is no grammar-level resource maximum.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * Message semantics may contribute information to:
 *
 *     classical IR;
 *     distributed execution metadata;
 *     networking semantic representation;
 *     data/schema representation;
 *     hardware representation;
 *     quantum::ir where quantum semantics participate.
 *
 * The grammar must never invent:
 *
 *     MessageIR;
 *     DistributedMessageIR;
 *     NetworkPacketIR;
 *     QuantumMessageIR
 *
 * as competing canonical representations merely because a message occurs in
 * a particular domain.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use message semantics for:
 *
 *     type checking;
 *     ownership analysis;
 *     effect analysis;
 *     communication optimization;
 *     serialization selection;
 *     placement;
 *     routing;
 *     scheduling;
 *     capability negotiation;
 *     resource allocation;
 *     target lowering.
 *
 * Those are downstream operations.
 *
 * This grammar provides only the source syntax required to express the
 * relevant semantic objects.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may realize messages using:
 *
 *     local memory;
 *     shared memory;
 *     IPC;
 *     actor mailboxes;
 *     channels;
 *     network transport;
 *     distributed services;
 *     hardware fabrics;
 *     quantum/classical communication mechanisms.
 *
 * The grammar does not select any of them.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     token stream;
 *     grammar version;
 *
 * parsing must produce the same syntactic structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     target hardware;
 *     node count;
 *     memory availability;
 *     network state;
 *     runtime state;
 *     scheduler state;
 *     wall-clock time;
 *     randomness;
 *     environment state.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     actions;
 *     semantic predicates;
 *     target discovery;
 *     network discovery;
 *     filesystem discovery;
 *     hardware discovery;
 *     runtime callbacks.
 *
 * Open-world message names are represented through canonical names.
 *
 * Repetition uses:
 *
 *     *
 *     +
 *
 * rather than finite alternatives.
 *
 * Parser resource protection must be implemented as explicit tooling/compiler
 * policy, not as artificial language semantics.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Diagnostics must distinguish:
 *
 * 1. Syntax error
 *
 *     malformed source structure.
 *
 * 2. Name/type error
 *
 *     syntactically valid but unresolved/invalid message reference.
 *
 * 3. Semantic error
 *
 *     invalid message/payload/type/ownership/effect semantics.
 *
 * 4. Capability/resource error
 *
 *     valid program but unavailable capability or insufficient resources.
 *
 * 5. Target error
 *
 *     valid semantic program but selected target cannot realize the required
 *     capability.
 *
 * Resource exhaustion must NOT be converted into a syntax error.
 *
 * ============================================================================
 * COMPATIBILITY WITH EXISTING messaging.g4
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/distributed/messaging.g4
 *
 * That file historically defines message declaration/value rules.
 *
 * It must NOT remain a second independent message-schema authority.
 *
 * The migration target is:
 *
 *     grammar/networking/messages.g4
 *                |
 *                v
 *     canonical message schema/value syntax
 *                |
 *                v
 *     grammar/distributed/messages.g4
 *                |
 *                v
 *     distributed message integration
 *
 * During migration, `messaging.g4` may remain as a compatibility adapter,
 * but its message-schema implementation should eventually delegate to the
 * canonical `Messages` grammar rather than duplicate:
 *
 *     messageDeclaration
 *     messageField
 *     messageValue
 *     messageTypeReference
 *
 * No source-level rename is required.
 *
 * ============================================================================
 * COMPATIBILITY WITH networking/messages.g4
 * ============================================================================
 *
 * `grammar/networking/messages.g4` remains the canonical owner of:
 *
 *     messageDeclaration
 *     messageField
 *     messageValue
 *     messageTypeReference
 *
 * This file must consume those rules rather than copy their implementation.
 *
 * If networking message syntax evolves, this file should remain stable unless
 * the distributed semantic boundary itself changes.
 *
 * This is deliberate decoupling.
 *
 * ============================================================================
 * COMPATIBILITY WITH communication.g4
 * ============================================================================
 *
 * `grammar/distributed/communication.g4` remains the owner of communication
 * operation syntax.
 *
 * Therefore this file does NOT define:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::scatter
 *     distributed::gather
 *     distributed::reduce
 *
 * Those remain representable through:
 *
 *     distributedCommunication
 *
 * A semantic communication operation may consume:
 *
 *     distributedMessageTypeReference
 *
 * or:
 *
 *     distributedMessageValue
 *
 * without this grammar owning the operation itself.
 *
 * ============================================================================
 * COMPATIBILITY WITH channels.g4
 * ============================================================================
 *
 * `grammar/distributed/channels.g4` remains the owner of channel syntax.
 *
 * A channel payload may semantically refer to:
 *
 *     messageTypeReference
 *
 * or another canonical type.
 *
 * This grammar must not redefine channel syntax.
 *
 * ============================================================================
 * COMPATIBILITY WITH concurrency
 * ============================================================================
 *
 * Generic concurrency channels remain owned by:
 *
 *     grammar/concurrency/
 *
 * Distributed message syntax does not replace generic concurrency.
 *
 * A message may travel through a generic channel, distributed channel, actor
 * mailbox, service boundary, or network realization depending on semantic
 * context.
 *
 * ============================================================================
 * COMPATIBILITY WITH networking
 * ============================================================================
 *
 * Networking owns physical/logical transport realization.
 *
 * This file remains transport-independent.
 *
 * ============================================================================
 * COMPATIBILITY WITH security
 * ============================================================================
 *
 * Security metadata must be expressed through canonical attributes/security
 * constructs rather than message-specific encryption syntax.
 *
 * ============================================================================
 * COMPATIBILITY WITH resources
 * ============================================================================
 *
 * Resource requirements belong to:
 *
 *     grammar/resources/
 *
 * This file only provides a source object that resource analysis may inspect.
 *
 * ============================================================================
 * COMPATIBILITY WITH execution
 * ============================================================================
 *
 * Execution semantics determine when/where/how a message is consumed.
 *
 * This grammar does not schedule message operations.
 *
 * ============================================================================
 * COMPATIBILITY WITH quantum
 * ============================================================================
 *
 * Quantum payloads remain semantic quantum constructs.
 *
 * They lower through:
 *
 *     quantum::ir
 *
 * where appropriate.
 *
 * No distributed-message-specific quantum IR is allowed.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     [ ] grammar name is DistributedMessages;
 *     [ ] filename is messages.g4;
 *     [ ] tokenVocab is ZamaniLexer;
 *     [ ] canonical Messages grammar is imported;
 *     [ ] message schema rules are not redefined;
 *     [ ] identifier is not redefined;
 *     [ ] qualifiedName is not redefined;
 *     [ ] expression is not redefined;
 *     [ ] typeExpression is not redefined;
 *     [ ] communication operations are not redefined;
 *     [ ] channel syntax is not redefined;
 *     [ ] no distributed-message lexer tokens are introduced;
 *     [ ] no hardware limits exist;
 *     [ ] no distributed resource limits exist;
 *     [ ] no physical topology is encoded;
 *     [ ] no transport protocol is encoded;
 *     [ ] no runtime actions exist;
 *     [ ] no semantic predicates exist;
 *     [ ] no unsafe Rust dependency exists;
 *     [ ] canonical quantum::ir remains the quantum boundary;
 *     [ ] source spans remain traceable;
 *     [ ] deterministic parsing is preserved.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 * Canonical message declaration:
 *
 *     message UserCreated {
 *         id: Identifier;
 *         name: String;
 *     }
 *
 * Distributed declaration adapter:
 *
 *     @distributed
 *     message UserCreated {
 *         id: Identifier;
 *     }
 *
 * Canonical message value:
 *
 *     UserCreated(id, name)
 *
 * Qualified message value:
 *
 *     events::UserCreated(id, name)
 *
 * Message reference:
 *
 *     events::UserCreated
 *
 * Payload:
 *
 *     result
 *
 * Computed payload:
 *
 *     compute(input)
 *
 * Indexed payload:
 *
 *     tensor[index]
 *
 * Quantum/classical payload:
 *
 *     measurement
 *
 * Generic message:
 *
 *     message Envelope<T> {
 *         payload: T;
 *     }
 *
 * Large logical message types:
 *
 *     message DistributedResult<T, U, V> {
 *         primary: T;
 *         secondary: U;
 *         metadata: V;
 *     }
 *
 * The grammar must not require any fixed number of fields.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * These should fail structurally through the canonical message grammar or
 * parent composition:
 *
 *     message;
 *
 *     message Event {
 *
 *     message Event {
 *         value:
 *     }
 *
 *     Event(
 *
 *     Event(value
 *
 *     distributed::message;
 *
 * where such forms are not valid under the canonical surrounding grammar.
 *
 * The exact diagnostic category must remain a parser/semantic distinction.
 *
 * ============================================================================
 * NON-OWNERSHIP TESTS
 * ============================================================================
 *
 * The following concepts must NOT require rules in this file:
 *
 *     distributed::send(channel, value, destination);
 *
 *     distributed::receive(channel);
 *
 *     distributed::broadcast(value, group);
 *
 *     distributed::channel data: Message;
 *
 *     TCP;
 *
 *     UDP;
 *
 *     QUIC;
 *
 *     MPI;
 *
 *     RDMA;
 *
 *     physical_node;
 *
 *     physical_qubit;
 *
 *     gpu0;
 *
 *     qpu0;
 *
 * Their syntax, where legal, is owned elsewhere or represented by canonical
 * names/expressions.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     zero distributed message fragments;
 *     one message;
 *     many messages;
 *     one field;
 *     many fields;
 *     one argument;
 *     many arguments;
 *     deeply qualified message names;
 *     deeply nested expression payloads;
 *     large generic parameter lists;
 *     large payload lists;
 *     nested distributed scopes;
 *     message declarations across modules;
 *     message references across namespaces.
 *
 * The tests must be bounded by actual test-resource policy, not grammar
 * semantics.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Verify that there is no language-level fixed limit on:
 *
 *     message count;
 *     field count;
 *     argument count;
 *     payload count;
 *     qualified-name depth;
 *     generic parameter count;
 *     distributed context depth.
 *
 * The test harness may impose explicit execution budgets.
 *
 * Those budgets must not be represented as grammar constants.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     parser configuration;
 *
 * must produce equivalent parse structures.
 *
 * Parsing must not depend on:
 *
 *     hardware;
 *     network state;
 *     deployment;
 *     node availability;
 *     runtime state;
 *     randomness.
 *
 * ============================================================================
 * PORTABILITY TESTS
 * ============================================================================
 *
 * The same source message construct must remain structurally valid when the
 * target realization changes among:
 *
 *     embedded;
 *     single CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     HPC;
 *     cluster;
 *     cloud;
 *     distributed system;
 *     heterogeneous system;
 *     future computational substrate.
 *
 * The target may lower the message differently without changing source
 * semantics.
 *
 * ============================================================================
 * AST TESTS
 * ============================================================================
 *
 * AST conformance must verify:
 *
 *     message declaration ordering;
 *     field ordering;
 *     message value argument ordering;
 *     qualified-name ordering;
 *     payload expression structure;
 *     attribute preservation;
 *     source spans.
 *
 * The AST must remain domain-neutral.
 *
 * ============================================================================
 * SEMANTIC TESTS
 * ============================================================================
 *
 * Semantic conformance must verify:
 *
 *     valid message type resolution;
 *     invalid message type resolution;
 *     field/type compatibility;
 *     default-value validity;
 *     ownership rules;
 *     effect rules;
 *     capability rules;
 *     resource requirements;
 *     distributed communication validity;
 *     security requirements;
 *     quantum/classical compatibility.
 *
 * ============================================================================
 * IR TESTS
 * ============================================================================
 *
 * Message grammar tests must verify that message syntax has a defined
 * downstream semantic destination.
 *
 * The grammar itself MUST NOT require a message-specific IR.
 *
 * Quantum-bearing semantics MUST continue through:
 *
 *     quantum::ir
 *
 * rather than a distributed-message quantum IR.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * Generated parser consumers must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * This grammar must not require:
 *
 *     unsafe blocks;
 *     unsafe functions;
 *     unsafe traits;
 *     unsafe FFI;
 *
 * merely to parse distributed messages.
 *
 * Repository-wide unsafe policy remains enforced by Rust build/CI validation.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing this grammar must never:
 *
 *     execute a message;
 *     send a message;
 *     receive a message;
 *     access a network;
 *     access a filesystem;
 *     inspect hardware;
 *     access credentials;
 *     invoke a transport;
 *     invoke a scheduler;
 *     invoke a runtime.
 *
 * Message names and expressions are parsed as syntax only.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This grammar uses only structural delegation and wrappers.
 *
 * It does not:
 *
 *     perform semantic lookups;
 *     perform network queries;
 *     perform hardware queries;
 *     perform filesystem queries;
 *     perform runtime callbacks.
 *
 * This is intentionally a thin integration boundary.
 *
 * ============================================================================
 * INTEGRATION PLAN
 * ============================================================================
 *
 * The file is independently complete as the distributed message integration
 * contract.
 *
 * Repository convergence should happen as follows.
 *
 * --------------------------------------------------------------------------
 * A. grammar/networking/messages.g4
 * --------------------------------------------------------------------------
 *
 * Remains the canonical message-schema authority.
 *
 * No change is required merely because this file exists.
 *
 * --------------------------------------------------------------------------
 * B. grammar/distributed/messaging.g4
 * --------------------------------------------------------------------------
 *
 * Existing message-schema rules should be migrated away from duplicate
 * ownership.
 *
 * The preferred compatibility direction is:
 *
 *     messaging.g4
 *          |
 *          v
 *     DistributedMessages
 *          |
 *          v
 *     networking/Messages
 *
 * It must not remain a second independent implementation of:
 *
 *     messageDeclaration;
 *     messageField;
 *     messageValue;
 *     messageTypeReference.
 *
 * --------------------------------------------------------------------------
 * C. grammar/distributed/distributed.g4
 * --------------------------------------------------------------------------
 *
 * The parent distributed composition grammar should eventually import:
 *
 *     DistributedMessages
 *
 * and use:
 *
 *     distributedMessageConstruct
 *
 * as the distributed message boundary.
 *
 * It should not copy message-schema rules into distributed.g4.
 *
 * --------------------------------------------------------------------------
 * D. grammar/distributed/communication.g4
 * --------------------------------------------------------------------------
 *
 * Remains the owner of:
 *
 *     distributedCommunication
 *     communicationOperation
 *
 * Communication operations may semantically consume message references or
 * values from this grammar.
 *
 * This file does not import its parent communication grammar, avoiding cycles.
 *
 * --------------------------------------------------------------------------
 * E. grammar/distributed/channels.g4
 * --------------------------------------------------------------------------
 *
 * Channels may consume message type references through semantic integration.
 *
 * Channel declaration syntax remains owned by channels.g4.
 *
 * --------------------------------------------------------------------------
 * F. grammar/networking/
 * --------------------------------------------------------------------------
 *
 * Networking remains responsible for physical/network realization.
 *
 * No transport-specific rule belongs here.
 *
 * --------------------------------------------------------------------------
 * G. grammar/resources/
 * --------------------------------------------------------------------------
 *
 * Resource requirements remain owned by the universal resource grammar.
 *
 * This file exposes semantic message objects to resource analysis but does
 * not define resource capacity.
 *
 * --------------------------------------------------------------------------
 * H. grammar/security/
 * --------------------------------------------------------------------------
 *
 * Security semantics consume canonical attributes/semantic contracts.
 *
 * This file does not create message-specific cryptography syntax.
 *
 * --------------------------------------------------------------------------
 * I. grammar/quantum/
 * --------------------------------------------------------------------------
 *
 * Quantum-bearing messages participate in semantic quantum lowering.
 *
 * The canonical boundary remains:
 *
 *     quantum::ir
 *
 * No distributed-message quantum IR is introduced.
 *
 * --------------------------------------------------------------------------
 * J. grammar/tests/
 * --------------------------------------------------------------------------
 *
 * Add distributed-message conformance tests covering:
 *
 *     lexical;
 *     syntax;
 *     AST;
 *     semantic;
 *     IR;
 *     portability;
 *     scalability;
 *     determinism;
 *     compatibility;
 *     negative cases.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The intended dependency direction is:
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
 *     ZamaniParser
 *
 * This file MUST NOT import:
 *
 *     Distributed
 *
 * because that would create a dependency cycle.
 *
 * It also MUST NOT import:
 *
 *     runtime;
 *     compiler;
 *     quantum IR;
 *     HAL;
 *     scheduler;
 *     router.
 *
 * ============================================================================
 * NO SECOND AUTHORITY
 * ============================================================================
 *
 * The final architecture must contain exactly one owner for each concern:
 *
 * MESSAGE SCHEMA
 *     grammar/networking/messages.g4
 *
 * DISTRIBUTED MESSAGE CONTEXT
 *     grammar/distributed/messages.g4
 *
 * COMMUNICATION OPERATION
 *     grammar/distributed/communication.g4
 *
 * DISTRIBUTED CHANNEL
 *     grammar/distributed/channels.g4
 *
 * NETWORK TRANSPORT
 *     grammar/networking/
 *
 * RESOURCE/CAPABILITY
 *     grammar/resources/
 *
 * SECURITY
 *     grammar/security/
 *
 * QUANTUM SEMANTICS
 *     quantum::ir
 *
 * TARGET REALIZATION
 *     compiler / HAL / runtime
 *
 * No concern should be duplicated merely because multiple domains use it.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] filename is messages.g4;
 *     [x] grammar name is DistributedMessages;
 *     [x] canonical ZamaniLexer is used;
 *     [x] canonical Messages grammar is imported;
 *     [x] message schema is not duplicated;
 *     [x] message values are not duplicated;
 *     [x] message type syntax is not duplicated;
 *     [x] identifiers are not duplicated;
 *     [x] qualified names are not duplicated;
 *     [x] expressions are not duplicated;
 *     [x] types are not duplicated;
 *     [x] communication operations are not duplicated;
 *     [x] channel syntax is not duplicated;
 *     [x] networking transport is not duplicated;
 *     [x] no new lexer keyword is required;
 *     [x] no finite distributed resource limit exists;
 *     [x] no fixed message count exists;
 *     [x] no fixed payload count exists;
 *     [x] no fixed field count exists;
 *     [x] no fixed node count exists;
 *     [x] no fixed device count exists;
 *     [x] no hardware topology exists;
 *     [x] no physical placement exists;
 *     [x] no serialization format exists;
 *     [x] no network transport exists;
 *     [x] no runtime behavior exists;
 *     [x] no semantic predicates exist;
 *     [x] no Rust actions exist;
 *     [x] no unsafe Rust requirement exists;
 *     [x] source spans remain representable;
 *     [x] deterministic parsing is preserved;
 *     [x] AST mapping is defined;
 *     [x] semantic mapping is defined;
 *     [x] IR boundary is defined;
 *     [x] quantum::ir remains canonical;
 *     [x] compiler integration is defined;
 *     [x] runtime integration is defined;
 *     [x] cross-domain integration is defined;
 *     [x] compatibility migration is defined;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] determinism tests are defined;
 *     [x] portability tests are defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How does a message participate in distributed Zamani syntax?"
 *
 * It does NOT answer:
 *
 *     "How is the message transported?"
 *
 *     "Where is the message stored?"
 *
 *     "Which machine handles it?"
 *
 *     "Which network handles it?"
 *
 *     "How is it serialized?"
 *
 *     "How is it scheduled?"
 *
 *     "How is it routed?"
 *
 *     "How is it replicated?"
 *
 *     "How is it encrypted?"
 *
 *     "How is it executed?"
 *
 * Those answers belong to downstream semantic/compiler/runtime systems.
 *
 * The final architecture is:
 *
 *     MESSAGE SCHEMA
 *          |
 *          v
 *     DISTRIBUTED MESSAGE CONTEXT
 *          |
 *          v
 *     COMMUNICATION INTENT
 *          |
 *          v
 *     SEMANTIC ANALYSIS
 *          |
 *          +--> types
 *          +--> effects
 *          +--> ownership
 *          +--> capabilities
 *          +--> resources
 *          +--> security
 *          +--> distributed semantics
 *          +--> quantum semantics where applicable
 *          |
 *          v
 *     CANONICAL IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed execution metadata
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * while ensuring that distributed messaging remains:
 *
 *     open-world;
 *     target-independent;
 *     resource-independent;
 *     deterministic;
 *     source-span traceable;
 *     domain-neutral at the AST boundary;
 *     compatible with classical computation;
 *     compatible with quantum computation;
 *     compatible with HDL/hardware co-design;
 *     compatible with AI/data computation;
 *     compatible with future computational substrates.
 *
 * ============================================================================
 * END OF grammar/distributed/messages.g4
 * ============================================================================
 */