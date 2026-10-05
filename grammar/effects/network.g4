/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/network.g4
 *
 * Grammar:
 *     NetworkEffects
 *
 * Status:
 *     Production network-effect domain adapter
 *
 * Technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust requirement.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No environment inspection.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical EFFECT-DOMAIN adapter for networking.
 *
 * It does NOT define the networking language itself.
 *
 * Source-level networking constructs such as:
 *
 *     endpoint
 *     channel
 *     message
 *     protocol
 *     request
 *     response
 *     route
 *     service
 *     socket
 *     stream
 *     service discovery
 *     distributed communication
 *
 * are owned by:
 *
 *     grammar/networking/
 *
 * This file instead connects networking semantics to Zamani's generic
 * effect system.
 *
 * The central architectural distinction is:
 *
 *     grammar/networking/
 *         =
 *     networking source-domain syntax
 *
 *     grammar/effects/network.g4
 *         =
 *     networking effect syntax/integration
 *
 * This prevents two independent networking languages from developing.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - networking effect references;
 *     - networking effect-operation references;
 *     - networking effect invocations;
 *     - networking effect-operation uses;
 *     - networking effect-reference lists;
 *     - networking effect sets;
 *     - stable parser integration points for networking effect analysis;
 *     - syntax-level wrappers that identify networking as an effect domain.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - endpoint declarations;
 *     - endpoint resolution;
 *     - addresses;
 *     - channels;
 *     - messages;
 *     - protocols;
 *     - requests;
 *     - responses;
 *     - routes;
 *     - service discovery;
 *     - services;
 *     - sockets;
 *     - streams;
 *     - distributed communication;
 *     - network topology;
 *     - routing;
 *     - scheduling;
 *     - packet transmission;
 *     - transport implementation;
 *     - DNS;
 *     - operating-system networking;
 *     - NIC selection;
 *     - device selection;
 *     - physical addresses;
 *     - ports;
 *     - bandwidth enforcement;
 *     - latency enforcement;
 *     - QoS enforcement;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - encryption implementation;
 *     - cryptographic implementation;
 *     - resource allocation;
 *     - capability discovery;
 *     - target selection;
 *     - deployment;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The production dependency direction is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     Effects                  Networking
 *       |                             |
 *       |                             |
 *       +------------+----------------+
 *                    |
 *                    v
 *             domain-neutral AST
 *                    |
 *                    v
 *             structural validation
 *                    |
 *                    v
 *             semantic analysis
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       effects   capability  resource
 *          |       analysis   analysis
 *          |         |         |
 *          +---------+---------+
 *                    |
 *                    v
 *             policy/security
 *                    |
 *                    v
 *          canonical semantic model
 *                    |
 *          +---------+----------------+
 *          |                          |
 *          v                          v
 *    classical semantics        quantum semantics
 *                                     |
 *                                     v
 *                                 quantum::ir
 *                    |
 *                    v
 *          optimization/lowering
 *                    |
 *             routing/scheduling
 *                    |
 *             resilience/recovery
 *                    |
 *                    v
 *             target realization
 *
 * This file exists above semantic realization.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * An EFFECT describes observable computational interaction.
 *
 * A CAPABILITY describes something an execution environment can provide.
 *
 * A RESOURCE describes something that may be consumed, required, available,
 * negotiated, or constrained.
 *
 * A REQUIREMENT describes a condition that must be satisfied.
 *
 * A CONSTRAINT describes a condition that a valid realization must obey.
 *
 * A PREFERENCE describes a desirable realization without changing correctness.
 *
 * A POLICY describes permitted/prohibited execution behavior.
 *
 * These concepts MUST NOT be collapsed into networking effect syntax.
 *
 * For example:
 *
 *     networking::send
 *
 * describes effect identity.
 *
 * It does NOT mean:
 *
 *     use a particular NIC
 *     use a particular interface
 *     use a particular address
 *     use a particular transport
 *     use a particular route
 *     use a fixed number of nodes
 *     use a fixed number of connections
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * OPEN-WORLD NETWORK EFFECT MODEL
 * ============================================================================
 *
 * Network effect identities are open-world qualified names.
 *
 * Examples:
 *
 *     networking::send
 *     networking::receive
 *     networking::request
 *     networking::response
 *     networking::connect
 *     networking::disconnect
 *     networking::publish
 *     networking::subscribe
 *     networking::stream
 *     networking::service
 *     networking::discovery
 *     networking::routing
 *     networking::transport
 *     networking::distributed
 *
 * Future domains are equally valid:
 *
 *     networking::future::operation
 *     networking::vendor::operation
 *     networking::custom::operation
 *     networking::quantum::communication
 *
 * This grammar deliberately does NOT enumerate those operations.
 *
 * The semantic system determines whether a resolved name actually represents
 * a valid networking effect.
 *
 * ============================================================================
 * NO NETWORK KEYWORD EXPLOSION
 * ============================================================================
 *
 * This file deliberately does NOT require new lexer keywords for:
 *
 *     send
 *     receive
 *     request
 *     response
 *     connect
 *     disconnect
 *     publish
 *     subscribe
 *     listen
 *     accept
 *     open
 *     close
 *     invoke
 *
 * Those names are represented through the existing open-world qualified-name
 * effect system.
 *
 * This prevents networking vocabulary from becoming a closed language
 * catalogue.
 *
 * New networking operations therefore do not require:
 *
 *     - a lexer modification;
 *     - a new parser alternative;
 *     - a new universal keyword;
 *     - a new effect grammar rule.
 *
 * A semantic registry/domain implementation may introduce new meanings
 * independently of the core grammar.
 *
 * ============================================================================
 * NETWORKING DOMAIN BOUNDARY
 * ============================================================================
 *
 * The relationship between this file and grammar/networking/ is:
 *
 *     networking source syntax
 *              |
 *              v
 *     grammar/networking/
 *              |
 *              +--> endpoint
 *              +--> channel
 *              +--> message
 *              +--> protocol
 *              +--> request
 *              +--> response
 *              +--> route
 *              +--> service
 *              +--> socket
 *              +--> stream
 *              +--> distributed communication
 *              |
 *              v
 *     networking semantic model
 *
 * while:
 *
 *     networking effect syntax
 *              |
 *              v
 *     grammar/effects/network.g4
 *              |
 *              v
 *     generic effect semantic model
 *
 * The two models are related semantically but are not competing grammar
 * authorities.
 *
 * ============================================================================
 * EFFECT OPERATION BOUNDARY
 * ============================================================================
 *
 * Generic effect operation syntax is owned by:
 *
 *     grammar/effects/effect-operations.g4
 *
 * Therefore this file MUST NOT redefine:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectInvocationArguments
 *     effectOperationCall
 *     effectOperationUse
 *
 * Instead, this file wraps those canonical rules.
 *
 * ============================================================================
 * EFFECT SET BOUNDARY
 * ============================================================================
 *
 * Generic effect collection syntax is owned by:
 *
 *     grammar/effects/effect-sets.g4
 *
 * Therefore this file MUST NOT redefine:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectSetBody
 *     effectSetComposition
 *
 * Instead, this file exposes networking-specific wrappers around those rules.
 *
 * ============================================================================
 * HANDLER BOUNDARY
 * ============================================================================
 *
 * Effect handling is owned by:
 *
 *     grammar/effects/effect-handling.g4
 *
 * This file does NOT define:
 *
 *     handleExpression
 *     handleStatement
 *     effectHandler
 *     effectHandlerArm
 *     effectHandlerBody
 *
 * Network effects participate in generic handlers through the canonical effect
 * system.
 *
 * ============================================================================
 * DECLARATION BOUNDARY
 * ============================================================================
 *
 * Generic effect declarations are owned by:
 *
 *     grammar/effects/effect-declarations.g4
 *
 * This file therefore does NOT create a second declaration syntax such as:
 *
 *     network effect ...
 *
 * A networking effect may be declared through the generic effect declaration
 * system and semantically classified under the networking domain.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * This grammar does not declare or discover capabilities.
 *
 * Networking capabilities are semantic names.
 *
 * Examples:
 *
 *     networking::communication
 *     networking::streaming
 *     networking::multicast
 *     networking::reliable_delivery
 *     networking::service_discovery
 *     networking::secure_transport
 *
 * The capability system determines:
 *
 *     - whether the capability exists;
 *     - whether a target provides it;
 *     - whether it is authorized;
 *     - whether it can satisfy a program requirement.
 *
 * This file merely preserves source structure.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * This file does not encode physical network resources.
 *
 * It MUST NOT define:
 *
 *     maximum bandwidth;
 *     maximum latency;
 *     maximum connections;
 *     maximum nodes;
 *     maximum endpoints;
 *     maximum messages;
 *     maximum streams;
 *     maximum routes;
 *     maximum devices;
 *     maximum network size.
 *
 * Resource quantities remain ordinary source values or semantic requirements.
 *
 * Physical feasibility is evaluated downstream.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Network effects may compose with security effects.
 *
 * Examples:
 *
 *     networking::request
 *     security::authenticate
 *     security::authorize
 *     security::confidentiality
 *     security::integrity
 *
 * This file does not import the security grammar.
 *
 * The relationship is intentionally semantic rather than a parser ownership
 * relationship.
 *
 * This prevents a cycle such as:
 *
 *     NetworkEffects
 *         -> Security
 *         -> NetworkEffects
 *
 * Security analysis may inspect the resulting AST/effect model after parsing.
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING BOUNDARY
 * ============================================================================
 *
 * Networking effects may participate in distributed computation.
 *
 * Examples:
 *
 *     networking::send
 *     networking::receive
 *     distributed::consensus
 *     distributed::replication
 *
 * This file does not own:
 *
 *     - actor semantics;
 *     - task scheduling;
 *     - node placement;
 *     - replication;
 *     - consistency;
 *     - consensus;
 *     - fault recovery;
 *     - distributed topology.
 *
 * Those remain owned by the distributed/concurrency subsystems.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking can surround or participate in quantum computation.
 *
 * Examples include:
 *
 *     networking::quantum::communication
 *     networking::quantum::execution
 *     networking::quantum::measurement_transport
 *     networking::quantum::remote_execution
 *
 * This grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *
 * If networking semantics participate in a quantum computation, downstream
 * semantic lowering may associate the resulting operation with:
 *
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar never creates another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Networking effects may describe abstract communication involving hardware
 * or HDL systems.
 *
 * Examples:
 *
 *     networking::hardware_link
 *     networking::accelerator_transport
 *     networking::device_communication
 *
 * This file does not define:
 *
 *     pins;
 *     buses;
 *     physical links;
 *     bus widths;
 *     registers;
 *     FPGA routing;
 *     ASIC wiring;
 *     physical interfaces;
 *     device IDs.
 *
 * Those concepts belong to HDL/hardware semantic layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A source program expresses communication intent.
 *
 * The same source-level effect may be realized through:
 *
 *     embedded communication
 *     local communication
 *     inter-process communication
 *     shared-memory communication
 *     CPU communication
 *     accelerator communication
 *     device communication
 *     cluster communication
 *     HPC communication
 *     cloud communication
 *     edge communication
 *     quantum-classical communication
 *     future communication substrates
 *
 * The grammar therefore describes:
 *
 *     WHAT
 *
 * rather than:
 *
 *     WHERE
 *     WHICH MACHINE
 *     WHICH DEVICE
 *     WHICH INTERFACE
 *     WHICH ROUTE
 *     WHICH PROVIDER
 *
 * This is required for Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no language-level finite limits on:
 *
 *     - qualified-name depth;
 *     - effect references;
 *     - effect operations;
 *     - effect-set entries;
 *     - operation arguments;
 *     - nested source constructs;
 *     - network participants;
 *     - endpoints;
 *     - channels;
 *     - messages;
 *     - services;
 *     - streams;
 *     - connections;
 *     - distributed participants;
 *     - network domains.
 *
 * This file contains no:
 *
 *     MAX_*
 *
 * capacity constants.
 *
 * Repetition and cardinality are inherited from the generic effect grammar
 * and ordinary parser structures.
 *
 * Practical compiler/parser resource exhaustion is an implementation/resource
 * condition and MUST NOT be transformed into a language-level networking
 * ceiling.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token stream;
 *     - selected grammar version;
 *     - explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware;
 *     - network availability;
 *     - DNS;
 *     - runtime state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - target availability;
 *     - resource availability.
 *
 * Identical token input under identical grammar configuration must produce
 * equivalent parse structures.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates parser structure only.
 *
 * The frontend AST should preserve:
 *
 *     - source span;
 *     - qualified-name segments;
 *     - operation identity;
 *     - invocation arguments;
 *     - effect-reference ordering;
 *     - effect-set structure.
 *
 * Suggested domain-neutral semantic classification:
 *
 *     NetworkEffectReference
 *     NetworkEffectOperationReference
 *     NetworkEffectInvocation
 *     NetworkEffectOperationUse
 *     NetworkEffectReferenceList
 *     NetworkEffectSet
 *
 * These are semantic/frontend concepts, not runtime networking objects.
 *
 * The parser MUST NOT construct:
 *
 *     Socket
 *     NetworkDevice
 *     NetworkInterface
 *     Router
 *     Packet
 *     PhysicalConnection
 *     QPU
 *     PhysicalQubit
 *     HardwareDevice
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether a reference resolves;
 *     - whether it belongs to the networking domain;
 *     - whether an operation is declared;
 *     - whether its arguments are valid;
 *     - whether required capabilities exist;
 *     - whether resource requirements can be satisfied;
 *     - whether policies permit the operation;
 *     - whether security requirements are satisfied;
 *     - whether distributed semantics are compatible;
 *     - whether quantum participation is valid;
 *     - whether the operation can be lowered to the canonical semantic model.
 *
 * The parser MUST NOT perform these checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The semantic pipeline is:
 *
 *     network effect syntax
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     effect semantic model
 *             |
 *             v
 *     networking semantic model
 *             |
 *       +-----+-------------------+
 *       |                         |
 *       v                         v
 * classical semantics       quantum semantics
 *                                 |
 *                                 v
 *                              quantum::ir
 *       |
 *       v
 * optimization
 *       |
 *       v
 * routing
 *       |
 *       v
 * scheduling
 *       |
 *       v
 * resilience/recovery
 *       |
 *       v
 * target realization
 *
 * The grammar does not choose the IR.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to malformed syntax.
 *
 * Examples:
 *
 *     malformed qualified name
 *     malformed invocation
 *     malformed effect set
 *     malformed effect reference list
 *
 * Semantic diagnostics are downstream.
 *
 * Examples:
 *
 *     unknown networking effect;
 *     unknown networking operation;
 *     unavailable networking capability;
 *     insufficient resources;
 *     forbidden policy;
 *     incompatible argument type;
 *     unsupported realization;
 *     invalid security combination.
 *
 * These MUST NOT be represented as parser-level keyword failures.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing generic effect syntax remains authoritative.
 *
 * Existing networking source syntax remains owned by:
 *
 *     grammar/networking/
 *
 * This file MUST NOT introduce alternate spellings for networking constructs
 * that already have an owner elsewhere.
 *
 * Compatibility aliases belong to:
 *
 *     grammar/compatibility/
 *
 * Historical examples do not automatically become legal syntax.
 *
 * Promotion remains:
 *
 *     proposal
 *         ->
 *     specification
 *         ->
 *     AST contract
 *         ->
 *     canonical grammar
 *         ->
 *     semantic implementation
 *         ->
 *     IR integration
 *         ->
 *     conformance tests
 *         ->
 *     stable feature
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/core/
 *     grammar/expressions/
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The generic effect-operation grammar already depends on the canonical core
 * and expression grammar.
 *
 * This file therefore does not duplicate those dependencies unnecessarily.
 *
 * EXPORTS:
 *
 *     networkEffectReference
 *     networkEffectReferenceList
 *     networkEffectSet
 *     networkEffectOperationReference
 *     networkEffectInvocation
 *     networkEffectOperationUse
 *     networkEffectConstruct
 *     networkEffectReferenceConstruct
 *     networkEffectOperationConstruct
 *
 * CONSUMED_BY:
 *
 *     semantic networking/effect analysis
 *     effect-domain tooling
 *     effect conformance tests
 *     future parser/domain adapters where explicitly required
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral Zamani frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     Effect analysis + networking semantic analysis.
 *
 * IR_OWNER:
 *
 *     Canonical compiler semantic/IR pipeline.
 *
 *     Quantum semantics ultimately use:
 *
 *         quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/effects/
 *     grammar/tests/networking/
 *     grammar/tests/cross-domain/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/effects.md
 *     grammar/spec/networking.md
 *     grammar/specification/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Only canonical generic effect components are imported.
 *
 * The dependency graph is:
 *
 *     NetworkEffects
 *          |
 *          +--> EffectOperations
 *          |       |
 *          |       +--> Core
 *          |       +--> Expressions
 *          |
 *          +--> EffectSets
 *                  |
 *                  +--> canonical effect-set rules
 *
 * This file MUST NOT import:
 *
 *     Networking
 *     Security
 *     Distributed
 *     Quantum
 *     Hardware
 *     Resources
 *
 * directly.
 *
 * Those relationships are semantic/domain integration boundaries.
 *
 * This is intentional and prevents grammar cycles.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * The public rules below are deliberately small wrappers around canonical
 * generic effect rules.
 *
 * They provide stable semantic ownership without creating duplicate syntax.
 *
 * ============================================================================
 */

parser grammar NetworkEffects;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    EffectOperations,
    EffectSets
;


/*
 * ============================================================================
 * 1. NETWORK EFFECT REFERENCE
 * ============================================================================
 *
 * Represents a source-level effect identity that semantic analysis may
 * classify as belonging to the networking domain.
 *
 * Examples:
 *
 *     networking
 *     networking::send
 *     networking::receive
 *     networking::request
 *     networking::response
 *     networking::stream
 *     networking::service
 *     networking::discovery
 *     networking::custom::operation
 *
 * No networking operation is enumerated here.
 */

networkEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 2. NETWORK EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Reuses the canonical effect-reference-list syntax.
 *
 * No fixed cardinality is imposed.
 */

networkEffectReferenceList
    : effectReferenceList
    ;


/*
 * ============================================================================
 * 3. NETWORK EFFECT SET
 * ============================================================================
 *
 * Reuses the canonical effect-set syntax.
 *
 * Example:
 *
 *     {
 *         networking::send,
 *         networking::receive
 *     }
 *
 * Semantic analysis determines whether the entries are valid networking
 * effects.
 */

networkEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * 4. NETWORK EFFECT OPERATION REFERENCE
 * ============================================================================
 *
 * Reuses the canonical effect-operation reference.
 *
 * Examples:
 *
 *     networking::send
 *     networking::receive
 *     networking::request
 *     networking::future::operation
 *     vendor::networking::operation
 */

networkEffectOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================