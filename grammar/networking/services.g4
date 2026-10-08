/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/networking/services.g4
 *
 * GRAMMAR
 * -------
 * NetworkingServices
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION NETWORKING SERVICE GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No unsafe implementation required
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SOURCE-LEVEL GRAMMAR OWNER for logical NETWORK
 * SERVICE CONTRACTS.
 *
 * A network service describes a logical communication-facing interface.
 *
 * It may describe:
 *
 *     - service identity;
 *     - generic parameters;
 *     - service refinement;
 *     - operations;
 *     - request relationships;
 *     - response relationships;
 *     - event relationships;
 *     - protocol references;
 *     - channel references;
 *     - endpoint references;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - contracts;
 *     - policies;
 *     - effects;
 *     - lifecycle intent;
 *     - metadata;
 *     - extensibility declarations.
 *
 * This file describes SOURCE-LEVEL INTENT.
 *
 * It does NOT describe physical deployment or runtime implementation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     networkServiceConstruct
 *     networkServiceDeclaration
 *     networkServiceReference
 *
 *     networkServiceBody
 *     networkServiceMember
 *
 *     networkServiceOperationDeclaration
 *     networkServiceParameterList
 *     networkServiceParameter
 *     networkServiceParameterDefault
 *     networkServiceReturnClause
 *     networkServiceOperationClause
 *
 *     networkServiceRequestDeclaration
 *     networkServiceRequestReference
 *
 *     networkServiceResponseDeclaration
 *     networkServiceResponseReference
 *
 *     networkServiceEventDeclaration
 *
 *     networkServiceProtocolReference
 *     networkServiceChannelReference
 *     networkServiceEndpointReference
 *
 *     networkServiceRequirement
 *     networkServiceCapability
 *     networkServiceConstraint
 *     networkServicePreference
 *
 *     networkServiceContract
 *     networkServicePolicy
 *     networkServiceEffect
 *
 *     networkServiceLifecycle
 *     networkServiceMetadata
 *     networkServiceExtension
 *
 *     network-service-specific structural wrappers needed by the above.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - attributes;
 *     - types;
 *     - expressions;
 *     - generic type syntax;
 *     - messages;
 *     - protocols;
 *     - channels;
 *     - endpoints;
 *     - requests as reusable top-level networking contracts;
 *     - responses as reusable top-level networking contracts;
 *     - service discovery;
 *     - sockets;
 *     - streams;
 *     - routing;
 *     - addressing;
 *     - transport implementation;
 *     - serialization;
 *     - authentication;
 *     - authorization;
 *     - cryptography;
 *     - distributed placement;
 *     - scheduling;
 *     - resource allocation;
 *     - hardware discovery;
 *     - quantum operations;
 *     - quantum states;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - classical IR;
 *     - quantum::ir;
 *     - runtime execution;
 *     - deployment.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST NOT recreate syntax owned by another canonical grammar.
 *
 * Canonical ownership:
 *
 *     identifiers / names
 *         -> grammar/core/names.g4
 *
 *     attributes
 *         -> grammar/core/attributes.g4
 *
 *     types
 *         -> grammar/types/
 *
 *     expressions
 *         -> grammar/expressions/
 *
 *     standalone contracts
 *         -> grammar/statements/contract.g4
 *
 *     reusable policies
 *         -> grammar/core/policies.g4
 *
 *     requirements
 *         -> grammar/core/requirements.g4
 *
 *     constraints
 *         -> grammar/core/constraints.g4
 *
 *     capabilities
 *         -> grammar/core/capabilities.g4
 *
 *     reusable network requests
 *         -> grammar/networking/requests.g4
 *
 *     reusable network responses
 *         -> grammar/networking/responses.g4
 *
 *     message schemas
 *         -> grammar/networking/messages.g4
 *
 *     protocols
 *         -> grammar/networking/protocols.g4
 *
 *     channels
 *         -> grammar/networking/channels.g4
 *
 *     endpoints
 *         -> grammar/networking/endpoints.g4
 *
 *     networking composition
 *         -> grammar/networking/networking.g4
 *
 * Service-local syntax in this file is allowed only where the construct
 * specifically belongs to the service contract boundary.
 *
 * ============================================================================
 * SERVICE VERSUS DISTRIBUTED SERVICE
 * ============================================================================
 *
 * `grammar/distributed/services.g4` owns distributed computational service
 * semantics.
 *
 * This file owns NETWORK-FACING SERVICE CONTRACTS.
 *
 * A distributed service may expose a network service.
 *
 * A network service may refer to a distributed service through a qualified
 * symbolic name.
 *
 * This grammar MUST NOT redefine:
 *
 *     nodes;
 *     processes;
 *     workers;
 *     actors;
 *     replicas;
 *     placement;
 *     distributed scheduling;
 *     consensus;
 *     distributed state;
 *     distributed lifecycle implementation.
 *
 * ============================================================================
 * NETWORKING COMPONENT BOUNDARIES
 * ============================================================================
 *
 * Protocol syntax:
 *
 *     grammar/networking/protocols.g4
 *
 * Channel syntax:
 *
 *     grammar/networking/channels.g4
 *
 * Endpoint syntax:
 *
 *     grammar/networking/endpoints.g4
 *
 * Message syntax:
 *
 *     grammar/networking/messages.g4
 *
 * Reusable request contracts:
 *
 *     grammar/networking/requests.g4
 *
 * Reusable response contracts:
 *
 *     grammar/networking/responses.g4
 *
 * Service discovery:
 *
 *     grammar/networking/service-discovery.g4
 *
 * This file references those concepts.
 *
 * It does not duplicate their declaration grammars.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A network service specifies WHAT communication contract is offered.
 *
 * It does not permanently specify WHERE that contract executes.
 *
 * Therefore this grammar MUST remain independent of:
 *
 *     - machine identity;
 *     - processor identity;
 *     - accelerator identity;
 *     - QPU identity;
 *     - FPGA identity;
 *     - node identity as physical placement;
 *     - physical address;
 *     - physical port;
 *     - network provider;
 *     - cloud provider;
 *     - fixed transport;
 *     - fixed topology;
 *     - fixed replica count;
 *     - fixed client count;
 *     - fixed server count.
 *
 * The same service source may be realized through:
 *
 *     local execution
 *     embedded execution
 *     IPC
 *     shared memory
 *     actor communication
 *     distributed execution
 *     network transport
 *     accelerator communication
 *     quantum/classical infrastructure
 *     HPC communication
 *     cluster infrastructure
 *     cloud infrastructure
 *     future computational substrates
 *
 * provided that downstream semantic analysis establishes feasibility.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar introduces NO artificial language-level finite limit for:
 *
 *     services
 *     operations
 *     parameters
 *     request references
 *     response references
 *     event declarations
 *     protocol references
 *     channel references
 *     endpoint references
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     contracts
 *     policies
 *     effects
 *     lifecycle members
 *     metadata
 *     extensions
 *     generic parameters
 *     qualified-name depth
 *     expression size
 *     source size
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * "Infinity" means:
 *
 *     no artificial finite capacity ceiling is imposed by this grammar.
 *
 * It does NOT mean physically infinite memory, compute, network capacity,
 * compilation resources, or execution resources.
 *
 * Resource exhaustion belongs to the implementation/resource policy layer.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar MUST NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     HTTP/2
 *     HTTP/3
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transports
 *     cloud providers
 *     hardware vendors
 *     AI models
 *     quantum operation catalogs.
 *
 * Such identities remain names, expressions, capabilities, or dialect-defined
 * semantic entities.
 *
 * Adding a new transport or provider MUST NOT require a universal service
 * grammar rewrite merely because the technology is new.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Lexical vocabulary is ultimately owned by:
 *
 *     grammar/lexer/
 *
 * This parser grammar consumes canonical tokens.
 *
 * Important canonical networking tokens include:
 *
 *     SERVICE
 *     ASYNC
 *     FN
 *     REQUIRES
 *     CAPABILITY
 *     CONSTRAINT
 *     PREFER
 *     POLICY
 *     EFFECT
 *     CONTRACT
 *     EVENT
 *     PROTOCOL
 *     CHANNEL
 *     ENDPOINT
 *
 * where those tokens exist in the canonical lexer.
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - declaration kind;
 *     - source span;
 *     - source order;
 *     - attributes;
 *     - visibility;
 *     - service name;
 *     - generic parameters;
 *     - refinement names;
 *     - where expression;
 *     - operation order;
 *     - parameter order;
 *     - parameter names;
 *     - parameter types;
 *     - defaults;
 *     - return type;
 *     - operation clauses;
 *     - request relationships;
 *     - response relationships;
 *     - event declarations;
 *     - protocol references;
 *     - channel references;
 *     - endpoint references;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - contracts;
 *     - policies;
 *     - effects;
 *     - lifecycle intent;
 *     - metadata;
 *     - extensions.
 *
 * Name resolution MUST remain downstream.
 *
 * The parser MUST NOT manufacture semantic identities.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - service identity resolution;
 *     - duplicate declaration detection;
 *     - operation identity resolution;
 *     - overload legality;
 *     - parameter/type compatibility;
 *     - request compatibility;
 *     - response compatibility;
 *     - event compatibility;
 *     - protocol compatibility;
 *     - channel compatibility;
 *     - endpoint-role compatibility;
 *     - effect compatibility;
 *     - capability satisfaction;
 *     - requirement satisfaction;
 *     - constraint satisfiability;
 *     - preference interpretation;
 *     - policy applicability;
 *     - contract validity;
 *     - provenance;
 *     - portability;
 *     - target feasibility.
 *
 * None of these decisions are performed by this grammar.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Network-facing service declarations may carry effects through the canonical
 * effect system.
 *
 * Networking commonly participates in effects such as:
 *
 *     network
 *     distributed
 *     io
 *     foreign
 *     native
 *     mutation
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * This file does not create a second effect taxonomy.
 *
 * Effect meaning is resolved by the existing effects subsystem.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE CONTRACT
 * ============================================================================
 *
 * A service may express semantic requirements such as:
 *
 *     requires capability("network.reliable");
 *
 *     requires capability("security.authentication");
 *
 *     requires memory >= required_memory;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 * These are source-level semantic requirements.
 *
 * They do NOT select hardware.
 *
 * They do NOT impose universal resource ceilings.
 *
 * Target capability negotiation belongs downstream.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Service contracts use the canonical contract semantics.
 *
 * The six universal contract forms remain:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This file MUST NOT implement a competing contract language.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies are semantic governance structures.
 *
 * This file may attach or reference policy information at the service boundary,
 * but policy evaluation remains owned by the policy subsystem.
 *
 * The service grammar does not implement:
 *
 *     authorization;
 *     admission;
 *     enforcement;
 *     conflict resolution;
 *     policy evaluation.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every service construct must remain traceable through:
 *
 *     source
 *       ->
 *     parser context
 *       ->
 *     AST
 *       ->
 *     semantic service model
 *       ->
 *     canonical semantic representation
 *
 * Downstream transformations may record:
 *
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *
 * This grammar itself does not construct provenance records.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The pipeline is:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     NetworkingServices
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     semantic service model
 *       ->
 *     effects/capabilities/resources/contracts/policies/provenance
 *       ->
 *     canonical semantic representation
 *       ->
 *     applicable IR
 *
 * Classical service computation may ultimately lower through the classical
 * semantic/IR pipeline.
 *
 * Quantum computation participating in a service MUST continue through:
 *
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * HDL/hardware communication intent remains target-independent until the HDL/
 * hardware semantic boundary.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns:
 *
 *     - service registration;
 *     - service discovery;
 *     - binding;
 *     - endpoint resolution;
 *     - dispatch;
 *     - transport;
 *     - connection management;
 *     - scheduling;
 *     - placement;
 *     - retry/recovery;
 *     - replication;
 *     - failover;
 *     - migration;
 *     - lifecycle execution.
 *
 * This grammar does not execute any of those operations.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * The grammar must provide enough structure for:
 *
 *     - syntax highlighting;
 *     - formatting;
 *     - service documentation;
 *     - interface extraction;
 *     - symbol indexing;
 *     - API documentation;
 *     - semantic navigation;
 *     - refactoring;
 *     - diagnostics;
 *     - provenance;
 *     - source mapping.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *
 *     canonical networking component grammars where their public reference
 *     rules are consumed.
 *
 * CONSUMED_BY:
 *
 *     grammar/networking/networking.g4
 *
 *     grammar/antlr/ZamaniParser.g4
 *         indirectly through Networking.
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST implementation.
 *
 * SEMANTIC_OWNER:
 *
 *     networking semantic subsystem plus shared semantic infrastructure.
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR pipeline.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/networking/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/networking.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/spec/contracts.md
 *     grammar/spec/policies.md
 *     grammar/spec/provenance.md
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Core supplies:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     whereExpression / where-related canonical constructs
 *     requirements
 *     constraints
 *     capabilities
 *     policies
 *     metadata where exposed by the canonical Core composition.
 *
 * Types supplies:
 *
 *     typeExpression
 *
 * Expressions supplies:
 *
 *     expression
 *     argument lists
 *     block expressions
 *
 * Contract syntax is intentionally consumed through the canonical
 * ContractStatements grammar rather than duplicated here.
 *
 * ============================================================================
 */

parser grammar NetworkingServices;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions,
    ContractStatements
    ;


/*
 * ============================================================================
 * PUBLIC SERVICE CONSTRUCT
 * ============================================================================
 *
 * This is the stable public rule consumed by:
 *
 *     grammar/networking/networking.g4
 *
 * It distinguishes a service declaration from a symbolic service reference.
 *
 * A reference is intentionally restricted to a qualified name.
 *
 * ============================================================================
 */

networkServiceConstruct
    : networkServiceDeclaration
    | networkServiceReference
    ;


/*
 * ============================================================================
 * SERVICE DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     service Name {
 *         ...
 *     }
 *
 * Optional forms:
 *
 *     public service Name { ... }
 *
 *     service Name<T> { ... }
 *
 *     service Name extends Base { ... }
 *
 *     service Name<T> extends Base where condition { ... }
 *
 * The SERVICE token is authoritative.
 *
 * ============================================================================
 */

networkServiceDeclaration
    : attribute*
      visibility?
      SERVICE
      qualifiedName
      genericParameters?
      networkServiceRefinementClause?
      networkServiceWhereClause?
      networkServiceBody
      SEMI?
    ;


/*
 * ============================================================================
 * SERVICE REFERENCE
 * ============================================================================
 *
 * A service reference is a symbolic reference to a service defined elsewhere.
 *
 * It is NOT a service declaration.
 *
 * Semantic analysis resolves the referenced service.
 * ============================================================================
 */

networkServiceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SERVICE REFINEMENT
 * ============================================================================
 *
 * Refinement composes service contracts.
 *
 * It does not imply:
 *
 *     process inheritance;
 *     machine inheritance;
 *     node inheritance;
 *     deployment inheritance.
 *
 * ============================================================================
 */

networkServiceRefinementClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * SERVICE WHERE
 * ============================================================================
 *
 * The condition is a canonical expression.
 *
 * Type checking and constraint validation are downstream.
 * ============================================================================
 */

networkServiceWhereClause
    : WHERE
      expression
    ;


/*
 * ============================================================================
 * SERVICE BODY
 * ============================================================================
 *
 * A service body is structurally ordered.
 *
 * The AST must preserve source order.
 * ============================================================================
 */

networkServiceBody
    : LBRACE
      networkServiceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * SERVICE MEMBER DISPATCH
 * ============================================================================
 *
 * Explicit lexical markers are preferred wherever canonical tokens exist.
 *
 * Generic extensions remain available through networkServiceExtension.
 *
 * This prevents future networking concepts from requiring a permanent
 * universal keyword for every possible capability.
 * ============================================================================
 */

networkServiceMember
    : networkServiceOperationDeclaration
    | networkServiceRequestDeclaration
    | networkServiceResponseDeclaration
    | networkServiceEventDeclaration
    | networkServiceProtocolReference
    | networkServiceChannelReference
    | networkServiceEndpointReference
    | networkServiceRequirement
    | networkServiceCapability
    | networkServiceConstraint
    | networkServicePreference
    | networkServiceContract
    | networkServicePolicy
    | networkServiceEffect
    | networkServiceLifecycle
    | networkServiceMetadata
    | networkServiceExtension
    ;


/*
 * ============================================================================
 * OPERATION DECLARATION
 * ============================================================================
 *
 * The canonical operation form accepts:
 *
 *     fn operation(...)
 *
 * and:
 *
 *     async fn operation(...)
 *
 * A legacy/contextual bare operation form is deliberately NOT accepted.
 *
 * This makes service-member parsing deterministic and prevents arbitrary
 * identifiers from being interpreted as operations.
 *
 * ============================================================================
 */

networkServiceOperationDeclaration
    : attribute*
      ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN
      networkServiceParameterList?
      RPAREN
      networkServiceReturnClause?
      networkServiceOperationClause*
      networkServiceWhereClause?
      networkServiceOperationBody?
      SEMI?
    ;


/*
 * ============================================================================
 * OPERATION PARAMETERS
 * ============================================================================
 */

networkServiceParameterList
    : networkServiceParameter
      (COMMA networkServiceParameter)*
      COMMA?
    ;


networkServiceParameter
    : attribute*
      identifier
      COLON
      typeExpression
      networkServiceParameterDefault?
    ;


networkServiceParameterDefault
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * OPERATION RETURN TYPE
 * ============================================================================
 */

networkServiceReturnClause
    : ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * OPERATION CLAUSES
 * ============================================================================
 *
 * Service operations may attach logical communication relationships and
 * semantic obligations.
 *
 * These clauses do not execute communication.
 * ============================================================================
 */

networkServiceOperationClause
    : networkServiceRequestReference
    | networkServiceResponseReference
    | networkServiceProtocolReference
    | networkServiceChannelReference
    | networkServiceEndpointReference
    | networkServiceRequirement
    | networkServiceCapability
    | networkServiceConstraint
    | networkServicePreference
    | networkServiceContract
    | networkServicePolicy
    | networkServiceEffect
    ;


/*
 * ============================================================================
 * OPERATION BODY
 * ============================================================================
 *
 * A service operation may be interface-only or may contain an implementation
 * body.
 *
 * Runtime behavior remains downstream.
 * ============================================================================
 */

networkServiceOperationBody
    : blockExpression
    ;


/*
 * ============================================================================
 * SERVICE-LOCAL REQUEST DECLARATION
 * ============================================================================
 *
 * A service-local request binds a symbolic request name to an existing message
 * or type.
 *
 * Reusable first-class request contracts remain owned by requests.g4.
 *
 * Example:
 *
 *     request GetUser : api::GetUser;
 *
 * The semantic layer determines whether the referenced entity is a legal
 * request/message/type.
 * ============================================================================
 */

networkServiceRequestDeclaration
    : attribute*
      networkServiceRequestMarker
      identifier
      networkServiceMessageTypeClause?
      networkServiceCommunicationReference*
      SEMI
    ;


networkServiceRequestMarker
    : REQUEST
    ;


networkServiceRequestReference
    : REQUEST
      qualifiedName
      SEMI?
    ;


/*
 * ============================================================================
 * SERVICE-LOCAL RESPONSE DECLARATION
 * ============================================================================
 *
 * Example:
 *
 *     response GetUserResult : api::User;
 * ============================================================================
 */

networkServiceResponseDeclaration
    : attribute*
      networkServiceResponseMarker
      identifier
      networkServiceMessageTypeClause?
      networkServiceCommunicationReference*
      SEMI
    ;


networkServiceResponseMarker
    : RESPONSE
    ;


networkServiceResponseReference
    : RESPONSE
      qualifiedName
      SEMI?
    ;


/*
 * ============================================================================
 * SERVICE EVENT
 * ============================================================================
 *
 * Events describe outbound logical communication.
 *
 * They do not imply:
 *
 *     multicast;
 *     broadcast;
 *     queueing;
 *     persistence;
 *     transport;
 *     replication.
 *
 * Those meanings remain semantic/runtime concerns.
 * ============================================================================
 */

networkServiceEventDeclaration
    : attribute*
      EVENT
      identifier
      networkServiceMessageTypeClause?
      networkServiceCommunicationReference*
      networkServiceRequirement*
      networkServiceCapability*
      networkServiceContract*
      networkServiceProperty*
      SEMI
    ;


/*
 * ============================================================================
 * MESSAGE TYPE CLAUSE
 * ============================================================================
 *
 * Type ownership remains with Types.
 * ============================================================================
 */

networkServiceMessageTypeClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * COMMUNICATION REFERENCES
 * ============================================================================
 *
 * These are service-local relationships to independently owned networking
 * constructs.
 * ============================================================================
 */

networkServiceCommunicationReference
    : networkServiceProtocolReference
    | networkServiceChannelReference
    | networkServiceEndpointReference
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCE
 * ============================================================================
 *
 * The protocol itself is owned by networking/protocols.g4.
 *
 * This rule records a reference only.
 * ============================================================================
 */

networkServiceProtocolReference
    : PROTOCOL
      qualifiedName
      networkServiceReferenceAssignment?
      SEMI?
    ;


networkServiceProtocolName
    : qualifiedName
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCE
 * ============================================================================
 *
 * Channel declarations are owned by networking/channels.g4.
 * ============================================================================
 */

networkServiceChannelReference
    : CHANNEL
      qualifiedName
      networkServiceReferenceAssignment?
      SEMI?
    ;


networkServiceChannelName
    : qualifiedName
    ;


/*
 * ============================================================================
 * ENDPOINT REFERENCE
 * ============================================================================
 *
 * Endpoint declarations are owned by networking/endpoints.g4.
 * ============================================================================
 */

networkServiceEndpointReference
    : ENDPOINT
      qualifiedName
      networkServiceReferenceAssignment?
      SEMI?
    ;


networkServiceEndpointName
    : qualifiedName
    ;


networkServiceReferenceAssignment
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Requirement syntax is intentionally attached to the canonical REQUIRES
 * token and canonical expression grammar.
 *
 * Semantic interpretation belongs to the resource/requirement subsystem.
 * ============================================================================
 */

networkServiceRequirement
    : attribute*
      REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Capability names are open-world qualified names.
 *
 * Optional assignment allows capability arguments/conditions without requiring
 * a new keyword for every capability family.
 * ============================================================================
 */

networkServiceCapability
    : attribute*
      CAPABILITY
      qualifiedName
      networkServiceReferenceAssignment?
      SEMI
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 */

networkServiceConstraint
    : attribute*
      CONSTRAINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences remain advisory.
 *
 * They MUST NOT silently become mandatory requirements.
 * ============================================================================
 */

networkServicePreference
    : attribute*
      PREFER
      expression
      SEMI
    ;


/*
 * ============================================================================
 * CONTRACT
 * ============================================================================
 *
 * A service-local contract block reuses the canonical six contract statements.
 *
 * Example:
 *
 *     contract {
 *         requires(condition);
 *         ensures(condition);
 *         invariant(condition);
 *     }
 *
 * The contract statements themselves remain owned by ContractStatements.
 * ============================================================================
 */

networkServiceContract
    : attribute*
      CONTRACT
      networkServiceContractName?
      LBRACE
      contractStatement*
      RBRACE
      SEMI?
    ;


networkServiceContractName
    : identifier
    ;


/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * A service may attach a named policy reference or a local policy body.
 *
 * The universal policy declaration itself remains owned by ZamaniPolicies.
 *
 * This wrapper is intentionally reference-oriented and does not recreate the
 * policy language.
 * ============================================================================
 */

networkServicePolicy
    : attribute*
      POLICY
      qualifiedName
      networkServiceReferenceAssignment?
      SEMI?
    ;


/*
 * ============================================================================
 * EFFECT
 * ============================================================================
 *
 * Service/operation effect annotations remain generic.
 *
 * The effect name is open-world.
 *
 * The semantic effect system determines validity.
 * ============================================================================
 */

networkServiceEffect
    : attribute*
      EFFECT
      qualifiedName
      networkServiceReferenceAssignment?
      SEMI
    ;


/*
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 *
 * Lifecycle is declarative intent only.
 *
 * It does not execute start/stop/restart/migrate/failover operations.
 *
 * Lifecycle vocabulary remains open-world and is represented structurally.
 * ============================================================================
 */

networkServiceLifecycle
    : attribute*
      lifecycleMarker
      identifier?
      networkServiceLifecycleBody?
      SEMI?
    ;


lifecycleMarker
    : identifier
    ;


networkServiceLifecycleBody
    : LBRACE
      networkServiceLifecycleMember*
      RBRACE
    ;


networkServiceLifecycleMember
    : networkServiceLifecycleClause
    | networkServiceRequirement
    | networkServiceCapability
    | networkServiceConstraint
    | networkServicePreference
    | networkServiceProperty
    | networkServiceExtension
    ;


networkServiceLifecycleClause
    : identifier
      networkServiceLifecycleValue?
      SEMI
    ;


networkServiceLifecycleValue
    : COLON
      expression
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is source-level descriptive information.
 *
 * Semantic metadata schemas remain outside this grammar.
 * ============================================================================
 */

networkServiceMetadata
    : attribute*
      metadataMarker
      identifier
      networkServiceMetadataValue
      SEMI
    ;


metadataMarker
    : identifier
    ;


networkServiceMetadataValue
    : COLON expression
    | ASSIGN expression
    ;


/*
 * ============================================================================
 * GENERIC PROPERTY
 * ============================================================================
 *
 * Generic service properties provide an open-world extension point without
 * reserving a new keyword for every future networking feature.
 *
 * Property names are ordinary identifiers or qualified names.
 * ============================================================================
 */

networkServiceProperty
    : attribute*
      networkServicePropertyName
      networkServicePropertyType?
      networkServicePropertyInitializer?
      SEMI
    ;


networkServicePropertyName
    : identifier
    | qualifiedName
    ;


networkServicePropertyType
    : COLON
      typeExpression
    ;


networkServicePropertyInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * EXTENSION
 * ============================================================================
 *
 * Extensions are explicitly named semantic extension points.
 *
 * They do not define a new language-wide subsystem.
 *
 * Semantic validation determines whether the extension is registered and
 * valid for the current language/dialect configuration.
 * ============================================================================
 */

networkServiceExtension
    : attribute*
      EXTENSION
      qualifiedName
      networkServiceExtensionBody?
      SEMI?
    ;


networkServiceExtensionBody
    : LBRACE
      networkServiceExtensionMember*
      RBRACE
    ;


networkServiceExtensionMember
    : networkServiceProperty
    | networkServiceRequirement
    | networkServiceCapability
    | networkServiceConstraint
    | networkServicePreference
    | networkServiceContract
    | networkServicePolicy
    | networkServiceEffect
    | networkServiceExtension
    ;


/*
 * ============================================================================
 * HELPER REFERENCES
 * ============================================================================
 *
 * These are syntactic helper boundaries only.
 *
 * They do not perform name lookup.
 * ============================================================================
 */

networkServiceMessageReference
    : qualifiedName
    ;


networkServiceProtocolReferenceName
    : qualifiedName
    ;


networkServiceChannelReferenceName
    : qualifiedName
    ;


networkServiceEndpointReferenceName
    : qualifiedName
    ;


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO:
 *
 *     MAX_SERVICES
 *     MAX_OPERATIONS
 *     MAX_PARAMETERS
 *     MAX_MESSAGES
 *     MAX_PROTOCOLS
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_CLIENTS
 *     MAX_SERVERS
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_DEVICE_COUNT
 *
 * It contains no fixed:
 *
 *     - transport;
 *     - provider;
 *     - topology;
 *     - machine;
 *     - hardware;
 *     - physical address;
 *     - physical port;
 *     - replica count.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token stream;
 *     - grammar version;
 *     - parser configuration;
 *     - explicitly selected dialect configuration where applicable.
 *
 * Parsing MUST NOT depend on:
 *
 *     - network state;
 *     - filesystem state;
 *     - hardware availability;
 *     - runtime state;
 *     - wall-clock time;
 *     - randomness;
 *     - scheduler state;
 *     - target selection.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime callbacks;
 *     - no unsafe implementation requirement.
 *
 * The consuming Zamani frontend remains compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *     - invalid service declaration;
 *     - missing service name;
 *     - malformed generic parameters;
 *     - malformed refinement;
 *     - malformed operation;
 *     - malformed parameter;
 *     - malformed return type;
 *     - malformed request;
 *     - malformed response;
 *     - malformed event;
 *     - malformed protocol reference;
 *     - malformed channel reference;
 *     - malformed endpoint reference;
 *     - malformed requirement;
 *     - malformed capability;
 *     - malformed constraint;
 *     - malformed preference;
 *     - malformed contract;
 *     - malformed policy reference;
 *     - malformed effect;
 *     - malformed lifecycle;
 *     - malformed metadata;
 *     - malformed extension.
 *
 * Semantic diagnostics remain downstream and should distinguish:
 *
 *     unresolved service;
 *     duplicate operation;
 *     incompatible types;
 *     incompatible request/response;
 *     unavailable capability;
 *     unsatisfied resource requirement;
 *     invalid effect;
 *     invalid policy;
 *     invalid contract;
 *     invalid endpoint;
 *     invalid channel;
 *     invalid protocol;
 *     invalid portability requirement.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE SERVICE DECLARATIONS
 * -----------------------------
 *
 *     service telemetry {
 *     }
 *
 *     public service telemetry {
 *     }
 *
 *     service compute<T> {
 *     }
 *
 *     service api extends base::Service {
 *     }
 *
 *
 * POSITIVE OPERATIONS
 * -------------------
 *
 *     service compute {
 *         fn run(input: Data) -> Result;
 *     }
 *
 *     service compute {
 *         async fn run(input: Data) -> Result;
 *     }
 *
 *     service compute {
 *         fn run(input: Data = default_value) -> Result;
 *     }
 *
 *
 * POSITIVE COMMUNICATION REFERENCES
 * ---------------------------------
 *
 *     service api {
 *         protocol network::reliable;
 *         channel telemetry::results;
 *         endpoint compute::worker;
 *     }
 *
 *
 * POSITIVE REQUEST / RESPONSE
 * ---------------------------
 *
 *     service api {
 *         request GetData: data::GetData;
 *         response GetDataResult: data::GetDataResult;
 *     }
 *
 *
 * POSITIVE EVENTS
 * ---------------
 *
 *     service telemetry {
 *         event Measurement: telemetry::Measurement;
 *     }
 *
 *
 * POSITIVE REQUIREMENTS
 * ---------------------
 *
 *     service quantum_api {
 *         requires capability("quantum.measurement");
 *         requires memory >= required_memory;
 *     }
 *
 *
 * POSITIVE CAPABILITY
 * -------------------
 *
 *     service compute {
 *         capability compute::tensor;
 *         capability quantum::measurement;
 *     }
 *
 *
 * POSITIVE CONSTRAINTS / PREFERENCES
 * ----------------------------------
 *
 *     service compute {
 *         constraint topology.supports(required_topology);
 *         prefer resource::latency;
 *     }
 *
 *
 * POSITIVE CONTRACT
 * -----------------
 *
 *     service compute {
 *         contract {
 *             requires(input.is_valid());
 *             ensures(result.is_valid());
 *             invariant(state.is_consistent());
 *             assume(environment.is_valid());
 *             guarantee(output.is_valid());
 *             property(result.is_reproducible());
 *         }
 *     }
 *
 *
 * POSITIVE EFFECTS
 * ----------------
 *
 *     service remote {
 *         effect network;
 *         effect distributed;
 *     }
 *
 *
 * POSITIVE GENERIC EXTENSIONS
 * ---------------------------
 *
 *     service future {
 *         future::property: expression;
 *     }
 *
 *
 * POSITIVE HYBRID / QUANTUM
 * -------------------------
 *
 *     service quantum_compute {
 *         requires capability("quantum.measurement");
 *         requires capability("network.quantum");
 *
 *         fn execute(circuit: QuantumCircuit)
 *             -> MeasurementResult;
 *     }
 *
 *
 * POSITIVE HARDWARE / ACCELERATOR
 * ------------------------------
 *
 *     service accelerator {
 *         requires capability("accelerator.compute");
 *         requires capability("tensor.compute");
 *
 *         fn execute(input: Tensor) -> Tensor;
 *     }
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     service;
 *
 *     service {
 *     }
 *
 *     service api
 *
 *     service api {
 *         fn;
 *     }
 *
 *     service api {
 *         fn run;
 *     }
 *
 *     service api {
 *         fn run(input);
 *     }
 *
 *     service api {
 *         fn run(input:);
 *     }
 *
 *     service api {
 *         fn run(input: Data) ->;
 *     }
 *
 *     service api {
 *         requires;
 *     }
 *
 *     service api {
 *         capability;
 *     }
 *
 *     service api {
 *         protocol;
 *     }
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generate services with progressively increasing:
 *
 *     - service count;
 *     - operation count;
 *     - parameter count;
 *     - reference count;
 *     - contract count;
 *     - policy count;
 *     - metadata count;
 *     - extension count;
 *     - qualified-name depth;
 *     - expression size;
 *     - generic nesting.
 *
 * No grammar change is permitted merely to accommodate larger valid inputs.
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Services must be tested with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     AI/model computation;
 *     tensor computation;
 *     HDL/hardware intent;
 *     accelerator computation;
 *     concurrency;
 *     distributed execution;
 *     data processing;
 *     security;
 *     simulation;
 *     deterministic execution;
 *     reproducible execution.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parse identical token streams repeatedly under the same:
 *
 *     lexer version;
 *     grammar version;
 *     parser configuration;
 *
 * and verify equivalent parse-tree structure and source spans.
 *
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * The same service source must remain syntactically valid regardless of
 * whether the eventual target is:
 *
 *     embedded;
 *     CPU;
 *     multicore;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     simulator;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud infrastructure;
 *     future computational substrate.
 *
 * Target feasibility is a downstream semantic concern.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         ->
 *     canonical lexical vocabulary
 *
 *     grammar/core/
 *         ->
 *     names, attributes, visibility, generics, requirements, constraints,
 *     capabilities, policies, metadata
 *
 *     grammar/types/
 *         ->
 *     typeExpression
 *
 *     grammar/expressions/
 *         ->
 *     expression and blockExpression
 *
 *     grammar/statements/contract.g4
 *         ->
 *     contractStatement
 *
 *
 * SIDEWAYS NETWORKING REFERENCES
 * ------------------------------
 *
 *     grammar/networking/protocols.g4
 *     grammar/networking/channels.g4
 *     grammar/networking/endpoints.g4
 *     grammar/networking/messages.g4
 *     grammar/networking/requests.g4
 *     grammar/networking/responses.g4
 *
 * These grammars remain separate authorities.
 *
 *
 * DOWNSTREAM
 * ----------
 *
 *     grammar/networking/networking.g4
 *         ->
 *     networkingService
 *
 *     frontend domain-neutral AST
 *         ->
 *     structural validation
 *
 *     structural validation
 *         ->
 *     semantic service model
 *
 *     semantic service model
 *         ->
 *     name/type/effect/capability/resource/contract/policy/provenance analysis
 *
 *     semantic service model
 *         ->
 *     canonical semantic representation
 *
 *     canonical semantic representation
 *         ->
 *     classical IR
 *     quantum::ir where applicable
 *     HDL/hardware semantic representation where applicable
 *
 *     downstream compiler
 *         ->
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     resilience
 *     ZQN
 *     HAL
 *     target realization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A service may expose quantum computation, but this grammar does not define
 * quantum operations.
 *
 * Example:
 *
 *     service quantum_compute {
 *         requires capability("quantum.execute");
 *
 *         fn execute(circuit: QuantumCircuit)
 *             -> MeasurementResult;
 *     }
 *
 * The semantic path is:
 *
 *     service AST
 *       ->
 *     semantic service model
 *       ->
 *     quantum semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * This file MUST NOT enumerate quantum gates or physical qubits.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A service may expose hardware or accelerator operations through ordinary
 * types, names, expressions, properties, and capabilities.
 *
 * This file does not define:
 *
 *     pins;
 *     wires;
 *     physical buses;
 *     FPGA routing;
 *     ASIC cells;
 *     clock implementations;
 *     device identifiers.
 *
 * Those belong to HDL/hardware semantic layers.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Service requirements participate in:
 *
 *     requirement analysis
 *       ->
 *     capability negotiation
 *       ->
 *     resource analysis
 *       ->
 *     execution planning
 *       ->
 *     target realization
 *
 * No resource is allocated during parsing.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policy references participate in:
 *
 *     service AST
 *       ->
 *     policy semantic model
 *       ->
 *     applicability
 *       ->
 *     authorization/admission where relevant
 *       ->
 *     execution planning
 *
 * This grammar never evaluates a policy.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Effect declarations participate in:
 *
 *     service AST
 *       ->
 *     effect analysis
 *       ->
 *     capability/resource/security analysis
 *       ->
 *     semantic service model
 *
 * Effect semantics remain globally owned.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Every service declaration and member must remain source-traceable.
 *
 * The AST builder should associate:
 *
 *     service declaration
 *     member
 *     operation
 *     parameter
 *     communication reference
 *     requirement
 *     capability
 *     contract
 *     policy
 *     effect
 *     extension
 *
 * with their exact source spans.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The service declaration spelling is now canonical:
 *
 *     service Name { ... }
 *
 * `SERVICE` is consumed directly from the canonical lexer.
 *
 * The old pattern:
 *
 *     identifier identifier ...
 *
 * MUST NOT be retained as the service declaration mechanism because it permits
 * arbitrary source words to masquerade as declaration markers.
 *
 * Service operations are canonical:
 *
 *     fn name(...)
 *
 *     async fn name(...)
 *
 * A bare identifier operation syntax is intentionally not accepted.
 *
 * This is a deliberate grammar-hardening change.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] NetworkingServices is the grammar name.
 *     [ ] tokenVocab = ZamaniLexer is present.
 *     [ ] Core is reused.
 *     [ ] Types is reused.
 *     [ ] Expressions is reused.
 *     [ ] ContractStatements is reused.
 *     [ ] SERVICE is consumed as the service declaration marker.
 *     [ ] service identity uses canonical qualifiedName.
 *     [ ] service body is structurally bounded by braces.
 *     [ ] operation syntax uses canonical FN/ASYNC tokens.
 *     [ ] operation parameters use canonical typeExpression.
 *     [ ] operation defaults use canonical expression.
 *     [ ] return types use canonical typeExpression.
 *     [ ] service-local requirements use canonical REQUIRES.
 *     [ ] service-local capabilities use canonical CAPABILITY.
 *     [ ] service-local constraints use canonical CONSTRAINT.
 *     [ ] service-local preferences use canonical PREFER.
 *     [ ] service contracts delegate to contractStatement.
 *     [ ] no competing contract grammar is implemented.
 *     [ ] policy handling remains policy-subsystem compatible.
 *     [ ] effects remain globally owned.
 *     [ ] protocol syntax is referenced rather than reimplemented.
 *     [ ] channel syntax is referenced rather than reimplemented.
 *     [ ] endpoint syntax is referenced rather than reimplemented.
 *     [ ] message schemas are not reimplemented.
 *     [ ] reusable request contracts remain owned by requests.g4.
 *     [ ] reusable response contracts remain owned by responses.g4.
 *     [ ] no transport catalog exists.
 *     [ ] no hardware catalog exists.
 *     [ ] no quantum operation catalog exists.
 *     [ ] no physical topology is encoded.
 *     [ ] no machine capacity is encoded.
 *     [ ] no fixed networking capacity is encoded.
 *     [ ] no embedded Rust exists.
 *     [ ] no unsafe implementation is required.
 *     [ ] no semantic predicates exist.
 *     [ ] no runtime behavior exists.
 *     [ ] no IR is constructed.
 *     [ ] AST ownership remains downstream.
 *     [ ] semantic ownership remains downstream.
 *     [ ] provenance remains traceable.
 *     [ ] positive tests exist.
 *     [ ] negative tests exist.
 *     [ ] scalability tests exist.
 *     [ ] determinism tests exist.
 *     [ ] portability tests exist.
 *     [ ] cross-domain tests exist.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 *     SERVICE SOURCE
 *          |
 *          v
 *     NETWORKING SERVICES GRAMMAR
 *          |
 *          v
 *     DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     STRUCTURAL VALIDATION
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     SEMANTIC SERVICE MODEL       PROVENANCE
 *          |
 *          +----------+----------+----------+
 *          |          |          |          |
 *          v          v          v          v
 *        TYPES      EFFECTS   CAPABILITIES RESOURCES
 *          |          |          |          |
 *          +----------+----------+----------+
 *                     |
 *                     v
 *                  CONTRACTS
 *                     |
 *                     v
 *                  POLICIES
 *                     |
 *                     v
 *          CANONICAL SEMANTIC MODEL
 *                     |
 *          +----------+----------+
 *          |                     |
 *          v                     v
 *     classical IR          quantum::ir
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *             optimization
 *                     |
 *             lowering/routing
 *                     |
 *                scheduling
 *                     |
 *               resilience
 *                     |
 *                ZQN / HAL
 *                     |
 *                     v
 *              TARGET REALIZATION
 *
 * The service source remains independent of the physical realization.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */