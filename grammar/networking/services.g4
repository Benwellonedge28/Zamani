/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Networking Service Grammar
 * ============================================================================
 *
 * File:
 *     grammar/networking/services.g4
 *
 * Grammar:
 *     NetworkingServices
 *
 * Purpose:
 *     Canonical source-level grammar for network-facing service contracts.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This grammar owns the SYNTAX of a logical network-facing service contract.
 *
 * A network service describes:
 *
 *     - a communication-facing interface;
 *     - operations exposed through communication;
 *     - accepted/request messages;
 *     - produced/response messages;
 *     - protocol references;
 *     - channel references;
 *     - endpoint roles;
 *     - capabilities;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - communication policies;
 *     - service metadata;
 *     - semantic lifecycle intent;
 *
 * A network service is a SOURCE-LEVEL CONTRACT.
 *
 * It is NOT a runtime service instance.
 *
 * It is NOT a process.
 *
 * It is NOT a node.
 *
 * It is NOT a machine.
 *
 * It is NOT a socket.
 *
 * It is NOT an IP address.
 *
 * It is NOT a transport implementation.
 *
 * It is NOT a deployment description.
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
 *     NetworkingServices
 *          |
 *          v
 *     Frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> networking analysis
 *          +--> distributed analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> distributed representation
 *          +--> hardware representation
 *          +--> networking representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / placement
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar NEVER constructs IR.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - network-facing service declarations;
 *     - service operation contracts;
 *     - service request/response contracts;
 *     - service message references;
 *     - service protocol references;
 *     - service channel references;
 *     - service endpoint-role references;
 *     - service capability requirements;
 *     - service semantic requirements;
 *     - service constraints;
 *     - service preferences;
 *     - service communication policies;
 *     - service lifecycle intent;
 *     - service metadata;
 *     - service extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic distributed service declarations;
 *     - distributed node declarations;
 *     - endpoint declarations;
 *     - channel declarations;
 *     - protocol declarations;
 *     - message schemas;
 *     - serialization;
 *     - networking transport;
 *     - sockets;
 *     - ports;
 *     - IP/MAC addresses;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - hardware discovery;
 *     - resource discovery;
 *     - security implementation;
 *     - cryptographic implementation;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - runtime service instances;
 *     - service discovery implementation;
 *     - load balancing;
 *     - replication;
 *     - consensus;
 *     - resilience;
 *     - deployment.
 *
 * ============================================================================
 * DISTRIBUTED SERVICE BOUNDARY
 * ============================================================================
 *
 * `grammar/distributed/services.g4` owns distributed computational services.
 *
 * This file MUST NOT redefine that grammar.
 *
 * Instead:
 *
 *     distributed service
 *          |
 *          +--> may expose a network service contract
 *                         |
 *                         v
 *                    networking service
 *
 * A network service may reference a distributed service symbolically.
 *
 * A network service does not create a distributed node or placement.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * Endpoint declarations are owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * Protocol declarations are owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * Channel declarations are owned by:
 *
 *     grammar/networking/channels.g4
 *
 * Message declarations are owned by:
 *
 *     grammar/networking/messages.g4
 *
 * This grammar references those concepts.
 *
 * It does not redefine them.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A network service describes WHAT communication interface exists.
 *
 * It does not permanently describe WHERE the interface executes.
 *
 * Therefore this grammar does NOT require:
 *
 *     - a specific machine;
 *     - a specific node;
 *     - a specific CPU;
 *     - a specific GPU;
 *     - a specific FPGA;
 *     - a specific QPU;
 *     - a specific network;
 *     - a specific address;
 *     - a specific port;
 *     - a specific transport;
 *     - a specific cloud provider;
 *     - a specific cluster;
 *     - a specific topology;
 *     - a fixed number of replicas;
 *     - a fixed number of clients;
 *     - a fixed number of servers.
 *
 * This preserves:
 *
 *     Program_Once
 *         ->
 *     Compile_Once
 *         ->
 *     Run_Everywhere
 *         ->
 *     Run_Anywhere
 *         ->
 *     Run_Forever
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *     - number of services;
 *     - number of operations;
 *     - number of parameters;
 *     - number of messages;
 *     - number of protocols;
 *     - number of channels;
 *     - number of endpoints;
 *     - number of requirements;
 *     - number of capabilities;
 *     - number of constraints;
 *     - number of preferences;
 *     - number of policies;
 *     - number of lifecycle clauses;
 *     - number of metadata entries;
 *     - number of extensions;
 *     - qualified-name depth;
 *     - generic nesting depth.
 *
 * All source collections use ANTLR repetition operators.
 *
 * Practical limits belong to:
 *
 *     - parser resource policies;
 *     - compiler resource policies;
 *     - target capabilities;
 *     - deployment;
 *     - runtime;
 *     - resource management.
 *
 * They are NOT language limits.
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
 *     HTTP
 *     HTTP2
 *     HTTP3
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *
 * or any other finite protocol universe.
 *
 * Protocols are referenced through canonical qualified names.
 *
 * Future protocols therefore do not require a grammar rewrite merely because
 * a new transport or communication technology is invented.
 *
 * ============================================================================
 * MESSAGE MODEL
 * ============================================================================
 *
 * This grammar does not define message schemas.
 *
 * It references logical message types/names owned by the networking/data
 * grammar layers.
 *
 * A service may therefore express:
 *
 *     request MessageType
 *     response MessageType
 *     event MessageType
 *
 * without defining:
 *
 *     serialization;
 *     binary encoding;
 *     packet format;
 *     compression;
 *     wire layout.
 *
 * ============================================================================
 * TYPE MODEL
 * ============================================================================
 *
 * Service operation parameters and return values use the canonical type
 * grammar.
 *
 * This prevents the networking service grammar from creating a second type
 * system.
 *
 * ============================================================================
 * EXPRESSION MODEL
 * ============================================================================
 *
 * Generic semantic values use the canonical expression grammar.
 *
 * Expressions may appear in:
 *
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - policies;
 *     - metadata;
 *     - capability arguments;
 *     - lifecycle guards;
 *     - defaults;
 *     - operation contracts.
 *
 * This grammar does not redefine expression syntax.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A network service may expose or transport quantum-related operations.
 *
 * Examples include:
 *
 *     remote quantum execution;
 *     measurement services;
 *     distributed quantum coordination;
 *     quantum-classical communication;
 *     quantum resource services.
 *
 * None of the following are defined here:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     circuit representation
 *     calibration
 *     topology
 *     QEC code
 *     noise model
 *     ZQN fault model
 *
 * If an operation eventually contains quantum computation:
 *
 *     networking service syntax
 *          ->
 *     semantic analysis
 *          ->
 *     quantum lowering
 *          ->
 *     quantum::ir
 *
 * The grammar never constructs `quantum::ir`.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A service may expose a hardware or accelerator interface.
 *
 * It does not define:
 *
 *     - pins;
 *     - wires;
 *     - clocks;
 *     - FPGA routing;
 *     - ASIC cells;
 *     - physical buses;
 *     - fixed device counts;
 *     - physical addresses.
 *
 * Those belong to HDL and hardware grammars.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * A service may reference security capabilities or requirements using ordinary
 * qualified names.
 *
 * Examples:
 *
 *     requires security::authentication;
 *     requires security::confidentiality;
 *
 * Security implementation remains outside this grammar.
 *
 * No cryptographic algorithm is hard-coded here.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Requirements, constraints and preferences are deliberately separate.
 *
 * Requirement:
 *     mandatory semantic condition.
 *
 * Constraint:
 *     restricts legal realization.
 *
 * Preference:
 *     advisory optimization objective.
 *
 * Capability:
 *     describes a required/provided ability.
 *
 * None of these inherently selects a physical machine.
 *
 * ============================================================================
 * LIFECYCLE MODEL
 * ============================================================================
 *
 * Lifecycle syntax expresses semantic intent only.
 *
 * It does NOT execute:
 *
 *     start;
 *     stop;
 *     restart;
 *     migrate;
 *     destroy;
 *     failover.
 *
 * Runtime and resilience layers own those actions.
 *
 * ============================================================================
 * FAILURE / RESILIENCE BOUNDARY
 * ============================================================================
 *
 * The grammar may describe failure-related requirements such as:
 *
 *     requires reliability::durable;
 *
 * but does not implement:
 *
 *     retries;
 *     failover;
 *     checkpointing;
 *     rollback;
 *     recovery;
 *     quarantine;
 *     backend switching.
 *
 * Those belong to runtime/resilience.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime calls;
 *     - no hardware inspection;
 *     - no randomness;
 *     - no environment-dependent parsing.
 *
 * Equal token streams under the same grammar version produce the same parse
 * structure.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore MUST NOT define lexer rules.
 *
 * Structural service vocabulary is consumed through canonical lexical tokens
 * where the lexer already defines them, and otherwise through identifiers/
 * qualified names.
 *
 * This file must not introduce a second lexer authority.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     Core
 *         - attributes
 *         - visibility
 *         - identifier
 *         - qualifiedName
 *         - genericParameters
 *         - whereClause
 *
 *     Types
 *         - typeExpression
 *
 *     Expressions
 *         - expression
 *         - argumentList
 *
 * Networking grammars:
 *
 *     Protocols
 *         - protocol references
 *
 *     Channels
 *         - channel references
 *
 *     Endpoints
 *         - endpoint references
 *
 *     Messages
 *         - message references
 *
 * The networking service grammar MUST consume those contracts rather than
 * redefine their syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - declaration kind;
 *     - attributes;
 *     - visibility;
 *     - service name;
 *     - generic parameters;
 *     - service refinement;
 *     - operation ordering;
 *     - parameter ordering;
 *     - request/response ordering;
 *     - protocol references;
 *     - channel references;
 *     - endpoint references;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - policies;
 *     - lifecycle clauses;
 *     - metadata;
 *     - extensions;
 *     - expressions;
 *     - source spans.
 *
 * The AST must preserve source order.
 *
 * The parser does not resolve semantic identities.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - service-name uniqueness;
 *     - operation uniqueness;
 *     - parameter validity;
 *     - return validity;
 *     - protocol existence;
 *     - channel existence;
 *     - endpoint-role validity;
 *     - message compatibility;
 *     - capability satisfaction;
 *     - requirement satisfaction;
 *     - constraint validity;
 *     - preference validity;
 *     - security compatibility;
 *     - effect compatibility;
 *     - distributed-service compatibility;
 *     - target feasibility.
 *
 * The parser does none of these.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO service IR.
 *
 * Semantic analysis may lower the AST into the repository's canonical semantic
 * representation.
 *
 * Where an operation contains quantum computation:
 *
 *     service AST
 *          ->
 *     semantic model
 *          ->
 *     quantum semantic lowering
 *          ->
 *     quantum::ir
 *
 * Where an operation is classical:
 *
 *     service AST
 *          ->
 *     semantic model
 *          ->
 *     canonical/classical IR
 *
 * Where an operation targets hardware:
 *
 *     service AST
 *          ->
 *     semantic model
 *          ->
 *     HDL/hardware semantic representation
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use service semantics to determine:
 *
 *     - interface lowering;
 *     - message compatibility;
 *     - communication requirements;
 *     - effect requirements;
 *     - resource requirements;
 *     - target compatibility;
 *     - serialization strategy;
 *     - routing strategy;
 *     - scheduling constraints.
 *
 * The grammar itself must remain independent of those implementation choices.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns:
 *
 *     - service discovery;
 *     - binding;
 *     - dispatch;
 *     - transport;
 *     - endpoint resolution;
 *     - scheduling;
 *     - placement;
 *     - lifecycle execution;
 *     - failure recovery.
 *
 * This grammar defines none of those runtime mechanisms.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * This grammar must support:
 *
 *     - syntax highlighting;
 *     - formatting;
 *     - documentation generation;
 *     - symbol indexing;
 *     - service interface extraction;
 *     - API documentation;
 *     - diagnostics;
 *     - refactoring;
 *     - semantic navigation.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The existing networking aggregate currently exposes:
 *
 *     serviceDeclaration
 *
 * That generic rule MUST be migrated to this grammar.
 *
 * The new authoritative public rule is:
 *
 *     networkServiceConstruct
 *
 * Existing distributed:
 *
 *     distributedServiceDeclaration
 *
 * remains owned by:
 *
 *     grammar/distributed/services.g4
 *
 * This distinction is intentional and prevents a generic `serviceDeclaration`
 * rule from becoming ambiguous across language domains.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar NetworkingServices;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Networking aggregates should consume this rule.
 */

networkServiceConstruct
    : networkServiceDeclaration
    | networkServiceReference
    ;


/*
 * ============================================================================
 * NETWORK SERVICE DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     service_api;
 *
 *     service_api {
 *         ...
 *     }
 *
 *     service_api<T> {
 *         ...
 *     }
 *
 * The exact contextual spelling used by the language's service declaration
 * surface is resolved by the canonical lexer/parser composition.
 *
 * The rule itself remains independent of physical deployment.
 */

networkServiceDeclaration
    : attribute*
      visibility?
      networkServiceMarker
      identifier
      genericParameters?
      networkServiceRefinementClause?
      networkServiceWhereClause?
      networkServiceBody?
      SEMI?
    ;


/*
 * ============================================================================
 * SERVICE MARKER
 * ============================================================================
 *
 * The current lexer does not establish a separate canonical SERVICE token in
 * the same way it establishes core tokens such as FN, EFFECT, REQUIRES, etc.
 *
 * Therefore this grammar deliberately keeps service-marker recognition behind
 * a dedicated integration rule rather than inventing a new lexer authority.
 *
 * The networking aggregate/lexer integration MUST bind the canonical service
 * spelling to this rule.
 */

networkServiceMarker
    : identifier
    ;


/*
 * ============================================================================
 * SERVICE REFERENCE
 * ============================================================================
 *
 * A reference names a service owned elsewhere.
 *
 * This is NOT a declaration.
 */

networkServiceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SERVICE REFINEMENT
 * ============================================================================
 *
 * Refinement composes logical contracts.
 *
 * It does not mean:
 *
 *     class inheritance;
 *     process inheritance;
 *     runtime implementation inheritance.
 */

networkServiceRefinementClause
    : EXTENDS
      qualifiedName
      (COMMA qualifiedName)*
    ;


/*
 * ============================================================================
 * SERVICE WHERE CLAUSE
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
 */

networkServiceBody
    : LBRACE
      networkServiceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * SERVICE MEMBERS
 * ============================================================================
 */

networkServiceMember
    : networkServiceOperationDeclaration
    | networkServiceProtocolReference
    | networkServiceChannelReference
    | networkServiceEndpointReference
    | networkServiceRequestDeclaration
    | networkServiceResponseDeclaration
    | networkServiceEventDeclaration
    | networkServiceRequirementDeclaration
    | networkServiceCapabilityDeclaration
    | networkServiceConstraintDeclaration
    | networkServicePreferenceDeclaration
    | networkServicePolicyDeclaration
    | networkServiceLifecycleDeclaration
    | networkServicePropertyDeclaration
    | networkServiceContractDeclaration
    | networkServiceExtensionDeclaration
    ;


/*
 * ============================================================================
 * SERVICE OPERATIONS
 * ============================================================================
 *
 * Operations describe logical callable communication contracts.
 *
 * They do not prescribe:
 *
 *     transport;
 *     socket;
 *     thread;
 *     CPU;
 *     machine;
 *     node;
 *     accelerator.
 */

networkServiceOperationDeclaration
    : attribute*
      ASYNC?
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
    : identifier
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
 * OPERATION RETURN
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
 */

networkServiceOperationClause
    : networkServiceRequestReference
    | networkServiceResponseReference
    | networkServiceProtocolReference
    | networkServiceChannelReference
    | networkServiceRequirementDeclaration
    | networkServiceCapabilityDeclaration
    | networkServiceContractDeclaration
    ;


/*
 * ============================================================================
 * OPERATION BODY
 * ============================================================================
 *
 * The body is optional because a network service grammar may describe either:
 *
 *     - a pure interface;
 *     - a source-level service implementation contract.
 *
 * Actual execution remains outside this grammar.
 */

networkServiceOperationBody
    : blockExpression
    ;


/*
 * ============================================================================
 * REQUEST DECLARATION
 * ============================================================================
 *
 * A request references an existing logical message/type.
 *
 * Message schema ownership remains elsewhere.
 */

networkServiceRequestDeclaration
    : attribute*
      networkServiceRequestMarker
      qualifiedName
      networkServiceRequestOptions*
      SEMI
    ;


networkServiceRequestMarker
    : identifier
    ;


networkServiceRequestOptions
    : networkServiceMessageTypeClause
    | networkServiceChannelReference
    | networkServiceProtocolReference
    ;


networkServiceRequestReference
    : networkServiceRequestMarker
      qualifiedName
    ;


networkServiceMessageTypeClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * RESPONSE DECLARATION
 * ============================================================================
 */

networkServiceResponseDeclaration
    : attribute*
      networkServiceResponseMarker
      qualifiedName
      networkServiceResponseOptions*
      SEMI
    ;


networkServiceResponseMarker
    : identifier
    ;


networkServiceResponseOptions
    : networkServiceMessageTypeClause
    | networkServiceChannelReference
    | networkServiceProtocolReference
    ;


networkServiceResponseReference
    : networkServiceResponseMarker
      qualifiedName
    ;


/*
 * ============================================================================
 * EVENT DECLARATION
 * ============================================================================
 *
 * Events are outbound communication contracts.
 *
 * They do not imply:
 *
 *     multicast;
 *     broadcast;
 *     queueing;
 *     persistence;
 *     transport.
 *
 * Those are semantic/runtime properties.
 */

networkServiceEventDeclaration
    : attribute*
      networkServiceEventMarker
      identifier
      networkServiceEventTypeClause?
      networkServiceEventOption*
      SEMI
    ;


networkServiceEventMarker
    : identifier
    ;


networkServiceEventTypeClause
    : COLON
      typeExpression
    ;


networkServiceEventOption
    : networkServiceChannelReference
    | networkServiceProtocolReference
    | networkServiceRequirementDeclaration
    | networkServicePropertyDeclaration
    ;


/*
 * ============================================================================
 * PROTOCOL REFERENCES
 * ============================================================================
 *
 * Protocol definitions are owned by networking/protocols.g4.
 *
 * This grammar only references them.
 */

networkServiceProtocolReference
    : networkServiceProtocolMarker
      qualifiedName
      networkServiceReferenceValue?
      SEMI?
    ;


networkServiceProtocolMarker
    : identifier
    ;


networkServiceReferenceValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCES
 * ============================================================================
 *
 * Channel definitions are owned by networking/channels.g4.
 *
 * This grammar does not create channels.
 */

networkServiceChannelReference
    : networkServiceChannelMarker
      qualifiedName
      networkServiceReferenceValue?
      SEMI?
    ;


networkServiceChannelMarker
    : identifier
    ;


/*
 * ============================================================================
 * ENDPOINT REFERENCES
 * ============================================================================
 *
 * Endpoint definitions are owned by networking/endpoints.g4.
 *
 * An endpoint reference is logical.
 *
 * It does not encode an address.
 */

networkServiceEndpointReference
    : networkServiceEndpointMarker
      qualifiedName
      networkServiceReferenceValue?
      SEMI?
    ;


networkServiceEndpointMarker
    : identifier
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 */

networkServiceRequirementDeclaration
    : attribute*
      networkServiceRequirementMarker
      expression
      SEMI
    ;


networkServiceRequirementMarker
    : REQUIRES
    ;


/*
 * ============================================================================
 * CAPABILITIES
 * ============================================================================
 *
 * Capability names remain open-world qualified names.
 */

networkServiceCapabilityDeclaration
    : attribute*
      networkServiceCapabilityMarker
      qualifiedName
      networkServiceReferenceValue?
      SEMI
    ;


networkServiceCapabilityMarker
    : identifier
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict legal realization.
 */

networkServiceConstraintDeclaration
    : attribute*
      networkServiceConstraintMarker
      expression
      SEMI
    ;


networkServiceConstraintMarker
    : identifier
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * They must never silently become hard requirements.
 */

networkServicePreferenceDeclaration
    : attribute*
      networkServicePreferenceMarker
      expression
      SEMI
    ;


networkServicePreferenceMarker
    : identifier
    ;


/*
 * ============================================================================
 * POLICIES
 * ============================================================================
 *
 * Policies express semantic policy intent.
 *
 * The grammar does not implement policy enforcement.
 */

networkServicePolicyDeclaration
    : attribute*
      networkServicePolicyMarker
      identifier?
      networkServicePolicyBody
    ;


networkServicePolicyMarker
    : identifier
    ;


networkServicePolicyBody
    : LBRACE
      networkServicePolicyMember*
      RBRACE
    ;


networkServicePolicyMember
    : networkServiceRequirementDeclaration
    | networkServiceConstraintDeclaration
    | networkServicePreferenceDeclaration
    | networkServicePropertyDeclaration
    | networkServiceExtensionDeclaration
    ;


/*
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 *
 * Lifecycle is semantic intent.
 *
 * Runtime owns actual process/service lifecycle execution.
 */

networkServiceLifecycleDeclaration
    : attribute*
      networkServiceLifecycleMarker
      identifier?
      networkServiceLifecycleBody?
      SEMI?
    ;


networkServiceLifecycleMarker
    : identifier
    ;


networkServiceLifecycleBody
    : LBRACE
      networkServiceLifecycleMember*
      RBRACE
    ;


networkServiceLifecycleMember
    : networkServiceLifecycleClause
    | networkServiceRequirementDeclaration
    | networkServiceConstraintDeclaration
    | networkServicePropertyDeclaration
    | networkServiceExtensionDeclaration
    ;


networkServiceLifecycleClause
    : identifier
      (COLON expression)?
      SEMI
    ;


/*
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 *
 * Contracts express semantic guarantees.
 *
 * They are not executable implementations.
 */

networkServiceContractDeclaration
    : attribute*
      networkServiceContractMarker
      identifier?
      networkServiceContractBody?
      SEMI?
    ;


networkServiceContractMarker
    : identifier
    ;


networkServiceContractBody
    : LBRACE
      networkServiceContractMember*
      RBRACE
    ;


networkServiceContractMember
    : networkServicePrecondition
    | networkServicePostcondition
    | networkServiceInvariant
    | networkServiceGuarantee
    | networkServiceRequirementDeclaration
    | networkServiceConstraintDeclaration
    | networkServicePropertyDeclaration
    | networkServiceExtensionDeclaration
    ;


networkServicePrecondition
    : identifier
      expression
      SEMI
    ;


networkServicePostcondition
    : identifier
      expression
      SEMI
    ;


networkServiceInvariant
    : identifier
      expression
      SEMI
    ;


networkServiceGuarantee
    : identifier
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 *
 * Properties are generic extensibility points.
 *
 * They do not create a fixed universe of networking concepts.
 */

networkServicePropertyDeclaration
    : attribute*
      networkServicePropertyMarker
      identifier
      networkServicePropertyTypeClause?
      networkServicePropertyInitializer?
      SEMI
    ;


networkServicePropertyMarker
    : identifier
    ;


networkServicePropertyTypeClause
    : COLON
      typeExpression
    ;


networkServicePropertyInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * EXTENSIONS
 * ============================================================================
 *
 * Extension syntax allows future networking/domain dialects to attach
 * information without modifying this core service grammar.
 *
 * Semantic validation determines whether an extension is valid.
 */

networkServiceExtensionDeclaration
    : attribute*
      networkServiceExtensionMarker
      qualifiedName
      networkServiceExtensionBody?
      SEMI?
    ;


networkServiceExtensionMarker
    : identifier
    ;


networkServiceExtensionBody
    : LBRACE
      networkServiceExtensionMember*
      RBRACE
    ;


networkServiceExtensionMember
    : networkServicePropertyDeclaration
    | networkServiceRequirementDeclaration
    | networkServiceConstraintDeclaration
    | networkServicePreferenceDeclaration
    | networkServiceExtensionDeclaration
    ;


/*
 * ============================================================================
 * PUBLIC SERVICE REFERENCE HELPERS
 * ============================================================================
 *
 * These rules provide stable semantic categories without introducing
 * additional type systems.
 */

networkServiceMessageReference
    : qualifiedName
    ;


networkServiceProtocolName
    : qualifiedName
    ;


networkServiceChannelName
    : qualifiedName
    ;


networkServiceEndpointName
    : qualifiedName
    ;


/*
 * ============================================================================
 * END
 * ============================================================================
 */