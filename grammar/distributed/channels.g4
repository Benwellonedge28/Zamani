/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * Production Distributed Channel Grammar
 * ============================================================================
 *
 * File:
 *     grammar/distributed/channels.g4
 *
 * Grammar:
 *     DistributedChannels
 *
 * Status:
 *     PRODUCTION
 *
 * Purpose:
 *     Define the source-level structure of logical distributed channels.
 *
 * This grammar describes channel intent and channel-local structure only.
 *
 * It does NOT define:
 *
 *     - transport protocols;
 *     - sockets;
 *     - network topology;
 *     - node allocation;
 *     - process allocation;
 *     - actor implementation;
 *     - scheduler implementation;
 *     - placement algorithms;
 *     - routing algorithms;
 *     - serialization;
 *     - encryption;
 *     - authentication;
 *     - replication;
 *     - consensus;
 *     - fault-tolerance algorithms;
 *     - runtime queues;
 *     - hardware resources;
 *     - physical memory;
 *     - quantum topology;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - target-specific limits.
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust is required by this grammar.
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
 *     ZamaniParser
 *          |
 *          v
 *     Distributed
 *          |
 *          v
 *     DistributedChannels
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          |
 *          v
 *     distributed semantic model
 *          |
 *          +--> classical execution
 *          +--> quantum semantic model
 *          +--> HDL/hardware realization
 *          +--> AI/data computation
 *          +--> networking
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> Classical IR
 *          +--> quantum::ir
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     placement / routing / scheduling
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed channel declarations;
 *     - distributed channel payload types;
 *     - distributed channel initializers;
 *     - channel-local endpoint declarations;
 *     - channel-local configuration;
 *     - channel-local policy clauses;
 *     - channel-local requirements;
 *     - channel-local selection;
 *     - channel-local communication adapters;
 *     - channel compatibility entry points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic channels;
 *     - generic concurrency;
 *     - generic communication;
 *     - distributed messages;
 *     - network channels;
 *     - actors;
 *     - services;
 *     - nodes;
 *     - placement;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - resource resolution;
 *     - capability resolution;
 *     - effect semantics;
 *     - policies as a universal semantic subsystem;
 *     - message schemas;
 *     - serialization;
 *     - execution;
 *     - runtime behavior.
 *
 * ============================================================================
 * RELATED GRAMMARS
 * ============================================================================
 *
 * Generic channels:
 *
 *     grammar/concurrency/channels.g4
 *
 * Generic distributed communication:
 *
 *     grammar/distributed/communication.g4
 *
 * Distributed messages:
 *
 *     grammar/distributed/messaging.g4
 *
 * Network-specific channels:
 *
 *     grammar/networking/channels.g4
 *
 * Distributed composition:
 *
 *     grammar/distributed/distributed.g4
 *
 * The relationship is:
 *
 *     channel intent
 *          |
 *          +--> local concurrency realization
 *          |
 *          +--> distributed realization
 *                    |
 *                    +--> communication
 *                    |
 *                    +--> networking
 *                    |
 *                    +--> future communication substrate
 *
 * There must be one semantic channel model downstream even though multiple
 * source-level domains consume channel syntax.
 *
 * ============================================================================
 * IMPORTANT SEPARATION
 * ============================================================================
 *
 * `grammar/distributed/communication.g4` owns generic distributed
 * communication operation syntax.
 *
 * Therefore this grammar does NOT redefine a second generic communication
 * language.
 *
 * For example:
 *
 *     distributed::send(channel, value, destination);
 *     distributed::receive(channel);
 *
 * remain structurally owned by distributed communication.
 *
 * This file provides channel-specific adapters where channel context is
 * required.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * Stable public rules:
 *
 *     distributedChannelDeclaration
 *     distributedChannelStatement
 *     distributedChannelExpression
 *     distributedChannelMember
 *     distributedChannelInvocation
 *     distributedChannelSelectExpression
 *     distributedChannelCompatibility
 *     distributedChannelConstruct
 *
 * Parent grammars should consume these public rules rather than internal
 * implementation rules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * A declaration conceptually maps to the domain-neutral declaration model:
 *
 *     declaration
 *         name
 *         type
 *         initializer
 *         members
 *         attributes/metadata
 *         source span
 *
 * Channel semantics conceptually contain:
 *
 *     channel identity
 *     payload type
 *     initializer
 *     endpoints
 *     policies
 *     requirements
 *     selection semantics
 *     source provenance
 *
 * This grammar does not define Rust AST structures.
 *
 * The frontend AST subsystem remains the AST owner.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *     - whether a declaration is legal;
 *     - whether the channel name is valid;
 *     - whether the payload type is valid;
 *     - whether endpoint types are compatible;
 *     - whether an operation is valid;
 *     - whether a channel may cross execution domains;
 *     - whether a channel may carry quantum values;
 *     - whether classical/quantum conversion is legal;
 *     - whether ownership/borrowing rules are satisfied;
 *     - whether effects are permitted;
 *     - whether capabilities exist;
 *     - whether resource requirements are satisfiable;
 *     - whether policies permit realization;
 *     - whether the requested realization is deterministic;
 *     - whether the target supports the required semantics.
 *
 * Resource failure is NOT a syntax failure.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Channel payloads and endpoint payloads use canonical `typeExpression`.
 *
 * This grammar does not define a second channel type system.
 *
 * A channel may therefore carry values whose semantic types belong to:
 *
 *     classical
 *     quantum
 *     tensor/data
 *     AI/model
 *     distributed
 *     HDL/hardware
 *     hybrid
 *     user-defined
 *     future domains
 *
 * Semantic validation determines whether a particular transfer is legal.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar does not assign effects.
 *
 * Downstream semantic analysis may associate channel operations with effects
 * such as:
 *
 *     io
 *     network
 *     distributed
 *     mutation
 *     foreign
 *     measurement
 *     quantum
 *     randomness
 *     simulation
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic information.
 *
 * Examples include:
 *
 *     capability("distributed.communication")
 *     capability("network.messaging")
 *     capability("quantum.communication")
 *
 * This grammar only preserves the source expression representing such
 * requirements.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * No physical capacity is represented as a grammar constant.
 *
 * Resource requirements may be expressed semantically through ordinary
 * Zamani expressions.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *     requires capability("distributed.communication");
 *     requires topology(required_topology);
 *
 * Whether those requirements can be satisfied belongs downstream.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Channel-local configuration is represented as declarative source intent.
 *
 * The grammar does not decide policy meaning.
 *
 * Examples:
 *
 *     distributed::ordering = ordering_policy;
 *     distributed::delivery = delivery_policy;
 *     distributed::reliability = reliability_policy;
 *     distributed::security = security_policy;
 *     distributed::capacity = capacity_expression;
 *
 * Policy semantics belong to the policy/semantic subsystems.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Frontend processing must preserve source locations for:
 *
 *     - channel declaration;
 *     - channel name;
 *     - payload type;
 *     - initializer;
 *     - endpoint;
 *     - endpoint type;
 *     - policy;
 *     - requirement;
 *     - select;
 *     - select arm;
 *     - operation;
 *     - operation arguments.
 *
 * Provenance generation belongs downstream.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * THIS FILE CREATES NO CHANNEL-SPECIFIC IR.
 *
 * Channel semantics must lower into the repository's existing canonical
 * semantic/IR architecture.
 *
 * Possible downstream realizations include:
 *
 *     Classical IR
 *     quantum::ir
 *     distributed execution metadata
 *     networking realization metadata
 *     HDL/hardware realization metadata
 *
 * No:
 *
 *     DistributedChannelIR
 *     NetworkChannelIR
 *     QuantumChannelIR
 *
 * is introduced by this grammar.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A channel may carry quantum-domain values where the semantic model permits
 * such a transfer.
 *
 * This grammar does not define:
 *
 *     qubit allocation;
 *     gate operations;
 *     physical links;
 *     entanglement routing;
 *     calibration;
 *     QEC.
 *
 * Quantum semantic information must eventually cross:
 *
 *     quantum::ir
 *
 * through the existing quantum architecture.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A distributed channel may eventually be realized using:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     hardware fabric
 *     DMA
 *     memory subsystem
 *     network fabric
 *     future substrate
 *
 * None of these are represented as universal grammar-level implementations.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar contains NO finite language-level limits for:
 *
 *     channels;
 *     endpoints;
 *     participants;
 *     operations;
 *     select arms;
 *     declarations;
 *     nesting;
 *     expression size;
 *     payload type complexity;
 *     qualified-name depth.
 *
 * No language constants such as:
 *
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_MESSAGES
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *     MAX_TOPOLOGY_SIZE
 *
 * are represented.
 *
 * `*` and `+` represent language cardinality constrained only by the parser
 * implementation and available compilation resources.
 *
 * Actual physical limits belong to:
 *
 *     resource analysis;
 *     capability negotiation;
 *     execution planning;
 *     runtime;
 *     target hardware.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A distributed channel describes logical communication intent.
 *
 * The same source can therefore be considered for:
 *
 *     one execution context;
 *     multiple cores;
 *     multiple processes;
 *     multiple machines;
 *     heterogeneous accelerators;
 *     quantum/classical systems;
 *     HPC;
 *     clusters;
 *     cloud execution;
 *     edge execution;
 *     future computing substrates.
 *
 * Changing target scale must not require changing channel grammar semantics.
 *
 * ============================================================================
 * TOKEN CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer.
 *
 * Structural tokens intentionally used here include:
 *
 *     DISTRIBUTED
 *     CHANNEL
 *     ENDPOINT
 *     SELECT
 *     CASE
 *     DEFAULT
 *     INPUT
 *     OUTPUT
 *     INOUT
 *     CAPACITY
 *     RELIABILITY
 *     AVAILABILITY
 *     REDUCE
 *     DOUBLE_COLON
 *     COLON
 *     ASSIGN
 *     COMMA
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *     FAT_ARROW
 *
 * No lexer rules are defined here.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Names:
 *
 *     identifier
 *     qualifiedName
 *
 * Types:
 *
 *     typeExpression
 *
 * Expressions:
 *
 *     expression
 *     expressionList
 *     optionalExpressionList
 *
 * Communication:
 *
 *     distributedCommunication
 *
 * This grammar must not import its parent `Distributed` grammar.
 *
 * Dependency direction:
 *
 *     Names
 *       |
 *     Types
 *       |
 *     Expressions
 *       |
 *     Communication
 *       |
 *     DistributedChannels
 *       |
 *     Distributed
 *       |
 *     ZamaniParser
 *
 * ============================================================================
 */

parser grammar DistributedChannels;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions,
    Communication
;


/*
 * ============================================================================
 * 1. DISTRIBUTED CHANNEL DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     distributed::channel data: Message;
 *
 * Optional initializer:
 *
 *     distributed::channel data: Message = initial_channel();
 *
 * Optional body:
 *
 *     distributed::channel data: Message {
 *         ...
 *     }
 *
 * The explicit `distributed::channel` prefix is intentional.
 *
 * It prevents this grammar from accepting arbitrary:
 *
 *     name value: Type;
 *
 * declarations and accidentally stealing declarations belonging to other
 * distributed grammars.
 */

distributedChannelDeclaration
    : DISTRIBUTED
      DOUBLE_COLON
      CHANNEL
      identifier
      COLON
      typeExpression
      distributedChannelInitializer?
      distributedChannelBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. INITIALIZER
 * ============================================================================
 */

distributedChannelInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 3. CHANNEL BODY
 * ============================================================================
 *
 * A channel body contains channel-local declarations and declarative clauses.
 *
 * Executable distributed communication remains owned by Communication.
 */

distributedChannelBody
    : LBRACE
      distributedChannelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. CHANNEL MEMBERS
 * ============================================================================
 */

distributedChannelMember
    : distributedChannelEndpointDeclaration
    | distributedChannelPolicyStatement
    | distributedChannelRequirementStatement
    | distributedChannelConfigurationStatement
    | distributedChannelSelectStatement
    ;


/*
 * ============================================================================
 * 5. ENDPOINT DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     endpoint input: Message;
 *     endpoint output: Result;
 *     endpoint inout: Payload;
 *
 * `input`, `output`, and `inout` are reserved lexer tokens, so they cannot
 * be handled only through `identifier`.
 *
 * They are therefore explicitly admitted as endpoint names.
 *
 * Their semantic role is determined downstream.
 */

distributedChannelEndpointDeclaration
    : ENDPOINT
      distributedChannelEndpointName
      COLON
      typeExpression
      distributedChannelInitializer?
      SEMICOLON?
    ;


distributedChannelEndpointName
    : identifier
    | INPUT
    | OUTPUT
    | INOUT
    ;


/*
 * ============================================================================
 * 6. CHANNEL CONFIGURATION
 * ============================================================================
 *
 * Configuration is declarative.
 *
 * Examples:
 *
 *     distributed::capacity = capacity_expression;
 *     distributed::reliability = reliability_policy;
 *     distributed::availability = availability_policy;
 *     distributed::ordering = ordering_policy;
 *     distributed::delivery = delivery_policy;
 *     distributed::security = security_policy;
 *
 * Only the structural prefix is fixed.
 *
 * The clause name remains open to future identifiers.
 *
 * `capacity`, `reliability`, and `availability` are already reserved tokens,
 * so they are explicitly accepted here.
 */

distributedChannelConfigurationStatement
    : distributedChannelClauseName
      ASSIGN
      expression
      SEMICOLON?
    ;


distributedChannelClauseName
    : distributedChannelNamespacePrefix
      distributedChannelConfigurationName
    ;


distributedChannelConfigurationName
    : identifier
    | CAPACITY
    | RELIABILITY
    | AVAILABILITY
    ;


distributedChannelNamespacePrefix
    : DISTRIBUTED
      DOUBLE_COLON
    ;


/*
 * ============================================================================
 * 7. POLICY
 * ============================================================================
 *
 * Policy statements use the same structural configuration boundary.
 *
 * The semantic policy subsystem determines whether the assigned expression
 * represents:
 *
 *     ordering;
 *     delivery;
 *     reliability;
 *     consistency;
 *     security;
 *     locality;
 *     confidentiality;
 *     integrity;
 *     durability;
 *     or a future policy dimension.
 */

distributedChannelPolicyStatement
    : distributedChannelNamespacePrefix
      distributedChannelPolicyName
      ASSIGN
      expression
      SEMICOLON?
    ;


distributedChannelPolicyName
    : identifier
    | RELIABILITY
    | AVAILABILITY
    | CAPACITY
    ;


/*
 * ============================================================================
 * 8. REQUIREMENT
 * ============================================================================
 *
 * Requirement syntax is deliberately expression-based.
 *
 * The requirement expression can represent:
 *
 *     capabilities;
 *     resources;
 *     topology;
 *     latency;
 *     bandwidth;
 *     availability;
 *     locality;
 *     reliability;
 *     security;
 *     transferability;
 *     future resource dimensions.
 *
 * No finite resource catalogue is encoded.
 */

distributedChannelRequirementStatement
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. CHANNEL EXPRESSION
 * ============================================================================
 *
 * The expression-level boundary contains channel-local selection and the
 * compatibility invocation adapter.
 */

distributedChannelExpression
    : distributedChannelSelectExpression
    | distributedChannelInvocation
    ;


/*
 * ============================================================================
 * 10. DISTRIBUTED CHANNEL INVOCATION
 * ============================================================================
 *
 * Generic distributed communication operation syntax remains owned by:
 *
 *     grammar/distributed/communication.g4
 *
 * This adapter gives channel-aware semantic tooling a stable rule without
 * redefining communication syntax.
 *
 * Examples:
 *
 *     distributed::send(channel, value, destination);
 *     distributed::receive(channel);
 *     distributed::broadcast(value, group);
 *     distributed::scatter(value, group);
 *     distributed::gather(group);
 *     distributed::reduce(value, operation, group);
 *
 * The actual communication grammar remains canonical.
 */

distributedChannelInvocation
    : distributedCommunication
    ;


/*
 * ============================================================================
 * 11. CHANNEL STATEMENT
 * ============================================================================
 *
 * This rule is intentionally limited to channel-local constructs.
 *
 * Generic communication operations remain owned by `Communication`.
 */

distributedChannelStatement
    : distributedChannelPolicyStatement
    | distributedChannelRequirementStatement
    | distributedChannelConfigurationStatement
    | distributedChannelSelectStatement
    ;


/*
 * ============================================================================
 * 12. SELECT
 * ============================================================================
 *
 * Canonical distributed selection:
 *
 *     distributed::select {
 *         distributed::receive(input) => process(input);
 *         distributed::send(output, value) => continue_work();
 *     }
 *
 * The selector itself is structurally fixed.
 *
 * The communication operations remain open-world through Communication.
 */

distributedChannelSelectExpression
    : DISTRIBUTED
      DOUBLE_COLON
      SELECT
      LBRACE
      distributedChannelSelectArm*
      distributedChannelSelectDefaultArm?
      RBRACE
    ;


distributedChannelSelectStatement
    : distributedChannelSelectExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 13. SELECT ARM
 * ============================================================================
 *
 * The arm guard is a communication construct.
 *
 * The result/body is ordinary Zamani expression syntax.
 */

distributedChannelSelectArm
    : distributedChannelInvocation
      FAT_ARROW
      distributedChannelSelectBody
      COMMA?
      SEMICOLON?
    ;


distributedChannelSelectDefaultArm
    : DISTRIBUTED
      DOUBLE_COLON
      DEFAULT
      FAT_ARROW
      distributedChannelSelectBody
      COMMA?
      SEMICOLON?
    ;


distributedChannelSelectBody
    : expression
    | distributedChannelSelectBlock
    ;


distributedChannelSelectBlock
    : LBRACE
      distributedChannelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 14. OPERATION COMPATIBILITY ADAPTERS
 * ============================================================================
 *
 * These names are retained for semantic tooling and compatibility with the
 * previous grammar surface.
 *
 * They DO NOT define independent communication syntax.
 *
 * All of them delegate to the canonical Communication grammar.
 */

distributedChannelSend
    : distributedChannelInvocation
    ;


distributedChannelReceive
    : distributedChannelInvocation
    ;


distributedChannelBroadcast
    : distributedChannelInvocation
    ;


distributedChannelScatter
    : distributedChannelInvocation
    ;


distributedChannelGather
    : distributedChannelInvocation
    ;


distributedChannelReduce
    : distributedChannelInvocation
    ;


distributedChannelRequest
    : distributedChannelInvocation
    ;


distributedChannelResponse
    : distributedChannelInvocation
    ;


distributedChannelStream
    : distributedChannelInvocation
    ;


distributedChannelCollective
    : distributedChannelInvocation
    ;


/*
 * ============================================================================
 * 15. CHANNEL REFERENCE
 * ============================================================================
 *
 * A channel reference is an ordinary expression.
 *
 * Semantic analysis determines whether the resulting value denotes a channel.
 */

distributedChannelReference
    : expression
    ;


/*
 * ============================================================================
 * 16. CHANNEL TYPE
 * ============================================================================
 *
 * The channel payload remains a canonical Zamani type.
 *
 * No channel-specific type system is introduced.
 */

distributedChannelType
    : typeExpression
    ;


distributedChannelPayloadType
    : typeExpression
    ;


/*
 * ============================================================================
 * 17. CHANNEL NAME
 * ============================================================================
 */

distributedChannelName
    : identifier
    ;


distributedChannelQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 18. TARGET / PARTICIPANT EXPRESSIONS
 * ============================================================================
 *
 * Participants are ordinary expressions.
 *
 * They may represent:
 *
 *     one participant;
 *     a collection;
 *     an actor;
 *     a service;
 *     a region;
 *     a dynamically computed group;
 *     a future distributed abstraction.
 *
 * No finite participant catalogue is encoded.
 */

distributedChannelTargetList
    : expressionList
    ;


optionalDistributedChannelTargetList
    : optionalExpressionList
    ;


/*
 * ============================================================================
 * 19. COMPATIBILITY ADAPTER
 * ============================================================================
 *
 * Existing consumers that need a single distributed-channel construct can use
 * this rule.
 */

distributedChannelCompatibility
    : distributedChannelDeclaration
    | distributedChannelStatement
    | distributedChannelExpression
    ;


/*
 * ============================================================================
 * 20. COMPLETE CHANNEL CONSTRUCT
 * ============================================================================
 *
 * This is the preferred leaf-level integration boundary.
 */

distributedChannelConstruct
    : distributedChannelDeclaration
    | distributedChannelStatement
    | distributedChannelExpression
    ;


/*
 * ============================================================================
 * 21. CHANNEL FRAGMENT
 * ============================================================================
 *
 * Reusable grammar fragment.
 *
 * It is not a program root.
 */

distributedChannelFragment
    : distributedChannelConstruct*
    ;


/*
 * ============================================================================
 * 22. PUBLIC DOMAIN BLOCK
 * ============================================================================
 */

distributedChannelDomainBlock
    : LBRACE
      distributedChannelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 23. DOMAIN ADAPTER
 * ============================================================================
 */

distributedChannelDomain
    : distributedChannelDeclaration
    | distributedChannelStatement
    | distributedChannelExpression
    ;


/*
 * ============================================================================
 * 24. AST TRACEABILITY
 * ============================================================================
 *
 *     distributedChannelDeclaration
 *              |
 *              v
 *     domain-neutral declaration
 *              |
 *              +--> name
 *              +--> payload type
 *              +--> initializer
 *              +--> members
 *              +--> source span
 *              |
 *              v
 *     semantic channel model
 *              |
 *              +--> effects
 *              +--> capabilities
 *              +--> resources
 *              +--> contracts
 *              +--> policies
 *              +--> provenance
 *              |
 *              v
 *     canonical semantic representation
 *
 * No channel-specific AST or IR is required.
 *
 * ============================================================================
 * 25. SEMANTIC DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level diagnostics:
 *
 *     - missing distributed namespace;
 *     - missing channel keyword;
 *     - missing channel name;
 *     - missing payload type;
 *     - malformed initializer;
 *     - malformed channel body;
 *     - malformed endpoint;
 *     - malformed configuration;
 *     - malformed requirement;
 *     - malformed select;
 *     - missing FAT_ARROW;
 *     - malformed expression.
 *
 * Semantic diagnostics:
 *
 *     - unresolved channel;
 *     - invalid channel kind;
 *     - invalid payload type;
 *     - incompatible endpoint;
 *     - invalid communication operation;
 *     - unsupported communication capability;
 *     - unsatisfied resource requirement;
 *     - policy violation;
 *     - ownership violation;
 *     - effect violation;
 *     - security violation;
 *     - invalid quantum/classical transfer;
 *     - invalid target realization.
 *
 * Resource exhaustion must never be converted into a parser error.
 *
 * ============================================================================
 * 26. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - lexer configuration;
 *     - parser grammar;
 *     - selected language compatibility version.
 *
 * Parsing must not inspect:
 *
 *     - hardware;
 *     - filesystem state;
 *     - network state;
 *     - runtime state;
 *     - scheduler state;
 *     - resource availability;
 *     - target availability;
 *     - randomness;
 *     - wall-clock state.
 *
 * ============================================================================
 * 27. SECURITY
 * ============================================================================
 *
 * Parsing never executes:
 *
 *     - channel operations;
 *     - expressions;
 *     - policies;
 *     - requirements;
 *     - capability queries;
 *     - resource queries;
 *     - target expressions.
 *
 * The grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no callbacks;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access.
 *
 * ============================================================================
 * 28. PERFORMANCE
 * ============================================================================
 *
 * The grammar avoids semantic probing during parsing.
 *
 * Open-world communication semantics are delegated to `Communication`.
 *
 * Channel structure is kept explicit so distributed declaration prediction does
 * not need to speculate over arbitrary qualified-name declarations.
 *
 * No universal finite cardinality is encoded.
 *
 * ============================================================================
 * 29. COMPATIBILITY
 * ============================================================================
 *
 * Existing source forms remain structurally supported:
 *
 *     distributed::channel data: Message;
 *
 *     distributed::channel data: Message = initial_channel();
 *
 *     distributed::channel data: Message {
 *         distributed::capacity = capacity_expression;
 *         distributed::reliability = reliability_policy;
 *     }
 *
 *     distributed::select {
 *         distributed::receive(input) => process(input);
 *         distributed::default => fallback();
 *     }
 *
 * Existing semantic adapter rule names such as:
 *
 *     distributedChannelSend
 *     distributedChannelReceive
 *     distributedChannelBroadcast
 *     distributedChannelScatter
 *     distributedChannelGather
 *     distributedChannelReduce
 *
 * remain available but delegate to the canonical communication grammar.
 *
 * ============================================================================
 * 30. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST demonstrate that the grammar does not impose artificial limits
 * on:
 *
 *     - channel declarations;
 *     - endpoint declarations;
 *     - channel members;
 *     - select arms;
 *     - nested channel bodies;
 *     - expression depth;
 *     - qualified-name depth;
 *     - payload type complexity;
 *     - number of communication constructs.
 *
 * Test sizes are generated according to available test resources rather than
 * encoded into the grammar.
 *
 * ============================================================================
 * 31. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required integration coverage includes:
 *
 *     classical payload
 *     tensor/data payload
 *     AI/model payload
 *     quantum-domain payload
 *     hybrid payload
 *     hardware-related payload
 *     distributed actors
 *     distributed services
 *     networking realization
 *     resource requirements
 *     capability requirements
 *     policies
 *     contracts
 *     provenance
 *     simulation
 *     adaptive execution
 *
 * The grammar only parses the structural portion.
 *
 * ============================================================================
 * 32. POCO-REAF TEST CONTRACT
 * ============================================================================
 *
 * The same source syntax must remain parseable without modification when the
 * downstream target changes between:
 *
 *     tiny execution context
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed system
 *     cloud
 *     future substrate
 *
 * Target feasibility belongs downstream.
 *
 * ============================================================================
 * 33. RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust source.
 *
 * Generated parser consumers are intended for:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * The repository implementation must use safe Rust.
 *
 * No:
 *
 *     unsafe fn
 *     unsafe block
 *     unsafe trait
 *
 * is required by this grammar.
 *
 * This grammar itself cannot enforce implementation-wide Rust safety; CI and
 * compiler configuration must enforce that repository invariant.
 *
 * ============================================================================
 * 34. PARENT INTEGRATION
 * ============================================================================
 *
 * `grammar/distributed/distributed.g4` already imports:
 *
 *     DistributedChannels
 *
 * and already exposes:
 *
 *     distributedChannelDeclaration
 *
 * Therefore no additional import is required in the parent grammar.
 *
 * Its existing composition:
 *
 *     distributedDeclaration
 *         |
 *         +--> distributedChannelDeclaration
 *
 * remains correct.
 *
 * Generic communication remains:
 *
 *     distributedCommunication
 *
 * and remains owned by:
 *
 *     grammar/distributed/communication.g4
 *
 * This separation is deliberate.
 *
 * ============================================================================
 * 35. CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * `grammar/concurrency/channels.g4` remains the generic channel grammar.
 *
 * It owns:
 *
 *     channel<T>
 *     send
 *     receive
 *     try send
 *     try receive
 *     close
 *     select
 *
 * Distributed channels do not replace generic channels.
 *
 * Semantic analysis may map a generic channel into a distributed realization
 * when program intent, capabilities, resources and policies require it.
 *
 * ============================================================================
 * 36. MESSAGING INTEGRATION
 * ============================================================================
 *
 * `grammar/distributed/messaging.g4` remains responsible for message schema
 * structure.
 *
 * This grammar treats payloads as ordinary canonical types/expressions.
 *
 * It does not define:
 *
 *     message schema syntax;
 *     serialization;
 *     packet structure.
 *
 * ============================================================================
 * 37. NETWORKING INTEGRATION
 * ============================================================================
 *
 * `grammar/networking/channels.g4` remains responsible for network-specific
 * realization.
 *
 * This grammar does not choose:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     RDMA;
 *     MPI;
 *     shared memory;
 *     accelerator fabric;
 *     quantum link;
 *     future transport.
 *
 * Such choices are downstream semantic/backend decisions.
 *
 * ============================================================================
 * 38. COMMUNICATION INTEGRATION
 * ============================================================================
 *
 * `grammar/distributed/communication.g4` owns:
 *
 *     distributedCommunication
 *     communicationOperation
 *     communicationName
 *     communicationArguments
 *
 * This grammar delegates to those rules instead of redefining them.
 *
 * This removes the previous competing communication grammar.
 *
 * ============================================================================
 * 39. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum values may participate in channel payloads.
 *
 * The distributed channel grammar does not create a quantum-specific channel
 * grammar or IR.
 *
 * Semantic quantum information eventually crosses:
 *
 *     quantum::ir
 *
 * before quantum target realization.
 *
 * ============================================================================
 * 40. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource and capability semantics are resolved by their canonical
 * subsystems.
 *
 * This file never introduces physical resource constants.
 *
 * The compiler may therefore negotiate:
 *
 *     memory
 *     bandwidth
 *     latency
 *     communication capability
 *     quantum communication capability
 *     topology
 *     reliability
 *     availability
 *     energy
 *     future resource dimensions
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * 41. CONTRACT / POLICY INTEGRATION
 * ============================================================================
 *
 * Channel declarations may participate in the universal:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * and policy systems.
 *
 * This file does not redefine those universal semantic systems.
 *
 * It merely preserves channel-local structural boundaries.
 *
 * ============================================================================
 * 42. PROVENANCE / EXPLAINABILITY
 * ============================================================================
 *
 * Channel-related decisions may later be recorded as provenance:
 *
 *     source declaration
 *     semantic interpretation
 *     capability decision
 *     resource decision
 *     placement decision
 *     routing decision
 *     scheduling decision
 *     target realization
 *
 * Explanation and provenance remain downstream semantic facilities.
 *
 * ============================================================================
 * 43. SIMULATION
 * ============================================================================
 *
 * The same channel syntax can participate in:
 *
 *     classical simulation;
 *     distributed simulation;
 *     network simulation;
 *     quantum simulation;
 *     fault simulation;
 *     performance simulation.
 *
 * Simulation is an execution strategy and does not require a second channel
 * grammar.
 *
 * ============================================================================
 * 44. ADAPTIVE EXECUTION
 * ============================================================================
 *
 * Channel realization may eventually adapt according to:
 *
 *     capabilities;
 *     resources;
 *     reliability;
 *     availability;
 *     topology;
 *     policy;
 *     runtime state.
 *
 * Such adaptation is downstream.
 *
 * Source-level channel meaning remains stable.
 *
 * ============================================================================
 * 45. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal physical limits.
 *
 * Specifically absent:
 *
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_MESSAGES
 *     MAX_QUEUE_SIZE
 *     MAX_BUFFER_SIZE
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *     MAX_TOPOLOGY_SIZE
 *
 * It also does not encode:
 *
 *     physical node IDs;
 *     device IDs;
 *     memory addresses;
 *     fixed topology;
 *     fixed transport;
 *     fixed serialization;
 *     fixed hardware placement.
 *
 * ============================================================================
 * 46. POSITIVE TESTS
 * ============================================================================
 *
 * Basic declaration:
 *
 *     distributed::channel data: Message;
 *
 * Initialized:
 *
 *     distributed::channel data: Message = initial_channel();
 *
 * Configured:
 *
 *     distributed::channel data: Message {
 *         distributed::capacity = capacity_expression;
 *         distributed::reliability = reliability_policy;
 *         distributed::availability = availability_policy;
 *     }
 *
 * Endpoints:
 *
 *     distributed::channel data: Message {
 *         endpoint input: Message;
 *         endpoint output: Message;
 *         endpoint inout: Message;
 *     }
 *
 * Requirements:
 *
 *     distributed::channel data: Message {
 *         requires capability("distributed.communication");
 *         requires memory >= required_memory;
 *     }
 *
 * Selection:
 *
 *     distributed::select {
 *         distributed::receive(input) => process(input);
 *         distributed::send(output, value) => continue_work();
 *     }
 *
 * Default selection:
 *
 *     distributed::select {
 *         distributed::receive(input) => process(input);
 *         distributed::default => fallback();
 *     }
 *
 * Quantum/classical payload:
 *
 *     distributed::channel results: Measurement;
 *
 * The grammar does not need to know whether `Measurement` is classical,
 * quantum, hybrid, AI, data, HDL or user-defined.
 *
 * ============================================================================
 * 47. NEGATIVE TESTS
 * ============================================================================
 *
 * Must reject:
 *
 *     distributed::channel;
 *
 *     distributed::channel data;
 *
 *     distributed::channel data:;
 *
 *     distributed::channel data: ;
 *
 *     distributed::channel data: Message =
 *
 *     endpoint;
 *
 *     endpoint input;
 *
 *     distributed::select {
 *
 *     distributed::receive(input)
 *
 * without an arrow in a select arm.
 *
 * ============================================================================
 * 48. BOUNDARY TESTS
 * ============================================================================
 *
 * Required:
 *
 *     one channel;
 *     many channels;
 *     one endpoint;
 *     many endpoints;
 *     one select arm;
 *     many select arms;
 *     nested channel bodies;
 *     deeply nested expressions;
 *     deeply qualified operation names;
 *     symbolic resource requirements;
 *     computed capacity;
 *     computed participants;
 *     quantum payload types;
 *     hybrid payload types;
 *     user-defined payload types.
 *
 * ============================================================================
 * 49. DETERMINISM TESTS
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     lexer configuration
 *     grammar version
 *     parser configuration
 *
 * must produce equivalent parse-tree structure.
 *
 * ============================================================================
 * 50. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] distributed channel declaration syntax has one owner;
 *     [x] distributed channel body syntax has one owner;
 *     [x] endpoint syntax has one owner;
 *     [x] channel-local policy/configuration has one owner;
 *     [x] channel-local requirement syntax has one owner;
 *     [x] channel selection has one owner;
 *     [x] generic distributed communication is delegated;
 *     [x] distributed messages are not duplicated;
 *     [x] generic concurrency channels remain independently owned;
 *     [x] networking remains downstream;
 *     [x] quantum semantics remain downstream;
 *     [x] no channel-specific IR exists;
 *     [x] no physical transport is selected;
 *     [x] no hardware limit is encoded;
 *     [x] no artificial channel cardinality is encoded;
 *     [x] source structure remains target-independent;
 *     [x] source spans remain preservable;
 *     [x] parser behavior is deterministic;
 *     [x] no semantic predicates exist;
 *     [x] no parser actions exist;
 *     [x] no runtime behavior occurs during parsing;
 *     [x] Rust 1.97+ safe-Rust integration is documented;
 *     [x] scalability tests are defined;
 *     [x] cross-domain tests are defined;
 *     [x] compatibility boundaries are explicit.
 *
 * ============================================================================
 * END OF grammar/distributed/channels.g4
 * ============================================================================
 */