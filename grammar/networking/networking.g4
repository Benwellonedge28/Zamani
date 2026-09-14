/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/networking.g4
 *
 * Grammar:
 *     Networking
 *
 * Status:
 *     Production networking grammar composition/root.
 *
 * Purpose:
 *     Provide the canonical parser-level integration boundary for all
 *     networking language constructs in Zamani.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust.
 *     - No unsafe code.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No environment-dependent parsing.
 *     - No randomness.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file is an AGGREGATE / COMPOSITION grammar.
 *
 * It does NOT duplicate the implementation of individual networking
 * constructs.
 *
 * Ownership is divided as follows:
 *
 *     networking.g4
 *         |
 *         +--> endpoints.g4
 *         +--> channels.g4
 *         +--> messages.g4
 *         +--> protocols.g4
 *         +--> services.g4
 *         +--> network-capabilities.g4
 *
 * The resulting parser surface is then consumed by the wider Zamani parser.
 *
 *
 * SOURCE
 *   |
 *   v
 * ZamaniLexer
 *   |
 *   v
 * Core parser / Networking parser
 *   |
 *   v
 * Networking syntax
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> type analysis
 *   +--> effect analysis
 *   +--> capability analysis
 *   +--> resource analysis
 *   +--> security analysis
 *   +--> distributed analysis
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> networking semantic model
 *   +--> distributed semantic model
 *   +--> quantum::ir where quantum computation participates
 *   +--> hardware/runtime model
 *   |
 *   v
 * Optimization
 *   |
 *   v
 * Routing
 *   |
 *   v
 * Scheduling
 *   |
 *   v
 * Placement / deployment
 *   |
 *   v
 * Runtime / hardware realization
 *
 * This grammar MUST NEVER construct IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Networking syntax describes logical communication intent.
 *
 * It MUST NOT make the following permanent source-level assumptions:
 *
 *     - machine count;
 *     - node count;
 *     - CPU count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - endpoint count;
 *     - channel count;
 *     - service count;
 *     - protocol count;
 *     - network topology;
 *     - physical addresses;
 *     - fixed bandwidth;
 *     - fixed latency;
 *     - fixed link count;
 *     - fixed deployment topology;
 *     - fixed provider;
 *     - fixed transport implementation.
 *
 * Therefore:
 *
 *     Program Once
 *         ->
 *     Compile Once
 *         ->
 *     Run Everywhere
 *         ->
 *     Run Anywhere
 *         ->
 *     Run Forever
 *
 * remains a semantic/compiler/runtime responsibility rather than a
 * machine-specific grammar restriction.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - networking grammar composition;
 *     - networking parser entry points;
 *     - networking construct aggregation;
 *     - networking-domain integration boundaries;
 *     - networking grammar version surface;
 *     - networking construct dispatch;
 *     - networking-wide syntax grouping.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - endpoint implementation;
 *     - channel implementation;
 *     - message schema implementation;
 *     - protocol implementation;
 *     - service implementation;
 *     - capability implementation;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - distributed execution;
 *     - hardware discovery;
 *     - resource allocation;
 *     - socket implementation;
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - HTTP;
 *     - MPI;
 *     - RDMA;
 *     - serialization;
 *     - cryptography;
 *     - authentication;
 *     - authorization;
 *     - quantum gates;
 *     - quantum states;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - classical IR;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * COMPONENT OWNERSHIP
 * ============================================================================
 *
 * Endpoint syntax:
 *
 *     grammar/networking/endpoints.g4
 *
 * Networking channel syntax:
 *
 *     grammar/networking/channels.g4
 *
 * Message syntax:
 *
 *     grammar/networking/messages.g4
 *
 * Protocol syntax:
 *
 *     grammar/networking/protocols.g4
 *
 * Service syntax:
 *
 *     grammar/networking/services.g4
 *
 * Networking capability syntax:
 *
 *     grammar/networking/network-capabilities.g4
 *
 * This file MUST NOT redefine any of those constructs.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: NETWORKING VS CONCURRENCY
 * ============================================================================
 *
 * Zamani also contains:
 *
 *     grammar/concurrency/channels.g4
 *
 * That grammar owns language-level concurrency channels.
 *
 * This directory owns networking channels.
 *
 * They MUST remain separate.
 *
 * Networking channels describe logical communication relationships and their
 * networking semantics.
 *
 * Concurrency channels describe language-level concurrent communication
 * primitives.
 *
 * This aggregate grammar MUST NOT merge the two abstractions.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: NETWORKING VS DISTRIBUTED
 * ============================================================================
 *
 * Networking owns communication semantics.
 *
 * Distributed computing owns:
 *
 *     - node membership;
 *     - placement;
 *     - replication;
 *     - distributed execution;
 *     - cluster semantics;
 *     - distributed scheduling;
 *     - distributed recovery.
 *
 * This file therefore does not import or duplicate distributed execution
 * declarations merely because they may use networking.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: NETWORKING VS HARDWARE
 * ============================================================================
 *
 * Networking can describe communication involving:
 *
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - accelerators;
 *     - embedded systems;
 *     - distributed systems;
 *     - future computational substrates.
 *
 * But networking.g4 does not define their physical resources.
 *
 * Hardware capability and physical realization remain downstream concerns.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: NETWORKING VS QUANTUM
 * ============================================================================
 *
 * Networking may participate in hybrid quantum/classical computation.
 *
 * This file does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     LogicalQubitId
 *     Gate
 *     Circuit
 *     QuantumState
 *     QuantumTopology
 *     Calibration
 *     Pulse
 *     QEC code
 *     ZQN fault
 *
 * Quantum semantic lowering remains connected to the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: NETWORKING VS SECURITY
 * ============================================================================
 *
 * Networking syntax may reference security requirements through expressions,
 * capabilities, effects, or qualified names.
 *
 * It does not implement:
 *
 *     - cryptography;
 *     - key generation;
 *     - key storage;
 *     - certificate validation;
 *     - authentication;
 *     - authorization;
 *     - trust evaluation.
 *
 * Those belong to the security/effects/security architecture.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION: NETWORKING VS DATA
 * ============================================================================
 *
 * Networking may reference message/data values.
 *
 * It does not define:
 *
 *     - serialization;
 *     - wire encodings;
 *     - database schemas;
 *     - compression;
 *     - binary layouts;
 *     - data transformation semantics.
 *
 * Message syntax is delegated to messages.g4.
 *
 * Data schemas remain owned by grammar/data.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar intentionally does NOT enumerate transports such as:
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
 * as a closed grammar vocabulary.
 *
 * Such names can be represented through the canonical name/expression system
 * and resolved by semantic/capability analysis.
 *
 * This means a future transport can be introduced without requiring a new
 * grammar rule merely because its name is new.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO constants such as:
 *
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_SERVICES
 *     MAX_PROTOCOLS
 *     MAX_MESSAGES
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *
 * in this grammar.
 *
 * Repetition is delegated to component grammars and uses ANTLR repetition
 * semantics without an artificial language-level upper bound.
 *
 * Practical limits may come from:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler resource policy;
 *     - operating-system resources;
 *     - runtime resources;
 *     - deployment resources;
 *     - target hardware.
 *
 * Those are NOT grammar limits.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no predicates;
 *     - no random behavior;
 *     - no environment inspection;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access.
 *
 * Consequently parsing depends only on the supplied token stream and the
 * imported grammar definitions.
 *
 * ============================================================================
 * IMPORT MODEL
 * ============================================================================
 *
 * ANTLR parser imports compose parser grammars into the importing grammar.
 *
 * The component grammars therefore remain independently maintainable while
 * networking.g4 provides one stable networking-domain integration boundary.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Direct dependencies:
 *
 *     ZamaniLexer
 *     Names
 *     Expressions
 *     Endpoints
 *     NetworkingChannels
 *     Messages
 *     Protocols
 *     Services
 *     NetworkCapabilities
 *
 * Indirect dependencies are inherited through those component grammars.
 *
 * This file MUST NOT depend directly on:
 *
 *     Quantum IR
 *     QEC
 *     ZQN
 *     Scheduling
 *     Routing
 *     Hardware HAL
 *     Resilience
 *     Runtime
 *
 * Those systems consume the semantic representation produced after parsing.
 *
 * ============================================================================
 * GRAMMAR COMPOSITION
 * ============================================================================
 *
 * The canonical component grammar names currently used by the networking
 * directory are:
 *
 *     Endpoints
 *     NetworkingChannels
 *     Messages
 *     Protocols
 *     Services
 *     NetworkCapabilities
 *
 * The component grammar root rules are intentionally consumed through stable
 * construct rules rather than copying their internal productions.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * `networkingUnit`
 *
 *     Parses one or more networking constructs followed by EOF.
 *
 * `networkingConstruct`
 *
 *     Parses exactly one networking construct.
 *
 * These are the stable composition boundaries for frontend integration.
 *
 * The wider Zamani parser SHOULD import Networking and consume
 * `networkingConstruct` at the appropriate language-domain boundary.
 *
 * ============================================================================
 * NO SECOND NETWORKING AST
 * ============================================================================
 *
 * This grammar produces ANTLR parse-tree contexts.
 *
 * It MUST NOT define a second semantic networking model.
 *
 * The frontend AST layer owns transformation from:
 *
 *     parse tree
 *         ->
 *     frontend AST
 *
 * Semantic analysis then produces canonical networking semantics.
 *
 * ============================================================================
 * FRONTEND INTEGRATION
 * ============================================================================
 *
 * Frontend responsibilities:
 *
 *     1. Invoke `networkingUnit` or `networkingConstruct`.
 *
 *     2. Preserve source spans.
 *
 *     3. Preserve the selected component construct.
 *
 *     4. Build the frontend AST.
 *
 *     5. Perform name resolution.
 *
 *     6. Perform type checking.
 *
 *     7. Perform effect checking.
 *
 *     8. Perform capability checking.
 *
 *     9. Perform resource/constraint analysis.
 *
 *     10. Lower to canonical semantic representations.
 *
 * This grammar MUST NOT perform any of those semantic operations itself.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Networking syntax may eventually lower to:
 *
 *     - networking semantic IR/model;
 *     - classical IR;
 *     - distributed semantic representation;
 *     - hardware communication representation;
 *     - quantum::ir-adjacent communication metadata where appropriate.
 *
 * The grammar itself never chooses the target representation.
 *
 * In particular:
 *
 *     grammar -> quantum::ir
 *
 * is NOT a direct dependency.
 *
 * The correct architecture is:
 *
 *     grammar
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic representation
 *       ->
 *     quantum::ir / classical IR / distributed / hardware representations
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime networking is downstream.
 *
 * This grammar does not:
 *
 *     - open sockets;
 *     - create endpoints;
 *     - allocate ports;
 *     - establish connections;
 *     - send messages;
 *     - receive messages;
 *     - select routes;
 *     - allocate physical links.
 *
 * Runtime behavior is derived from semantic information after compilation.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Networking requirements such as:
 *
 *     latency
 *     bandwidth
 *     reliability
 *     locality
 *     energy
 *     availability
 *     security
 *
 * remain expressions / semantic properties.
 *
 * This grammar does not turn them into hard-coded resource limits.
 *
 * Resource analysis determines whether a realization satisfies the program.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Network capabilities are delegated to:
 *
 *     grammar/networking/network-capabilities.g4
 *
 * Capability checking is semantic.
 *
 * A source-level capability requirement MUST NOT automatically select a
 * particular device or provider.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * This grammar is part of the Zamani language grammar version.
 *
 * Syntax changes MUST follow the repository's language compatibility policy.
 *
 * Component grammars may evolve independently internally, but the public
 * construct rules exposed here MUST remain stable across compatible grammar
 * revisions.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors are reported by the ANTLR parser/frontend diagnostic layer.
 *
 * This grammar MUST NOT embed custom error handling code.
 *
 * Semantic errors such as:
 *
 *     unknown protocol
 *     unsatisfied capability
 *     impossible requirement
 *     invalid endpoint reference
 *     invalid security requirement
 *
 * are NOT parser errors and must be reported by semantic analysis.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * networkingUnit MUST be tested with:
 *
 *     endpoint declarations;
 *     channel declarations;
 *     message declarations;
 *     protocol declarations;
 *     service declarations;
 *     networking capability declarations;
 *     mixed networking programs.
 *
 * Cross-domain tests MUST include:
 *
 *     classical + networking
 *     quantum + networking
 *     hybrid + networking
 *     HDL + networking
 *     hardware + networking
 *     distributed + networking
 *     AI + networking
 *     data + networking
 *     security + networking
 *
 * Scalability tests MUST verify absence of grammar-level finite limits for:
 *
 *     endpoints;
 *     channels;
 *     messages;
 *     protocols;
 *     services;
 *     capability declarations;
 *     construct count.
 *
 * Determinism tests MUST parse identical token streams repeatedly and obtain
 * identical parse structures.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_SERVICES
 *     MAX_PROTOCOLS
 *     MAX_MESSAGES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUANTUM_NODES
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_NETWORK_SIZE
 *
 * Any such restriction belongs to resource analysis, compilation policy,
 * runtime policy, or target capability negotiation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It is the sole aggregate networking grammar.
 *
 * [ ] It does not duplicate endpoint syntax.
 *
 * [ ] It does not duplicate channel syntax.
 *
 * [ ] It does not duplicate message syntax.
 *
 * [ ] It does not duplicate protocol syntax.
 *
 * [ ] It does not duplicate service syntax.
 *
 * [ ] It does not duplicate capability syntax.
 *
 * [ ] It exposes stable networking parser entry points.
 *
 * [ ] It imports the canonical networking component grammars.
 *
 * [ ] It uses the canonical Zamani lexer.
 *
 * [ ] It uses canonical Names and Expressions.
 *
 * [ ] It has no embedded Rust.
 *
 * [ ] It has no unsafe code.
 *
 * [ ] It has no semantic predicates.
 *
 * [ ] It has no machine-size constants.
 *
 * [ ] It has no physical-network assumptions.
 *
 * [ ] It has no transport-specific closed-world enumeration.
 *
 * [ ] It does not depend directly on runtime/hardware/QEC/ZQN/resilience.
 *
 * [ ] Component grammars remain independently completable.
 *
 * [ ] Frontend AST integration consumes the public rules.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * ============================================================================
 */

parser grammar Networking;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Parser composition imports.
 *
 * Names and Expressions are retained here because the networking component
 * grammars use the canonical name/expression rules and the aggregate grammar
 * is intended to remain independently generatable.
 */
import
    Names,
    Expressions,
    Endpoints,
    NetworkingChannels,
    Messages,
    Protocols,
    Services,
    NetworkCapabilities
    ;

/*
 * ============================================================================
 * PUBLIC ROOT
 * ============================================================================
 *
 * A networking compilation unit consists of zero or more networking
 * constructs followed by EOF.
 *
 * No finite construct count is encoded.
 */
networkingUnit
    : networkingConstruct*
      EOF
    ;

/*
 * ============================================================================
 * PUBLIC SINGLE-CONSTRUCT ENTRY POINT
 * ============================================================================
 *
 * This is the preferred integration point for the wider Zamani parser.
 */
networkingConstruct
    : endpointDeclaration
    | networkChannelConstruct
    | messageConstruct
    | protocolConstruct
    | serviceDeclaration
    | networkCapabilityConstruct
    ;

/*
 * ============================================================================
 * STABLE COMPONENT ADAPTERS
 * ============================================================================
 *
 * These adapter rules intentionally contain no semantic logic.
 *
 * They exist to provide a stable aggregate-domain naming layer while the
 * component grammars remain independently maintainable.
 */

/*
 * Endpoint adapter.
 *
 * `endpointDeclaration` is owned by Endpoints.
 */
networkingEndpoint
    : endpointDeclaration
    ;

/*
 * Networking-channel adapter.
 *
 * `networkChannelConstruct` is owned by NetworkingChannels.
 */
networkingChannel
    : networkChannelConstruct
    ;

/*
 * Message adapter.
 *
 * `messageConstruct` is owned by Messages.
 */
networkingMessage
    : messageConstruct
    ;

/*
 * Protocol adapter.
 *
 * `protocolConstruct` is owned by Protocols.
 */
networkingProtocol
    : protocolConstruct
    ;

/*
 * Service adapter.
 *
 * `serviceDeclaration` is owned by Services.
 */
networkingService
    : serviceDeclaration
    ;

/*
 * Capability adapter.
 *
 * `networkCapabilityConstruct` is owned by NetworkCapabilities.
 */
networkingCapability
    : networkCapabilityConstruct
    ;