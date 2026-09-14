/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/communication.g4
 *
 * Grammar:
 *     Communication
 *
 * Status:
 *     Production distributed-communication grammar.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No mutable compiler-global state.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL COMMUNICATION SYNTAX for Zamani's
 * distributed-computing model.
 *
 * It expresses communication INTENT.
 *
 * It does not implement communication.
 *
 * It does not select or implement:
 *
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - RDMA;
 *     - InfiniBand;
 *     - MPI;
 *     - shared-memory transport;
 *     - message brokers;
 *     - cloud transports;
 *     - vendor transports;
 *     - quantum links;
 *     - optical links;
 *     - wireless links;
 *     - future transports.
 *
 * Those are downstream networking, hardware, runtime, scheduling, routing,
 * deployment, and resource-realization concerns.
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
 *     ZamaniParser / Distributed parser
 *          |
 *          v
 *     Communication AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> communication semantic validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed communication metadata
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / placement
 *          |
 *          v
 *     networking / hardware realization
 *          |
 *          v
 *     runtime
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NEVER create, own, or redefine quantum::ir.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - communication declarations;
 *     - communication operations;
 *     - communication invocation syntax;
 *     - communication endpoints as symbolic references;
 *     - communication payload syntax;
 *     - communication argument syntax;
 *     - communication attributes;
 *     - communication options;
 *     - communication dependencies;
 *     - communication ordering intent;
 *     - communication stream syntax;
 *     - communication request/reply syntax;
 *     - communication publication/subscription syntax;
 *     - communication transfer syntax;
 *     - communication synchronization intent;
 *     - communication scope syntax;
 *     - communication extension syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - lexer tokens;
 *     - distributed nodes;
 *     - distributed services;
 *     - distributed placement;
 *     - distributed replication;
 *     - distributed consistency;
 *     - network protocols;
 *     - network addresses;
 *     - sockets;
 *     - ports;
 *     - routing;
 *     - scheduling;
 *     - hardware;
 *     - resource discovery;
 *     - resource allocation;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Communication must remain extensible.
 *
 * This grammar therefore does NOT create a closed list such as:
 *
 *     SEND
 *     RECEIVE
 *     BROADCAST
 *     MULTICAST
 *     GATHER
 *     SCATTER
 *     REDUCE
 *     PUBLISH
 *     SUBSCRIBE
 *     RPC
 *     RDMA
 *
 * as mandatory lexer keywords.
 *
 * Communication operation identity is represented through canonical names.
 *
 * Examples of semantic operation names include:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     distributed::multicast
 *     distributed::gather
 *     distributed::scatter
 *     distributed::reduce
 *     distributed::publish
 *     distributed::subscribe
 *     distributed::request
 *     distributed::reply
 *     distributed::stream
 *     distributed::transfer
 *     distributed::sync
 *
 * Future operations can therefore be introduced without changing this
 * grammar merely because a new communication abstraction was invented.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Communication syntax describes WHAT communication is required.
 *
 * It does not permanently encode WHERE or HOW communication occurs.
 *
 * Therefore this grammar must not require:
 *
 *     - a particular host;
 *     - a particular machine;
 *     - a particular network;
 *     - a particular provider;
 *     - a particular transport;
 *     - a particular interface;
 *     - a particular address;
 *     - a particular port;
 *     - a fixed number of endpoints;
 *     - a fixed number of nodes;
 *     - a fixed number of channels;
 *     - a fixed bandwidth;
 *     - a fixed latency;
 *     - a fixed topology.
 *
 * These may exist as explicit program data when genuinely part of program
 * semantics, but they must never be implicit grammar-level machine limits.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO language-level finite limits on:
 *
 *     - communication declarations;
 *     - communication operations;
 *     - endpoints;
 *     - arguments;
 *     - payload expressions;
 *     - communication clauses;
 *     - dependencies;
 *     - channels;
 *     - streams;
 *     - requests;
 *     - replies;
 *     - subscriptions;
 *     - publications;
 *     - nested communication scopes;
 *     - qualified-name depth.
 *
 * Lists use:
 *
 *     *
 *     +
 *
 * rather than finite alternatives.
 *
 * Actual limits may arise from:
 *
 *     - parser memory;
 *     - compiler memory;
 *     - runtime memory;
 *     - operating-system limits;
 *     - available network resources;
 *     - hardware capabilities;
 *     - deployment policy.
 *
 * Those are NOT grammar limits.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical dependency graph:
 *
 *     ZamaniLexer
 *          |
 *          +--> Names
 *          |
 *          +--> Expressions
 *          |
 *          v
 *     Communication
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *     nameReference
 *
 * Expressions owns:
 *
 *     expression
 *     expressionList
 *
 * This grammar MUST NOT duplicate those rules.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Canonical options:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Canonical imports:
 *
 *     Names
 *     Expressions
 *
 * The ANTLR build must make the canonical grammar source/import paths
 * available to the generator.
 *
 * This grammar contains no target-language-specific actions.
 *
 * Therefore generated Rust parser code remains responsible for satisfying
 * the repository's Rust 1.97 / Rust 1.97.1 and `#![forbid(unsafe_code)]`
 * requirements.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser establishes structural syntax.
 *
 * Semantic analysis establishes:
 *
 *     - whether an operation is a communication operation;
 *     - whether endpoints are valid;
 *     - whether source and destination types are compatible;
 *     - whether a payload is serializable;
 *     - whether a communication effect is permitted;
 *     - whether the requested capability exists;
 *     - whether resource requirements can be satisfied;
 *     - whether security policy permits the communication;
 *     - whether communication ordering is satisfiable;
 *     - whether the operation can be realized on the selected target.
 *
 * This grammar does none of those things.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * Communication syntax is transport-neutral.
 *
 * For example, a source program may express:
 *
 *     distributed::send(...)
 *
 * without implying:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     shared memory
 *     vendor transport.
 *
 * The networking subsystem chooses or negotiates an appropriate realization.
 *
 * ============================================================================
 * ENDPOINT BOUNDARY
 * ============================================================================
 *
 * Endpoints are symbolic semantic references.
 *
 * This grammar does not define:
 *
 *     IP addresses;
 *     MAC addresses;
 *     sockets;
 *     ports;
 *     physical interfaces;
 *     machine identifiers;
 *     device identifiers.
 *
 * If such information is explicitly represented in source, it is parsed as
 * ordinary language data and must be classified downstream as either:
 *
 *     - semantic data;
 *     - deployment configuration;
 *     - target constraint;
 *     - implementation detail;
 *     - non-portable requirement.
 *
 * The grammar itself must not force any of those interpretations.
 *
 * ============================================================================
 * PAYLOAD BOUNDARY
 * ============================================================================
 *
 * Payloads are expressions.
 *
 * This allows communication to work with:
 *
 *     classical values;
 *     structured values;
 *     generic values;
 *     streams;
 *     tensors;
 *     scientific data;
 *     AI data;
 *     hardware data;
 *     quantum measurement results;
 *     future data abstractions.
 *
 * Serialization is NOT owned by this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Communication may transport or coordinate quantum-related semantic values,
 * but this grammar does not define quantum representation.
 *
 * Examples include:
 *
 *     measurement results;
 *     classical control information;
 *     logical-qubit coordination metadata;
 *     distributed quantum execution requests.
 *
 * Quantum semantics remain owned by the quantum subsystem and ultimately
 * `quantum::ir`.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     pulse semantics
 *     calibration
 *     QEC
 *     ZQN.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Communication may coordinate hardware or HDL computations.
 *
 * This grammar does not define:
 *
 *     wires;
 *     clocks;
 *     pins;
 *     physical buses;
 *     FPGA resources;
 *     ASIC cells;
 *     physical interfaces.
 *
 * Those belong to HDL/hardware grammar and downstream target systems.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Communication may produce effects.
 *
 * The grammar does not redefine the effect system.
 *
 * Downstream semantic analysis should map communication constructs to the
 * canonical distributed/network effects already owned by:
 *
 *     grammar/effects/
 *
 * and the corresponding semantic/compiler effect model.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Communication may require:
 *
 *     bandwidth;
 *     latency bounds;
 *     reliability;
 *     ordering;
 *     capacity;
 *     connectivity;
 *     security;
 *     locality;
 *     energy;
 *     performance.
 *
 * This grammar does not define those resource semantics.
 *
 * It merely permits expressions/attributes through which the semantic layer
 * can represent them.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Communication may interact with:
 *
 *     identity;
 *     authentication;
 *     authorization;
 *     confidentiality;
 *     integrity;
 *     privacy;
 *     trust;
 *     cryptographic requirements.
 *
 * Security semantics remain owned by `grammar/security/` and downstream
 * security analysis.
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
 *     - no network calls;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no randomness;
 *     - no mutable global state.
 *
 * Parsing is therefore a deterministic function of the token stream.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * AST construction must preserve:
 *
 *     - source spans;
 *     - operation names;
 *     - endpoint ordering;
 *     - argument ordering;
 *     - payload expression structure;
 *     - clause ordering;
 *     - nesting;
 *     - optional trailing separators;
 *     - declaration ordering.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 */

parser grammar Communication;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the stable rule consumed by Distributed.
 */

communicationDeclaration
    : communicationStatement
    | communicationBlock
    ;


/* ============================================================================
 * 2. COMMUNICATION STATEMENT
 * ============================================================================
 *
 * General form:
 *
 *     <operation>(...)
 *
 * or:
 *
 *     <operation> <argument> ...;
 *
 * The operation is intentionally represented by a qualified name rather than
 * a closed keyword inventory.
 */

communicationStatement
    : communicationInvocation SEMICOLON
    | communicationOperationStatement SEMICOLON
    ;


/* ============================================================================
 * 3. INVOCATION FORM
 * ============================================================================
 *
 * Examples:
 *
 *     distributed::send(value)
 *     distributed::receive(channel)
 *     distributed::broadcast(value, group)
 *     distributed::publish(topic, value)
 *
 * The semantic layer determines the meaning of the operation.
 */

communicationInvocation
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 4. OPERATION STATEMENT
 * ============================================================================
 *
 * This form permits communication constructs whose syntax is not naturally
 * represented as a function-style invocation.
 *
 * Examples may include future language extensions with named operands.
 */

communicationOperationStatement
    : qualifiedName
      communicationOperand+
    ;


/* ============================================================================
 * 5. OPERANDS
 * ============================================================================
 */

communicationOperand
    : expression
    | qualifiedName
    ;


/* ============================================================================
 * 6. ARGUMENT LIST
 * ============================================================================
 *
 * Unbounded by language design.
 */

communicationArgumentList
    : communicationArgument
      (COMMA communicationArgument)*
      COMMA?
    ;


/* ============================================================================
 * 7. ARGUMENT
 * ============================================================================
 *
 * An argument may be:
 *
 *     - an expression;
 *     - a symbolic reference.
 *
 * The expression grammar remains authoritative for expression syntax.
 */

communicationArgument
    : expression
    | qualifiedName
    ;


/* ============================================================================
 * 8. COMMUNICATION BLOCK
 * ============================================================================
 *
 * A block provides a structured communication scope.
 *
 * Example semantic forms:
 *
 *     distributed::communication channel {
 *         endpoint ...
 *         payload ...
 *         policy ...
 *     }
 *
 * The actual semantic classification remains downstream.
 */

communicationBlock
    : qualifiedName
      qualifiedName
      LBRACE
      communicationMember*
      RBRACE
    ;


/* ============================================================================
 * 9. COMMUNICATION MEMBERS
 * ============================================================================
 *
 * Members are structurally classified rather than closed over a finite
 * vocabulary.
 */

communicationMember
    : communicationEndpointClause
    | communicationPayloadClause
    | communicationSourceClause
    | communicationDestinationClause
    | communicationPeerClause
    | communicationOperationClause
    | communicationOrderingClause
    | communicationDeliveryClause
    | communicationReliabilityClause
    | communicationSecurityClause
    | communicationRequirementClause
    | communicationConstraintClause
    | communicationPreferenceClause
    | communicationDependencyClause
    | communicationAttributeClause
    | communicationExtensionClause
    ;


/* ============================================================================
 * 10. ENDPOINT
 * ============================================================================
 */

communicationEndpointClause
    : qualifiedName
      communicationReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 11. SOURCE
 * ============================================================================
 */

communicationSourceClause
    : qualifiedName
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 12. DESTINATION
 * ============================================================================
 */

communicationDestinationClause
    : qualifiedName
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 13. PEER
 * ============================================================================
 */

communicationPeerClause
    : qualifiedName
      communicationReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 14. PAYLOAD
 * ============================================================================
 */

communicationPayloadClause
    : qualifiedName
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 15. OPERATION
 * ============================================================================
 */

communicationOperationClause
    : qualifiedName
      communicationInvocation?
      SEMICOLON
    ;


/* ============================================================================
 * 16. ORDERING
 * ============================================================================
 *
 * Ordering is represented as an expression/reference property.
 *
 * The scheduler remains responsible for realization.
 */

communicationOrderingClause
    : qualifiedName
      expression?
      SEMICOLON
    ;


/* ============================================================================
 * 17. DELIVERY
 * ============================================================================
 *
 * Delivery semantics remain abstract.
 *
 * They are not transport implementations.
 */

communicationDeliveryClause
    : qualifiedName
      expression?
      SEMICOLON
    ;


/* ============================================================================
 * 18. RELIABILITY
 * ============================================================================
 */

communicationReliabilityClause
    : qualifiedName
      expression?
      SEMICOLON
    ;


/* ============================================================================
 * 19. SECURITY
 * ============================================================================
 */

communicationSecurityClause
    : qualifiedName
      expression?
      SEMICOLON
    ;


/* ============================================================================
 * 20. REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe what a valid realization must provide.
 *
 * They do not allocate resources.
 */

communicationRequirementClause
    : qualifiedName
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. CONSTRAINTS
 * ============================================================================
 *
 * Constraints describe admissible realizations.
 *
 * They do not select a physical target.
 */

communicationConstraintClause
    : qualifiedName
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 22. PREFERENCES
 * ============================================================================
 *
 * Preferences guide realization without becoming mandatory semantic
 * requirements.
 */

communicationPreferenceClause
    : qualifiedName
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 23. DEPENDENCIES
 * ============================================================================
 *
 * Dependencies describe logical communication ordering/dependency intent.
 *
 * Scheduling remains downstream.
 */

communicationDependencyClause
    : qualifiedName
      communicationReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 24. ATTRIBUTES
 * ============================================================================
 *
 * Attributes provide structured extensibility without requiring lexer changes.
 */

communicationAttributeClause
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 25. EXTENSIONS
 * ============================================================================
 *
 * Future communication-domain constructs may be represented through qualified
 * names and expressions.
 *
 * Semantic validation determines whether an extension is registered,
 * supported, experimental, deprecated, or unknown.
 */

communicationExtensionClause
    : qualifiedName
      communicationExtensionValue?
      SEMICOLON
    ;

communicationExtensionValue
    : expression
    | communicationArgumentList
    | qualifiedName
    ;


/* ============================================================================
 * 26. REFERENCE LIST
 * ============================================================================
 *
 * No finite endpoint/channel count is imposed.
 */

communicationReferenceList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;


/* ============================================================================
 * 27. NAMED COMMUNICATION OPERATION
 * ============================================================================
 *
 * This rule exists as a semantic integration point for downstream AST
 * construction.
 *
 * It intentionally contains no closed-world operation list.
 */

communicationOperation
    : qualifiedName
    ;


/* ============================================================================
 * 28. SEND
 * ============================================================================
 *
 * These named rules are compatibility/convenience entry points.
 *
 * They do not define a closed set of communication operations.
 */

communicationSend
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 29. RECEIVE
 * ============================================================================
 */

communicationReceive
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 30. BROADCAST / MULTICAST FAMILY
 * ============================================================================
 *
 * The syntax remains operation-name driven.
 */

communicationGroupOperation
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 31. STREAM
 * ============================================================================
 */

communicationStream
    : qualifiedName
      qualifiedName?
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 32. REQUEST / RESPONSE
 * ============================================================================
 */

communicationRequest
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;

communicationResponse
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 33. PUBLICATION / SUBSCRIPTION
 * ============================================================================
 */

communicationPublication
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;

communicationSubscription
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 34. SYNCHRONIZATION
 * ============================================================================
 */

communicationSynchronization
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 35. DATA TRANSFER
 * ============================================================================
 */

communicationTransfer
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * 36. OPTIONAL COMMUNICATION EXPRESSION
 * ============================================================================
 */

optionalCommunicationExpression
    : expression?
    ;


/* ============================================================================
 * 37. OPTIONAL COMMUNICATION REFERENCE
 * ============================================================================
 */

optionalCommunicationReference
    : qualifiedName?
    ;


/* ============================================================================
 * 38. COMMUNICATION EXPRESSION LIST
 * ============================================================================
 */

communicationExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 39. COMMUNICATION REFERENCE
 * ============================================================================
 */

communicationReference
    : qualifiedName
    ;


/* ============================================================================
 * 40. COMMUNICATION QUALIFIED REFERENCE
 * ============================================================================
 */

communicationQualifiedReference
    : qualifiedName
    ;


/* ============================================================================
 * 41. COMMUNICATION FUTURE RESULT
 * ============================================================================
 *
 * A future is represented syntactically as an invocation/result relation.
 *
 * The runtime determines the actual future implementation.
 */

communicationFuture
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 42. COMMUNICATION DEPENDENCY
 * ============================================================================
 */

communicationDependency
    : qualifiedName
      communicationReferenceList
      SEMICOLON
    ;


/* ============================================================================
 * 43. COMMUNICATION DATA FLOW
 * ============================================================================
 */

communicationDataFlow
    : qualifiedName
      qualifiedName
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 44. COMMUNICATION PIPELINE
 * ============================================================================
 *
 * A pipeline is logical communication/data-flow structure.
 *
 * It does not prescribe a physical topology.
 */

communicationPipeline
    : qualifiedName
      qualifiedName
      LBRACE
      communicationPipelineStage*
      RBRACE
    ;

communicationPipelineStage
    : qualifiedName
      communicationReferenceList?
      SEMICOLON
    ;


/* ============================================================================
 * 45. COMMUNICATION DOMAIN REFERENCE
 * ============================================================================
 */

communicationDomainReference
    : qualifiedName
    ;


/* ============================================================================
 * 46. COMMUNICATION NAMESPACE REFERENCE
 * ============================================================================
 */

communicationNamespaceReference
    : qualifiedName
    ;


/* ============================================================================
 * 47. EXTENSION-SAFE COMMUNICATION INVOCATION
 * ============================================================================
 *
 * This is the preferred generic extension point.
 *
 * New communication semantics should normally be represented here instead
 * of requiring a grammar modification.
 */

communicationExtensionInvocation
    : qualifiedName
      LPAREN
      communicationArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 48. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are deliberately no grammar productions resembling:
 *
 *     endpoint1
 *     endpoint2
 *     node1
 *     node2
 *     channel1
 *     channel2
 *
 * and no bounded repetitions such as:
 *
 *     item item?
 *     item item item?
 *
 * where those forms would impose artificial machine limits.
 *
 * All scalable collections use unbounded grammar repetition.
 *
 * ============================================================================
 * 49. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_NODES
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_SERVICES
 *     MAX_CONNECTIONS
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_QUBITS
 *     fixed host
 *     fixed IP
 *     fixed port
 *     fixed topology
 *     fixed provider
 *     fixed transport.
 *
 * Any future addition introducing such a limit must undergo the grammar
 * hard-coding audit before acceptance.
 *
 * ============================================================================
 * 50. ERROR HANDLING CONTRACT
 * ============================================================================
 *
 * Syntax errors are reported by the generated ANTLR parser.
 *
 * Semantic errors belong downstream.
 *
 * This grammar must not:
 *
 *     - emit diagnostics;
 *     - print to stdout/stderr;
 *     - silently recover invalid semantics;
 *     - convert semantic errors into comments;
 *     - invoke runtime services.
 *
 * ============================================================================
 * 51. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should expose a canonical communication node containing,
 * at minimum, the semantic equivalents of:
 *
 *     operation
 *     arguments
 *     endpoints
 *     payload
 *     clauses
 *     attributes
 *     source span
 *
 * The grammar itself does not define the AST type.
 *
 * ============================================================================
 * 52. COMPILER CONTRACT
 * ============================================================================
 *
 * Downstream compilation may lower communication into:
 *
 *     classical operations;
 *     distributed operations;
 *     network effects;
 *     resource requirements;
 *     scheduling dependencies;
 *     routing requirements;
 *     hardware communication operations;
 *     quantum/classical coordination metadata.
 *
 * The lowering must preserve source semantics.
 *
 * ============================================================================
 * 53. QUANTUM COMPILER CONTRACT
 * ============================================================================
 *
 * If communication participates in a quantum program:
 *
 *     source
 *       ->
 *     semantic communication model
 *       ->
 *     quantum/classical semantic integration
 *       ->
 *     quantum::ir
 *
 * The grammar MUST NOT directly construct quantum::ir.
 *
 * ============================================================================
 * 54. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime communication implementation is completely downstream.
 *
 * The runtime may resolve:
 *
 *     endpoints;
 *     transport;
 *     routes;
 *     scheduling;
 *     retries;
 *     serialization;
 *     security;
 *     resource allocation;
 *     hardware interfaces.
 *
 * None of those decisions are embedded here.
 *
 * ============================================================================
 * 55. TEST CONTRACT
 * ============================================================================
 *
 * Required tests:
 *
 * POSITIVE:
 *
 *     - minimal communication invocation;
 *     - qualified operation;
 *     - arbitrary argument count;
 *     - arbitrary endpoint count;
 *     - payload expression;
 *     - nested communication block;
 *     - stream;
 *     - request/reply;
 *     - publish/subscribe;
 *     - synchronization;
 *     - communication pipeline;
 *     - future operation;
 *     - extension operation.
 *
 * NEGATIVE:
 *
 *     - missing operation;
 *     - missing closing parenthesis;
 *     - malformed argument separator;
 *     - malformed endpoint list;
 *     - missing semicolon;
 *     - malformed block;
 *     - incomplete qualified name.
 *
 * BOUNDARY:
 *
 *     - zero arguments where permitted;
 *     - one argument;
 *     - many arguments;
 *     - one endpoint;
 *     - many endpoints;
 *     - deeply nested scopes;
 *     - long qualified names;
 *     - very large communication blocks.
 *
 * CROSS-DOMAIN:
 *
 *     - classical + communication;
 *     - quantum + communication;
 *     - quantum + classical + communication;
 *     - HDL + communication;
 *     - hardware + communication;
 *     - AI + communication;
 *     - distributed + security + communication;
 *     - distributed + resource + communication.
 *
 * SCALABILITY:
 *
 *     - no fixed endpoint count;
 *     - no fixed operation count;
 *     - no fixed channel count;
 *     - no fixed node count;
 *     - no fixed payload count;
 *     - no fixed topology.
 *
 * DETERMINISM:
 *
 *     identical token streams produce identical parse structures.
 *
 * ROUND-TRIP:
 *
 *     source -> lexer -> parser -> AST -> serializer -> parser
 *
 * must preserve intended communication semantics.
 *
 * ============================================================================
 * 56. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [ ] It compiles as an ANTLR parser grammar.
 *     [ ] ZamaniLexer is the canonical token vocabulary.
 *     [ ] Names is the authoritative name grammar.
 *     [ ] Expressions is the authoritative expression grammar.
 *     [ ] No identifier grammar is duplicated.
 *     [ ] No expression grammar is duplicated.
 *     [ ] No network protocol is hard-coded.
 *     [ ] No hardware topology is hard-coded.
 *     [ ] No machine count is hard-coded.
 *     [ ] No resource capacity is hard-coded.
 *     [ ] No runtime action exists.
 *     [ ] No semantic predicate exists.
 *     [ ] No Rust code exists inside the grammar.
 *     [ ] Generated Rust integrates with Rust 1.97/1.97.1.
 *     [ ] The Rust crate forbids unsafe code.
 *     [ ] Distributed.g4 consumes communicationDeclaration instead of
 *         redefining communication syntax.
 *     [ ] The communication AST preserves source information.
 *     [ ] Semantic lowering is tested.
 *     [ ] Classical integration is tested.
 *     [ ] Quantum integration is tested.
 *     [ ] HDL integration is tested.
 *     [ ] Hardware integration is tested.
 *     [ ] Security integration is tested.
 *     [ ] Resource integration is tested.
 *     [ ] Scalability tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Round-trip tests pass.
 *
 * ============================================================================
 */