/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/network-capabilities.g4
 *
 * Grammar:
 *     NetworkingCapabilities
 *
 * Status:
 *     Production candidate / canonical networking capability grammar
 *
 * Purpose:
 *     Define the networking-domain SOURCE SYNTAX for:
 *
 *       - capability requirements;
 *       - capability provisions;
 *       - capability constraints;
 *       - capability preferences;
 *       - capability contracts;
 *       - capability sets;
 *       - capability negotiation intent;
 *       - capability predicates;
 *       - parameterized capability references;
 *       - networking capability metadata.
 *
 * This grammar is intentionally OPEN-WORLD.
 *
 * A capability identity is a semantic name, not a closed enumeration.
 *
 * Examples:
 *
 *     networking::reliable_delivery
 *     networking::ordered_delivery
 *     networking::low_latency
 *     networking::high_throughput
 *     quantum::communication
 *     security::confidentiality
 *     hardware::communication_fabric
 *     future::networking::new_capability
 *
 * ============================================================================
 * RUST / IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *     1.97 / 1.97.1
 *
 * Edition:
 *     2021
 *
 * Safety:
 *     Safe Rust only.
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no environment inspection.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     Zamani lexer
 *          |
 *          v
 *     NetworkingCapabilities
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Structural validation
 *          |
 *          v
 *     Name / capability resolution
 *          |
 *          v
 *     Resource + capability analysis
 *          |
 *          v
 *     Canonical semantic representation
 *          |
 *          +-------------------+------------------+
 *          |                   |                  |
 *       classical          quantum::ir       HDL/hardware
 *          |                   |                  |
 *          +-------------------+------------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                       target realization
 *
 * The grammar never constructs IR and never selects hardware.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - networking capability contract syntax;
 *     - networking capability requirement syntax;
 *     - networking capability provision syntax;
 *     - networking capability constraint syntax;
 *     - networking capability preference syntax;
 *     - networking capability predicate syntax;
 *     - networking capability grouping;
 *     - networking capability sets;
 *     - networking capability negotiation intent;
 *     - networking capability metadata syntax;
 *     - networking capability call syntax;
 *     - networking capability expression composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - keywords;
 *     - general capability declarations;
 *     - capability version semantics;
 *     - capability registry;
 *     - capability discovery;
 *     - resource allocation;
 *     - hardware discovery;
 *     - endpoint declarations;
 *     - address declarations;
 *     - protocol declarations;
 *     - channel declarations;
 *     - message declarations;
 *     - service declarations;
 *     - socket declarations;
 *     - routing;
 *     - placement;
 *     - scheduling;
 *     - distributed execution;
 *     - transport implementations;
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - HTTP;
 *     - MPI;
 *     - RDMA;
 *     - IP/MAC addressing;
 *     - physical topology;
 *     - cryptographic implementation;
 *     - authentication;
 *     - authorization;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - runtime execution;
 *     - backend selection.
 *
 * ============================================================================
 * EXISTING REPOSITORY OWNERSHIP
 * ============================================================================
 *
 * General capability identity and version syntax remain owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Canonical names remain owned by:
 *
 *     grammar/core/names.g4
 *
 * Canonical expressions remain owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Attributes remain owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Networking endpoints remain owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * Networking addresses remain owned by:
 *
 *     grammar/networking/addresses.g4
 *
 * Networking protocols remain owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * Networking channels remain owned by:
 *
 *     grammar/networking/channels.g4
 *
 * Networking messages remain owned by:
 *
 *     grammar/networking/messages.g4
 *
 * Networking services remain owned by:
 *
 *     grammar/networking/services.g4
 *
 * Networking sockets remain owned by:
 *
 *     grammar/networking/sockets.g4
 *
 * Networking routing remains owned by:
 *
 *     grammar/networking/routing.g4
 *
 * Distributed execution remains owned by:
 *
 *     grammar/distributed/
 *
 * Hardware intent remains owned by:
 *
 *     grammar/hardware/
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * Security semantics remain owned by:
 *
 *     grammar/security/
 *
 * ============================================================================
 * CRITICAL DESIGN RULE
 * ============================================================================
 *
 * Capability identity is OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     Ethernet
 *     InfiniBand
 *     MPI
 *     RDMA
 *     GPU
 *     FPGA
 *     QPU
 *     CPU
 *     vendor-specific devices
 *
 * as a closed capability grammar.
 *
 * Instead:
 *
 *     capabilityReference
 *
 * from core/capabilities.g4 is reused.
 *
 * Therefore future capability identities do not require modification of this
 * grammar.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE SEPARATION
 * ============================================================================
 *
 * REQUIREMENT:
 *
 *     requires networking::reliable_delivery;
 *
 * means that a valid realization MUST satisfy the requirement.
 *
 * PROVISION:
 *
 *     provides networking::reliable_delivery;
 *
 * describes a capability exposed by a logical declaration.
 *
 * CONSTRAINT:
 *
 *     constraint networking::latency <= maximum_latency;
 *
 * restricts valid realizations.
 *
 * PREFERENCE:
 *
 *     preference networking::low_latency;
 *
 * expresses an advisory preference.
 *
 * These meanings are semantic.
 *
 * The parser only records their syntax.
 *
 * A preference MUST NOT silently become a requirement.
 *
 * A provision MUST NOT imply runtime authorization.
 *
 * A capability reference MUST NOT imply physical resource allocation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Networking capability syntax MUST remain independent of target scale.
 *
 * The same source program may ultimately be realized using:
 *
 *     in-process communication
 *     shared memory
 *     IPC
 *     local networking
 *     distributed networking
 *     accelerator fabrics
 *     quantum communication
 *     future communication substrates
 *
 * without changing the capability grammar.
 *
 * Capability declarations describe WHAT is required or provided.
 *
 * They do not describe:
 *
 *     WHICH machine;
 *     WHICH CPU;
 *     WHICH GPU;
 *     WHICH FPGA;
 *     WHICH QPU;
 *     WHICH router;
 *     WHICH network interface;
 *     WHICH physical link;
 *     WHICH physical qubit;
 *     WHICH provider.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT impose:
 *
 *     MAX_NETWORK_CAPABILITIES
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_SERVICES
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_PROTOCOLS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_THREADS
 *     MAX_MEMORY
 *
 * It MUST NOT contain special constructs such as:
 *
 *     node_0
 *     node_1
 *     gpu_0
 *     gpu_1
 *     qpu_0
 *     router_0
 *     interface_0
 *
 * as universal language concepts.
 *
 * Numeric expressions are program semantics.
 *
 * For example:
 *
 *     requires networking::bandwidth >= required_bandwidth;
 *
 * is valid.
 *
 * The grammar does not decide what `required_bandwidth` means physically.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Capability requirements may refer to semantic resources:
 *
 *     requires networking::throughput >= required_rate;
 *
 *     requires networking::latency <= latency_budget;
 *
 *     requires capability("network.reliable");
 *
 * The grammar does not allocate resources.
 *
 * Resource feasibility is determined downstream by:
 *
 *     resource analysis;
 *     capability analysis;
 *     target discovery;
 *     compilation;
 *     runtime/deployment.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Capability names may refer to security capabilities:
 *
 *     security::authentication
 *     security::confidentiality
 *     security::integrity
 *     security::authorization
 *
 * This grammar does not implement:
 *
 *     cryptography;
 *     keys;
 *     credentials;
 *     certificate validation;
 *     identity providers;
 *     authorization engines;
 *     trust evaluation.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking capabilities may reference quantum communication capabilities:
 *
 *     quantum::communication
 *     quantum::entanglement_distribution
 *     quantum::quantum_networking
 *
 * These remain semantic capability identities.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     LogicalQubitId
 *     GateKind
 *     QuantumState
 *     QuantumTopology
 *     Calibration
 *     QEC
 *     ZQN
 *
 * When a capability affects quantum computation, semantic lowering eventually
 * reaches the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR is created here.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Networking capabilities may refer to hardware capabilities:
 *
 *     hardware::communication_fabric
 *     hardware::dma
 *     accelerator::networking
 *
 * This grammar does not define:
 *
 *     pins;
 *     wires;
 *     buses;
 *     clocks;
 *     physical routing;
 *     fixed bus widths;
 *     device counts.
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING BOUNDARY
 * ============================================================================
 *
 * Networking capability syntax may be consumed by distributed computation.
 *
 * This file does not own:
 *
 *     node membership;
 *     process placement;
 *     replication;
 *     partitioning;
 *     consensus;
 *     distributed scheduling;
 *     distributed recovery.
 *
 * Those belong to:
 *
 *     grammar/distributed/
 *
 * or the networking/distributed-compute integration boundary.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST:
 *
 *     - produce the same structure for the same token stream;
 *     - perform no capability discovery;
 *     - perform no resource discovery;
 *     - perform no network discovery;
 *     - perform no target selection;
 *     - perform no runtime negotiation.
 *
 * Capability resolution is semantic, not syntactic.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The frontend AST must retain source spans for:
 *
 *     - complete capability construct;
 *     - marker;
 *     - capability reference;
 *     - version requirement;
 *     - predicate operators;
 *     - predicate operands;
 *     - call arguments;
 *     - metadata keys;
 *     - metadata values;
 *     - nested capability blocks;
 *     - attributes.
 *
 * This grammar must not discard source structure.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser provides enough structure for a domain-neutral frontend AST to
 * represent conceptually:
 *
 *     NetworkingCapabilityRequirement
 *     NetworkingCapabilityProvision
 *     NetworkingCapabilityConstraint
 *     NetworkingCapabilityPreference
 *     NetworkingCapabilityPredicate
 *     NetworkingCapabilityContract
 *     NetworkingCapabilitySet
 *     NetworkingCapabilityNegotiation
 *     NetworkingCapabilityMetadata
 *     NetworkingCapabilityCall
 *
 * Every node must retain:
 *
 *     source span;
 *     source ordering;
 *     referenced name;
 *     predicate structure;
 *     argument structure.
 *
 * The AST MUST NOT directly contain:
 *
 *     physical device state;
 *     resolved router;
 *     resolved network interface;
 *     allocated bandwidth;
 *     selected hardware;
 *     runtime capability token.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability names;
 *     - resolving namespaces;
 *     - validating capability versions;
 *     - validating capability applicability;
 *     - checking requirements;
 *     - checking provisions;
 *     - checking constraints;
 *     - evaluating preferences;
 *     - checking conflicts;
 *     - integrating resource requirements;
 *     - integrating security requirements;
 *     - integrating distributed requirements;
 *     - checking target feasibility;
 *     - checking portability;
 *     - resolving capability references against declarations/registries.
 *
 * The grammar does none of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * Capability information lowers through the canonical semantic model.
 *
 * Possible downstream destinations include:
 *
 *     classical semantic representation;
 *     networking semantic representation;
 *     distributed semantic representation;
 *     hardware semantic representation;
 *     quantum semantic representation;
 *     quantum::ir.
 *
 * The grammar MUST NOT define:
 *
 *     NetworkingCapabilityIR
 *     QuantumCapabilityIR
 *
 * merely to represent syntax.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use capability information to:
 *
 *     - validate target compatibility;
 *     - constrain routing;
 *     - constrain scheduling;
 *     - select legal communication strategies;
 *     - select legal serialization strategies;
 *     - determine security obligations;
 *     - determine distributed execution requirements;
 *     - determine whether a target can satisfy the program.
 *
 * Compiler decisions remain downstream from parsing.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may perform:
 *
 *     capability discovery;
 *     environment inspection;
 *     negotiation;
 *     endpoint resolution;
 *     transport selection;
 *     service binding;
 *     adaptation.
 *
 * None of these actions occur in this grammar.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may use this grammar for:
 *
 *     - syntax highlighting;
 *     - completion;
 *     - capability navigation;
 *     - diagnostics;
 *     - documentation generation;
 *     - semantic queries;
 *     - dependency visualization.
 *
 * Tooling MUST NOT infer physical hardware merely from a capability name.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar reuses:
 *
 *     core/capabilities.g4
 *
 * for capability identity and version syntax.
 *
 * Consequently, capability-version compatibility remains a semantic concern.
 *
 * Adding:
 *
 *     future::networking::new_capability
 *
 * does not require a grammar change.
 *
 * Changing an established capability's semantic meaning requires the normal
 * Zamani compatibility/versioning process.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * The primary public entry point is:
 *
 *     networkCapabilityConstruct
 *
 * It intentionally accepts ONLY complete networking capability constructs.
 *
 * A bare capability predicate is NOT a top-level networking construct.
 *
 * This prevents ordinary expression syntax from being accidentally accepted
 * as a networking declaration by the networking aggregate grammar.
 *
 * Reusable predicate syntax remains publicly available through:
 *
 *     networkCapabilityPredicate
 *
 * for consumers that already own an enclosing declaration.
 *
 * ============================================================================
 */

parser grammar NetworkingCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    Capabilities
    ;


/* ============================================================================
 * PUBLIC CONSTRUCT ENTRY POINT
 * ============================================================================
 *
 * Only declaration/contract-level forms belong here.
 *
 * IMPORTANT:
 *
 * Do not add:
 *
 *     | networkCapabilityPredicate
 *
 * to this rule.
 *
 * A predicate is an expression-level component, not a top-level networking
 * declaration.
 */
networkCapabilityConstruct
    : networkCapabilityRequirement
    | networkCapabilityProvision
    | networkCapabilityConstraint
    | networkCapabilityPreference
    | networkCapabilityContract
    | networkCapabilitySet
    | networkCapabilityNegotiation
    ;


/* ============================================================================
 * CAPABILITY REFERENCE
 * ============================================================================
 *
 * Reuses the canonical capability identity/version model.
 *
 * Examples:
 *
 *     networking::reliable_delivery
 *     networking::low_latency
 *     quantum::communication
 *     security::confidentiality
 *     future::networking::new_capability
 *
 * This grammar assigns no semantic meaning to the namespace.
 */
networkCapabilityReference
    : capabilityReference
    ;


/* ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Mandatory semantic condition.
 *
 * Examples:
 *
 *     requires networking::reliable_delivery;
 *
 *     requires networking::ordered_delivery;
 *
 *     requires quantum::communication;
 *
 *     requires networking::latency <= latency_budget;
 */
networkCapabilityRequirement
    : REQUIRES
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * PROVISION
 * ============================================================================
 *
 * Describes a capability exposed by a logical declaration.
 *
 * It does NOT grant authorization.
 *
 * It does NOT allocate a physical resource.
 */
networkCapabilityProvision
    : PROVIDES
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Restricts valid realizations.
 */
networkCapabilityConstraint
    : CONSTRAINT
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Advisory semantic intent.
 *
 * It MUST remain distinguishable from a requirement.
 */
networkCapabilityPreference
    : PREFERENCE
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * PREDICATE
 * ============================================================================
 *
 * The predicate language is deliberately small and compositional.
 *
 * Capability identity itself remains open-world.
 */
networkCapabilityPredicate
    : networkCapabilityDisjunction
    ;


/* ============================================================================
 * DISJUNCTION
 * ============================================================================
 */
networkCapabilityDisjunction
    : networkCapabilityConjunction
      (OR networkCapabilityConjunction)*
    ;


/* ============================================================================
 * CONJUNCTION
 * ============================================================================
 */
networkCapabilityConjunction
    : networkCapabilityUnary
      (AND networkCapabilityUnary)*
    ;


/* ============================================================================
 * UNARY
 * ============================================================================
 */
networkCapabilityUnary
    : NOT networkCapabilityUnary
    | networkCapabilityPrimary
    ;


/* ============================================================================
 * PRIMARY
 * ============================================================================
 */
networkCapabilityPrimary
    : networkCapabilityComparison
    | networkCapabilityCall
    | networkCapabilityPresence
    | networkCapabilityReference
    | networkCapabilityGroup
    ;


/* ============================================================================
 * COMPARISON
 * ============================================================================
 *
 * Capability/resource values may be compared without encoding physical limits.
 *
 * Examples:
 *
 *     networking::latency <= latency_budget
 *
 *     networking::throughput >= required_throughput
 *
 *     networking::reliability >= required_reliability
 *
 *     networking::availability != unavailable
 */
networkCapabilityComparison
    : networkCapabilityReference
      networkCapabilityComparator
      expression
    ;


/* ============================================================================
 * COMPARISON OPERATOR
 * ============================================================================
 */
networkCapabilityComparator
    : LT
    | LE
    | GT
    | GE
    | EQ
    | NE
    ;


/* ============================================================================
 * PARAMETERIZED CAPABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     networking::throughput(required_rate)
 *
 *     networking::latency(maximum_latency)
 *
 *     networking::delivery(reliable)
 *
 *     future::networking::capability(parameter)
 *
 * Whether a capability is semantically callable is decided downstream.
 */
networkCapabilityCall
    : networkCapabilityReference
      LPAREN
      networkCapabilityArgumentList?
      RPAREN
    ;


/* ============================================================================
 * CALL ARGUMENTS
 * ============================================================================
 */
networkCapabilityArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * PRESENCE TEST
 * ============================================================================
 *
 * Example:
 *
 *     networking::reliable_delivery is true
 *
 * or:
 *
 *     networking::reliable_delivery is false
 *
 * Semantic analysis determines whether presence testing is meaningful for the
 * referenced capability.
 */
networkCapabilityPresence
    : networkCapabilityReference
      IS
      networkCapabilityPresenceValue
    ;


networkCapabilityPresenceValue
    : TRUE
    | FALSE
    ;


/* ============================================================================
 * GROUP
 * ============================================================================
 */
networkCapabilityGroup
    : LPAREN
      networkCapabilityPredicate
      RPAREN
    ;


/* ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Example:
 *
 *     capability_contract {
 *         requires networking::reliable_delivery;
 *         provides networking::ordered_delivery;
 *         constraint networking::latency <= latency_budget;
 *         preference networking::low_latency;
 *     }
 *
 * Contract membership is unbounded.
 */
networkCapabilityContract
    : CAPABILITY_CONTRACT
      LBRACE
      networkCapabilityContractMember*
      RBRACE
    ;


networkCapabilityContractMember
    : networkCapabilityRequirement
    | networkCapabilityProvision
    | networkCapabilityConstraint
    | networkCapabilityPreference
    | networkCapabilityNegotiation
    | networkCapabilitySet
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * CAPABILITY SET
 * ============================================================================
 *
 * A set exposes or groups capabilities without imposing a fixed cardinality.
 *
 * Example:
 *
 *     capabilities {
 *         networking::reliable_delivery;
 *         networking::ordered_delivery;
 *         networking::multicast;
 *     }
 */
networkCapabilitySet
    : CAPABILITIES
      LBRACE
      networkCapabilitySetMember*
      RBRACE
    ;


networkCapabilitySetMember
    : networkCapabilityReference
      SEMI
    | networkCapabilityProvision
    | networkCapabilityRequirement
    | networkCapabilityConstraint
    | networkCapabilityPreference
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * NEGOTIATION
 * ============================================================================
 *
 * This represents SOURCE INTENT only.
 *
 * It does not perform runtime negotiation.
 */
networkCapabilityNegotiation
    : NEGOTIATE
      LBRACE
      networkCapabilityNegotiationMember*
      RBRACE
    ;


networkCapabilityNegotiationMember
    : networkCapabilityPredicate
      SEMI
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * GENERIC METADATA
 * ============================================================================
 *
 * Metadata is intentionally open-world.
 *
 * It does not introduce a second networking property language.
 *
 * Example:
 *
 *     metadata priority: preferred;
 *
 *     metadata::priority: preferred;
 *
 * The actual metadata marker is lexically defined by the canonical lexer.
 */
networkCapabilityMetadata
    : PROPERTY
      identifier
      networkCapabilityMetadataValue?
      SEMI
    ;


networkCapabilityMetadataValue
    : COLON
      expression
    | ASSIGN
      expression
    ;


/* ============================================================================
 * INTEGRATION ADAPTERS
 * ============================================================================
 *
 * These adapters allow endpoint/channel/service/protocol grammars to consume
 * the canonical capability contract without copying its implementation.
 *
 * They introduce no new semantics.
 */
endpointCapabilityContract
    : networkCapabilityContract
    ;


channelCapabilityContract
    : networkCapabilityContract
    ;


serviceCapabilityContract
    : networkCapabilityContract
    ;


protocolCapabilityContract
    : networkCapabilityContract
    ;


/* ============================================================================
 * REUSABLE REQUIREMENT ADAPTER
 * ============================================================================
 *
 * Useful for declarations that already own their surrounding syntax.
 */
networkingCapabilityRequirement
    : networkCapabilityRequirement
    ;


/* ============================================================================
 * SOURCE-LEVEL RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * This adapter is deliberately syntactic.
 *
 * It does not create a second resource language.
 */
networkingCapabilityPredicate
    : networkCapabilityPredicate
    ;


/* ============================================================================
 * SEMANTIC INTEGRATION NOTES
 * ============================================================================
 *
 * Endpoint:
 *
 *     endpoint
 *        |
 *        +--> capability contract
 *
 * The endpoint grammar remains responsible for endpoint identity.
 *
 * Channel:
 *
 *     channel
 *        |
 *        +--> capability contract
 *
 * The channel grammar remains responsible for channel identity and channel
 * semantics.
 *
 * Protocol:
 *
 *     protocol
 *        |
 *        +--> capability contract
 *
 * The protocol grammar remains responsible for protocol identity and syntax.
 *
 * Service:
 *
 *     service
 *        |
 *        +--> capability contract
 *
 * The service grammar remains responsible for service syntax.
 *
 * Routing:
 *
 *     route
 *        |
 *        +--> capability requirement / constraint
 *
 * Routing remains responsible for route intent.
 *
 * Distributed computing:
 *
 *     distributed computation
 *        |
 *        +--> capability contract
 *
 * Distributed grammar remains responsible for distributed execution.
 *
 * Hardware:
 *
 *     hardware intent
 *        |
 *        +--> capability requirement
 *
 * Hardware grammar remains responsible for hardware intent.
 *
 * Quantum:
 *
 *     quantum computation
 *        |
 *        +--> capability requirement
 *        |
 *        v
 *     semantic analysis
 *        |
 *        v
 *     quantum::ir
 *
 * No quantum implementation details enter this grammar.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Grammar rule                 Semantic AST concept
 * ---------------------------------------------------------------------------
 * networkCapabilityReference   CapabilityReference
 * networkCapabilityRequirement CapabilityRequirement
 * networkCapabilityProvision   CapabilityProvision
 * networkCapabilityConstraint  CapabilityConstraint
 * networkCapabilityPreference  CapabilityPreference
 * networkCapabilityPredicate   CapabilityPredicate
 * networkCapabilityCall        CapabilityCall
 * networkCapabilityPresence    CapabilityPresence
 * networkCapabilityContract    CapabilityContract
 * networkCapabilitySet         CapabilitySet
 * networkCapabilityNegotiation CapabilityNegotiation
 * networkCapabilityMetadata    CapabilityMetadata
 *
 * The actual Rust AST types remain owned by the frontend AST implementation.
 *
 * This grammar does not define Rust structures.
 *
 * ============================================================================
 * SEMANTIC RULES
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     1. resolve capability names;
 *     2. resolve capability versions;
 *     3. distinguish requirement/provision/constraint/preference;
 *     4. validate capability applicability;
 *     5. validate predicate operands;
 *     6. validate parameterized capability arguments;
 *     7. validate presence tests;
 *     8. detect contradictory requirements;
 *     9. evaluate resource implications;
 *    10. evaluate target feasibility;
 *    11. preserve portability;
 *    12. preserve source meaning when target hardware changes.
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * Parser errors include:
 *
 *     requires;
 *     requires capability
 *     requires: capability;
 *     provides;
 *     constraint;
 *     preference;
 *     capability_contract;
 *     capabilities;
 *     negotiate;
 *     malformed qualified capability names;
 *     malformed comparisons;
 *     malformed calls;
 *     missing delimiters;
 *     missing semicolons.
 *
 * Semantic errors include:
 *
 *     unknown capability;
 *     unknown capability version;
 *     unsupported capability;
 *     conflicting capabilities;
 *     unsatisfied requirement;
 *     invalid capability parameterization;
 *     invalid presence operation;
 *     impossible constraint;
 *     unavailable resource;
 *     incompatible target.
 *
 * Parser syntax and semantic validation MUST remain separate.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Capability syntax MUST NOT itself grant authority.
 *
 * For example:
 *
 *     provides security::authorization;
 *
 * does not authorize anything.
 *
 * Likewise:
 *
 *     requires security::authentication;
 *
 * does not perform authentication.
 *
 * Credentials, secrets, private keys and authorization decisions belong to the
 * security/runtime layers.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar contains no semantic work.
 *
 * Complexity is therefore determined by:
 *
 *     - source size;
 *     - token count;
 *     - nesting depth;
 *     - expression complexity.
 *
 * No grammar-level finite capacity is imposed.
 *
 * Implementations MUST avoid introducing artificial domain limits.
 *
 * Any parser/resource protection belongs to the compiler's resource policy,
 * not this language grammar.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The test suite MUST verify:
 *
 *     - one capability;
 *     - many capabilities;
 *     - deeply qualified capabilities;
 *     - parameterized capabilities;
 *     - many predicate terms;
 *     - nested predicates;
 *     - many contract members;
 *     - many set members;
 *     - many negotiation members;
 *     - large expressions;
 *     - large source programs;
 *     - arbitrary namespace depth.
 *
 * There must be NO test asserting a maximum number of:
 *
 *     capabilities;
 *     requirements;
 *     provisions;
 *     constraints;
 *     preferences;
 *     endpoints;
 *     nodes;
 *     devices;
 *     machines;
 *     network links.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * requires networking::reliable_delivery;
 *
 * requires networking::latency <= latency_budget;
 *
 * requires networking::throughput >= required_throughput;
 *
 * requires networking::reliable_delivery
 *     and networking::ordered_delivery;
 *
 * requires networking::reliable_delivery
 *     or networking::best_effort;
 *
 * requires (
 *     networking::reliable_delivery
 *     and networking::secure_transport
 * );
 *
 * provides networking::message_delivery;
 *
 * constraint networking::latency <= maximum_latency;
 *
 * preference networking::low_latency;
 *
 * requires networking::throughput(required_rate);
 *
 * requires networking::reliable_delivery is true;
 *
 * capabilities {
 *     networking::reliable_delivery;
 *     networking::ordered_delivery;
 *     networking::multicast;
 * }
 *
 * capability_contract {
 *     requires networking::reliable_delivery;
 *     provides networking::ordered_delivery;
 *     constraint networking::latency <= latency_budget;
 *     preference networking::low_latency;
 * }
 *
 * negotiate {
 *     networking::reliable_delivery;
 *     networking::best_effort;
 * }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * requires;
 *
 * requires;
 *
 * provides;
 *
 * constraint;
 *
 * preference;
 *
 * requires networking::reliable_delivery
 *
 * requires: networking::reliable_delivery;
 *
 * requires networking::reliable_delivery <;
 *
 * requires networking::reliable_delivery and;
 *
 * requires (networking::reliable_delivery;
 *
 * requires networking::reliable_delivery(
 *
 * capabilities {
 *     networking::reliable_delivery
 * }
 *
 * capability_contract {
 *     requires networking::reliable_delivery
 * }
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     - empty contracts;
 *     - empty sets;
 *     - empty negotiation blocks;
 *     - one member;
 *     - many members;
 *     - deeply nested predicate groups;
 *     - deeply qualified capability names;
 *     - large argument lists;
 *     - large expressions;
 *     - long source files.
 *
 * Empty structures are syntactically accepted where the surrounding semantic
 * contract permits them; semantic validation determines whether an empty
 * construct is meaningful.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The same token stream MUST produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware;
 *     - network state;
 *     - environment variables;
 *     - time;
 *     - randomness;
 *     - capability discovery;
 *     - runtime state.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Required result:
 *
 *     PASS
 *
 * No universal capacity constants are represented by this grammar.
 *
 * No fixed:
 *
 *     qubit count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     node count;
 *     device count;
 *     memory size;
 *     thread count;
 *     network size;
 *     endpoint count;
 *     capability count;
 *     bandwidth;
 *     latency;
 *     topology size
 *
 * is encoded.
 *
 * Capability identity is open-world through canonical capability references.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It is the sole owner of networking capability syntax.
 *
 * [ ] General capability identity remains owned by core/capabilities.g4.
 *
 * [ ] Names remain owned by core/names.g4.
 *
 * [ ] Expressions remain owned by expressions/expressions.g4.
 *
 * [ ] No bare capability predicate is exposed as a top-level networking
 *     construct.
 *
 * [ ] Capability identities are open-world.
 *
 * [ ] Requirements are distinct from provisions.
 *
 * [ ] Requirements are distinct from preferences.
 *
 * [ ] Constraints are distinct from preferences.
 *
 * [ ] Capability negotiation remains source intent only.
 *
 * [ ] Capability syntax performs no discovery.
 *
 * [ ] Capability syntax performs no allocation.
 *
 * [ ] Capability syntax performs no routing.
 *
 * [ ] Capability syntax performs no scheduling.
 *
 * [ ] Capability syntax performs no target selection.
 *
 * [ ] Capability syntax creates no IR.
 *
 * [ ] No second quantum IR exists.
 *
 * [ ] Source spans remain recoverable.
 *
 * [ ] Diagnostics remain deterministic.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] Rust 1.97 / 1.97.1 compatibility is preserved.
 *
 * [ ] Generated parser integration requires no unsafe Rust.
 *
 * ============================================================================
 */