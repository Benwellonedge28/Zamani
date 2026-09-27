/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/network-capabilities.g4
 *
 * Grammar:
 *     NetworkCapabilities
 *
 * Status:
 *     PRODUCTION COMPONENT GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
 *
 * Safety:
 *     This grammar contains no embedded Rust, actions, semantic predicates,
 *     filesystem access, network access, hardware access, runtime callbacks,
 *     or unsafe code.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical networking-domain SOURCE-SYNTAX component for
 * expressing communication capabilities and capability intent.
 *
 * It provides syntax for:
 *
 *     - capability requirements;
 *     - capability provisions;
 *     - capability constraints;
 *     - capability preferences;
 *     - capability contracts;
 *     - capability sets;
 *     - capability negotiation intent;
 *     - capability predicates;
 *     - parameterized capability references;
 *     - capability metadata.
 *
 * Capability identity is OPEN-WORLD.
 *
 * This grammar deliberately does NOT enumerate networking technologies,
 * hardware vendors, machines, devices, protocols, transports, or topology.
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
 *     ZamaniParser / Networking
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural analysis
 *          |
 *          v
 *     name / capability / type / effect / resource analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *       classical             quantum::ir          HDL/hardware
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         optimization / lowering
 *                                 |
 *                         routing / scheduling
 *                                 |
 *                         resilience / QEC / ZQN
 *                                 |
 *                                 v
 *                                HAL
 *                                 |
 *                                 v
 *                          target realization
 *
 * This grammar:
 *
 *     - parses source intent;
 *     - preserves structure;
 *     - creates no IR;
 *     - performs no discovery;
 *     - performs no allocation;
 *     - performs no routing;
 *     - performs no scheduling;
 *     - performs no runtime negotiation;
 *     - selects no target.
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     networkCapabilityConstruct
 *     networkCapabilityRequirement
 *     networkCapabilityProvision
 *     networkCapabilityConstraint
 *     networkCapabilityPreference
 *     networkCapabilityPredicate
 *     networkCapabilityContract
 *     networkCapabilitySet
 *     networkCapabilityNegotiation
 *     networkCapabilityCall
 *     networkCapabilityPresence
 *     networkCapabilityMetadata
 *     the networking-capability predicate hierarchy.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical tokens;
 *     identifiers;
 *     qualified names;
 *     general expression precedence;
 *     capability declaration syntax;
 *     capability identity/version syntax;
 *     endpoints;
 *     addresses;
 *     protocols;
 *     sockets;
 *     channels;
 *     services;
 *     messages;
 *     routing;
 *     distributed placement;
 *     hardware discovery;
 *     resource allocation;
 *     security implementation;
 *     quantum IR;
 *     classical IR;
 *     HDL IR;
 *     runtime behavior.
 *
 * ============================================================================
 * DEPENDENCY OWNERSHIP
 * ============================================================================
 *
 * Capability identity/version:
 *
 *     grammar/core/capabilities.g4
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Lexical vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         +--> grammar/lexer/tokens.g4
 *                 |
 *                 +--> keywords.g4
 *                 +--> operators.g4
 *                 +--> punctuation.g4
 *                 +--> identifiers.g4
 *                 +--> literals
 *
 * Networking aggregate:
 *
 *     grammar/networking/networking.g4
 *
 * Networking consumers:
 *
 *     endpoints.g4
 *     channels.g4
 *     protocols.g4
 *     services.g4
 *     routing.g4
 *     distributed-compute.g4
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * A capability reference is a semantic name.
 *
 * Examples:
 *
 *     networking::reliable_delivery
 *     networking::ordered_delivery
 *     networking::low_latency
 *     networking::high_throughput
 *     networking::multicast
 *     networking::streaming
 *     networking::full_duplex
 *
 * Cross-domain examples are also legal:
 *
 *     quantum::communication
 *     quantum::entanglement_distribution
 *     hardware::communication_fabric
 *     security::confidentiality
 *     future::networking::new_capability
 *
 * This grammar assigns NO intrinsic meaning to any namespace.
 *
 * New capabilities MUST NOT require a grammar modification merely because
 * their names are new.
 *
 * ============================================================================
 * REQUIREMENT / PROVISION / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * Requirement:
 *
 *     requires networking::reliable_delivery;
 *
 * A requirement is mandatory semantic intent.
 *
 * Provision:
 *
 *     provides networking::reliable_delivery;
 *
 * A provision describes capability exposed by a logical declaration or
 * contract. It does NOT grant authority or allocate a physical resource.
 *
 * Constraint:
 *
 *     constraint networking::latency <= latency_budget;
 *
 * A constraint restricts legal realizations.
 *
 * Preference:
 *
 *     preference networking::low_latency;
 *
 * A preference is advisory.
 *
 * A preference MUST NOT silently become a requirement.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability syntax describes WHAT is required or preferred.
 *
 * It MUST NOT prescribe:
 *
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular QPU;
 *     - a particular NIC;
 *     - a particular router;
 *     - a particular physical link;
 *     - a particular provider;
 *     - a particular network topology;
 *     - a fixed number of nodes;
 *     - a fixed number of endpoints;
 *     - a fixed number of channels.
 *
 * Therefore the same source-level capability contract can be realized using:
 *
 *     in-process communication;
 *     shared memory;
 *     IPC;
 *     local networking;
 *     distributed networking;
 *     accelerator fabrics;
 *     quantum networking;
 *     future communication substrates.
 *
 * Target realization occurs downstream.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode universal limits such as:
 *
 *     MAX_NETWORK_CAPABILITIES
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_SERVICES
 *     MAX_PROTOCOLS
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_THREADS
 *     MAX_MEMORY
 *
 * It MUST NOT create universal physical identifiers such as:
 *
 *     node_0
 *     router_0
 *     interface_0
 *     gpu_0
 *     qpu_0
 *
 * as grammar-level concepts.
 *
 * Ordinary identifiers may contain such spellings when they are meaningful to
 * an application, but this grammar gives them no special physical meaning.
 *
 * ============================================================================
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * Numeric values and expressions are PROGRAM SEMANTICS.
 *
 * For example:
 *
 *     requires networking::throughput >= required_throughput;
 *
 *     requires networking::latency <= latency_budget;
 *
 * does not establish a universal bandwidth or latency limit.
 *
 * Actual feasibility is determined downstream from:
 *
 *     semantic analysis;
 *     resource analysis;
 *     capability discovery;
 *     target analysis;
 *     compilation;
 *     scheduling;
 *     deployment;
 *     runtime.
 *
 * The grammar imposes no finite cardinality on capability terms, contract
 * members, set members, negotiation members, or qualified-name depth.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * A capability such as:
 *
 *     security::authentication
 *     security::authorization
 *     security::confidentiality
 *     security::integrity
 *
 * is merely source-level semantic intent.
 *
 * This grammar does NOT:
 *
 *     - authenticate;
 *     - authorize;
 *     - generate keys;
 *     - access credentials;
 *     - validate certificates;
 *     - establish trust;
 *     - implement cryptography.
 *
 * Those responsibilities remain downstream.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking capabilities may describe quantum communication intent:
 *
 *     quantum::communication
 *     quantum::entanglement_distribution
 *     quantum::quantum_networking
 *
 * This grammar does NOT define:
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
 * Quantum semantics eventually use the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR is introduced here.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Networking capabilities may refer to semantic hardware capabilities:
 *
 *     hardware::communication_fabric
 *     hardware::dma
 *     hardware::network_acceleration
 *
 * This grammar does NOT define:
 *
 *     pins;
 *     wires;
 *     fixed bus widths;
 *     physical links;
 *     clock frequencies;
 *     device counts;
 *     physical topology.
 *
 * Those belong to hardware/HDL semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token stream;
 *     - grammar;
 *     - language-version context supplied by the parser.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware;
 *     - network state;
 *     - capability discovery;
 *     - environment variables;
 *     - filesystem state;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime state;
 *     - deployment topology.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must be able to retain source spans for:
 *
 *     - complete construct;
 *     - construct marker;
 *     - capability reference;
 *     - version clause;
 *     - predicate operator;
 *     - predicate operands;
 *     - call;
 *     - call arguments;
 *     - metadata;
 *     - contract members;
 *     - set members;
 *     - negotiation members;
 *     - grouping.
 *
 * The grammar does not define the Rust AST types.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual frontend AST mappings:
 *
 *     networkCapabilityReference
 *         -> CapabilityReference
 *
 *     networkCapabilityRequirement
 *         -> CapabilityRequirement
 *
 *     networkCapabilityProvision
 *         -> CapabilityProvision
 *
 *     networkCapabilityConstraint
 *         -> CapabilityConstraint
 *
 *     networkCapabilityPreference
 *         -> CapabilityPreference
 *
 *     networkCapabilityPredicate
 *         -> CapabilityPredicate
 *
 *     networkCapabilityCall
 *         -> CapabilityCall
 *
 *     networkCapabilityPresence
 *         -> CapabilityPresence
 *
 *     networkCapabilityContract
 *         -> CapabilityContract
 *
 *     networkCapabilitySet
 *         -> CapabilitySet
 *
 *     networkCapabilityNegotiation
 *         -> CapabilityNegotiation
 *
 *     networkCapabilityMetadata
 *         -> CapabilityMetadata
 *
 * The actual AST implementation remains under the frontend AST architecture.
 *
 * AST nodes MUST NOT contain:
 *
 *     - selected hardware;
 *     - allocated bandwidth;
 *     - selected network interface;
 *     - selected route;
 *     - runtime capability token;
 *     - device state.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - capability resolution;
 *     - namespace resolution;
 *     - version compatibility;
 *     - capability existence;
 *     - capability applicability;
 *     - requirement satisfiability;
 *     - provision validation;
 *     - constraint validation;
 *     - preference interpretation;
 *     - contradiction detection;
 *     - resource implications;
 *     - security implications;
 *     - distributed implications;
 *     - target feasibility;
 *     - portability analysis.
 *
 * The parser performs NONE of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Capability information passes through the frontend semantic model and can
 * subsequently contribute to:
 *
 *     - networking semantics;
 *     - distributed semantics;
 *     - classical semantics;
 *     - hardware semantics;
 *     - quantum semantics;
 *     - quantum::ir where relevant.
 *
 * No NetworkingCapabilityIR is defined here.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Downstream compiler stages may use capability information to:
 *
 *     - reject infeasible targets;
 *     - constrain legal routing;
 *     - constrain legal scheduling;
 *     - select legal communication mechanisms;
 *     - select legal serialization strategies;
 *     - establish security obligations;
 *     - establish distributed execution requirements;
 *     - determine target compatibility.
 *
 * These are compiler decisions, not parser decisions.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime MAY perform:
 *
 *     capability discovery;
 *     endpoint discovery;
 *     transport selection;
 *     service binding;
 *     negotiation;
 *     adaptation;
 *     environment inspection.
 *
 * This grammar performs none of those actions.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The file MUST NOT define lexer rules.
 *
 * Important current lexical token names include:
 *
 *     REQUIRES
 *     CONSTRAINT
 *     IS
 *     AND
 *     OR
 *     NOT
 *     TRUE
 *     FALSE
 *
 * and comparison operators:
 *
 *     LESS
 *     LESS_EQUAL
 *     GREATER
 *     GREATER_EQUAL
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *
 * Structural tokens include the canonical:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     SEMI
 *
 * This grammar deliberately does NOT rename those tokens or create parser-side
 * aliases for them.
 *
 * ============================================================================
 * CAPABILITY VERSION CONTRACT
 * ============================================================================
 *
 * Capability references reuse:
 *
 *     capabilityReference
 *
 * from:
 *
 *     grammar/core/capabilities.g4
 *
 * Therefore version syntax is NOT duplicated here.
 *
 * Examples supported through the canonical capability reference:
 *
 *     networking::reliable_delivery
 *
 *     networking::reliable_delivery version >= 1.2
 *
 *     future::networking::capability version [1.0, 2.0]
 *
 * Version compatibility remains semantic.
 *
 * ============================================================================
 * GENERAL EXPRESSION CONTRACT
 * ============================================================================
 *
 * Capability arguments and capability comparison values reuse:
 *
 *     expression
 *
 * from the canonical expression grammar.
 *
 * This grammar MUST NOT define a second arithmetic, logical, or precedence
 * hierarchy for ordinary Zamani expressions.
 *
 * The capability predicate hierarchy exists only because capability predicates
 * have a distinct source-level contract and must remain distinguishable from
 * ordinary expressions.
 *
 * ============================================================================
 */

parser grammar NetworkCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    Capabilities
    ;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Exactly one public component construct is exposed.
 *
 * The aggregate networking grammar consumes this rule.
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
 * Canonical capability identity and version syntax belong to Core/Capabilities.
 *
 * This wrapper gives networking grammar a stable semantic boundary without
 * duplicating capability syntax.
 */
networkCapabilityReference
    : capabilityReference
    ;


/* ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Mandatory capability intent.
 *
 * Examples:
 *
 *     requires networking::reliable_delivery;
 *
 *     requires networking::latency <= latency_budget;
 *
 *     requires networking::reliable_delivery
 *          and networking::ordered_delivery;
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
 * Capability exposed by a logical source-level declaration or contract.
 *
 * Provision is NOT authorization.
 *
 * Provision is NOT physical allocation.
 *
 * Provision is NOT capability discovery.
 *
 * NOTE:
 *
 * `PROVIDES` is part of the required lexical integration contract described
 * below. It must be added to the canonical keyword vocabulary before this
 * component is generated.
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
 * Restricts legal realizations.
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
 * Advisory preference.
 *
 * A preference MUST remain distinguishable from a requirement.
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
 * Dedicated capability predicate hierarchy.
 *
 * Precedence:
 *
 *     NOT
 *       >
 *     AND
 *       >
 *     OR
 *
 * Parentheses override the hierarchy.
 *
 * Capability identity remains open-world.
 */
networkCapabilityPredicate
    : networkCapabilityDisjunction
    ;


networkCapabilityDisjunction
    : networkCapabilityConjunction
      (OR networkCapabilityConjunction)*
    ;


networkCapabilityConjunction
    : networkCapabilityUnary
      (AND networkCapabilityUnary)*
    ;


networkCapabilityUnary
    : NOT networkCapabilityUnary
    | networkCapabilityPrimary
    ;


/* ============================================================================
 * PRIMARY
 * ============================================================================
 *
 * Ordering is intentional:
 *
 *     comparison
 *     call
 *     presence
 *     reference
 *     group
 *
 * All alternatives beginning with capabilityReference remain structurally
 * distinguishable by their following token:
 *
 *     comparator
 *     LPAREN
 *     IS
 *     predicate terminator/context
 *
 * ANTLR adaptive prediction resolves the common prefix without semantic
 * predicates.
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
 * Capability-valued or resource-valued property compared against an ordinary
 * Zamani expression.
 *
 * Examples:
 *
 *     networking::latency <= latency_budget
 *
 *     networking::throughput >= required_throughput
 *
 *     networking::reliability == required_reliability
 *
 *     networking::availability != unavailable
 *
 * The RHS belongs to the canonical expression grammar.
 */
networkCapabilityComparison
    : networkCapabilityReference
      networkCapabilityComparator
      expression
    ;


networkCapabilityComparator
    : LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    | EQUAL_EQUAL
    | NOT_EQUAL
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
 * Whether a particular capability is callable is a semantic question.
 */
networkCapabilityCall
    : networkCapabilityReference
      LPAREN
      networkCapabilityArgumentList?
      RPAREN
    ;


networkCapabilityArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * PRESENCE
 * ============================================================================
 *
 * Explicit boolean presence/availability test.
 *
 * Examples:
 *
 *     networking::reliable_delivery is true
 *
 *     networking::reliable_delivery is false
 *
 * Semantic analysis determines whether the referenced capability supports
 * this operation.
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
 *
 * Parentheses establish explicit capability-predicate grouping.
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
 * Contract cardinality is unbounded by language design.
 *
 * NOTE:
 *
 * `CAPABILITY_CONTRACT` is a required lexical integration token.
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
 * Example:
 *
 *     capabilities {
 *         networking::reliable_delivery;
 *         networking::ordered_delivery;
 *         networking::multicast;
 *     }
 *
 * A set is syntactic grouping only.
 *
 * It does not establish runtime availability.
 *
 * NOTE:
 *
 * `CAPABILITIES` is a required lexical integration token.
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
 * CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * Negotiation describes SOURCE INTENT.
 *
 * It does NOT execute negotiation.
 *
 * Example:
 *
 *     negotiate {
 *         networking::reliable_delivery;
 *         networking::ordered_delivery;
 *         networking::best_effort;
 *     }
 *
 * The runtime/compiler may later realize this intent against actual
 * capabilities.
 *
 * NOTE:
 *
 * `NEGOTIATE` is a required lexical integration token.
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
 * METADATA
 * ============================================================================
 *
 * Metadata remains open-world.
 *
 * Example:
 *
 *     property priority: preferred;
 *
 *     property priority = preferred;
 *
 * Metadata has no intrinsic runtime meaning at the grammar layer.
 *
 * `PROPERTY` is an existing canonical keyword token.
 */
networkCapabilityMetadata
    : PROPERTY
      identifier
      networkCapabilityMetadataValue?
      SEMI
    ;


networkCapabilityMetadataValue
    : COLON expression
    | ASSIGN expression
    ;


/* ============================================================================
 * REUSABLE INTEGRATION ADAPTERS
 * ============================================================================
 *
 * IMPORTANT:
 *
 * These adapters have UNIQUE names.
 *
 * The previous implementation contained invalid self-recursive rules:
 *
 *     networkingCapabilityRequirement
 *         : networkCapabilityRequirement
 *         ;
 *
 *     networkingCapabilityPredicate
 *         : networkCapabilityPredicate
 *         ;
 *
 * Those rules were removed.
 *
 * A component grammar must never define an adapter that recursively references
 * itself. Such a rule does not adapt anything and can make the grammar
 * unusable.
 *
 * These adapters instead expose the canonical component rules under
 * networking-domain names.
 */


/*
 * Requirement adapter.
 *
 * Consumers that already own their surrounding declaration may use:
 *
 *     networkingCapabilityRequirement
 *
 * without duplicating requirement syntax.
 */
networkingCapabilityRequirement
    : networkCapabilityRequirement
    ;


/*
 * Provision adapter.
 */
networkingCapabilityProvision
    : networkCapabilityProvision
    ;


/*
 * Constraint adapter.
 */
networkingCapabilityConstraint
    : networkCapabilityConstraint
    ;


/*
 * Preference adapter.
 */
networkingCapabilityPreference
    : networkCapabilityPreference
    ;


/*
 * Contract adapter.
 */
networkingCapabilityContract
    : networkCapabilityContract
    ;


/*
 * Set adapter.
 */
networkingCapabilitySet
    : networkCapabilitySet
    ;


/*
 * Negotiation adapter.
 */
networkingCapabilityNegotiation
    : networkCapabilityNegotiation
    ;


/*
 * Predicate adapter.
 */
networkingCapabilityPredicate
    : networkCapabilityPredicate
    ;


/* ============================================================================
 * INTEGRATION SEMANTICS
 * ============================================================================
 *
 * ENDPOINT
 *
 *     endpoint
 *         |
 *         +--> capability contract
 *
 * Endpoint identity and endpoint configuration remain owned by endpoints.g4.
 *
 *
 * CHANNEL
 *
 *     channel
 *         |
 *         +--> capability contract
 *
 * Channel identity and channel semantics remain owned by channels.g4.
 *
 *
 * PROTOCOL
 *
 *     protocol
 *         |
 *         +--> capability contract
 *
 * Protocol identity and protocol syntax remain owned by protocols.g4.
 *
 *
 * SERVICE
 *
 *     service
 *         |
 *         +--> capability contract
 *
 * Service identity and service syntax remain owned by services.g4.
 *
 *
 * ROUTING
 *
 *     route
 *         |
 *         +--> capability requirement / constraint
 *
 * Routing itself remains owned by routing.g4.
 *
 *
 * DISTRIBUTED COMPUTING
 *
 *     distributed computation
 *         |
 *         +--> capability requirement
 *
 * Distributed placement, membership, replication and scheduling remain outside
 * this grammar.
 *
 *
 * HARDWARE
 *
 *     hardware intent
 *         |
 *         +--> capability requirement
 *
 * Hardware realization remains outside this grammar.
 *
 *
 * QUANTUM
 *
 *     quantum computation
 *         |
 *         +--> networking capability requirement
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum::ir
 *
 * No quantum implementation information is introduced here.
 *
 * ============================================================================
 * FRONTEND INTEGRATION
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     source span
 *     source ordering
 *     capability identity
 *     capability version
 *     predicate structure
 *     comparator
 *     RHS expression
 *     argument ordering
 *     metadata
 *     contract membership
 *     negotiation membership
 *
 * No syntactic information required for diagnostics or semantic analysis may
 * be silently discarded.
 *
 * ============================================================================
 * SEMANTIC VALIDATION
 * ============================================================================
 *
 * The semantic layer MUST determine:
 *
 *     - whether the capability exists;
 *     - whether its namespace is valid;
 *     - whether its version constraint is satisfiable;
 *     - whether the capability is applicable;
 *     - whether requirements conflict;
 *     - whether provisions are legal;
 *     - whether constraints are satisfiable;
 *     - how preferences affect realization;
 *     - whether arguments have valid types;
 *     - whether a capability can be presence-tested;
 *     - whether a target satisfies the resulting contract.
 *
 * The semantic layer MUST distinguish:
 *
 *     source invalidity
 *
 * from:
 *
 *     target infeasibility.
 *
 * For example, a syntactically valid program may fail on a target because a
 * required networking capability is unavailable. That is not a parser error.
 *
 * ============================================================================
 * REQUIREMENT / TARGET SEPARATION
 * ============================================================================
 *
 * Valid:
 *
 *     requires networking::reliable_delivery;
 *
 *     requires networking::throughput >= required_throughput;
 *
 *     requires networking::latency <= latency_budget;
 *
 * Invalid as a universal source-language resource model:
 *
 *     requires router_0;
 *
 *     requires gpu_0;
 *
 *     requires node_0;
 *
 *     requires exactly_some_fixed_physical_interface;
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * CAPABILITY VS RESOURCE
 * ============================================================================
 *
 * Capability:
 *
 *     what an execution context can do.
 *
 * Resource:
 *
 *     what computational/physical quantity is available or requested.
 *
 * Requirement:
 *
 *     what must be satisfied.
 *
 * Constraint:
 *
 *     what legal realizations must obey.
 *
 * Preference:
 *
 *     which legal realization is desirable.
 *
 * This grammar preserves those distinctions syntactically.
 *
 * ============================================================================
 * CAPABILITY VS AUTHORITY
 * ============================================================================
 *
 * The presence of a capability name does not grant authority.
 *
 * For example:
 *
 *     provides security::authorization;
 *
 * does not grant authorization.
 *
 * Likewise:
 *
 *     requires security::authentication;
 *
 * does not perform authentication.
 *
 * Security enforcement belongs to downstream security/runtime systems.
 *
 * ============================================================================
 * CAPABILITY VS DISCOVERY
 * ============================================================================
 *
 * The source program may request:
 *
 *     requires networking::reliable_delivery;
 *
 * Discovery answers later:
 *
 *     "Does the current execution environment provide it?"
 *
 * The parser MUST NOT answer that question.
 *
 * ============================================================================
 * CAPABILITY VS ROUTING
 * ============================================================================
 *
 * A capability such as:
 *
 *     networking::low_latency
 *
 * may constrain routing.
 *
 * It does not itself select a route.
 *
 * Route selection remains a downstream optimization/routing decision.
 *
 * ============================================================================
 * CAPABILITY VS SCHEDULING
 * ============================================================================
 *
 * A capability such as:
 *
 *     networking::ordered_delivery
 *
 * may constrain scheduling.
 *
 * It does not encode a scheduler implementation.
 *
 * ============================================================================
 * CAPABILITY VS TOPOLOGY
 * ============================================================================
 *
 * This grammar does not encode physical topology.
 *
 * A semantic requirement such as:
 *
 *     networking::low_latency
 *
 * may eventually be evaluated against topology.
 *
 * Topology discovery and realization remain downstream.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser-level errors include:
 *
 *     requires;
 *     provides;
 *     constraint;
 *     preference;
 *     capability_contract;
 *     capabilities;
 *     negotiate;
 *     malformed capability reference;
 *     malformed comparison;
 *     malformed argument list;
 *     malformed predicate;
 *     missing closing delimiter;
 *     missing semicolon.
 *
 * Semantic errors include:
 *
 *     unknown capability;
 *     unsupported capability;
 *     incompatible capability version;
 *     invalid capability argument;
 *     contradictory requirements;
 *     impossible constraint;
 *     invalid presence test;
 *     unavailable capability on selected target.
 *
 * Parser and semantic errors MUST remain separate.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * Grammar complexity grows with source structure:
 *
 *     O(tokens)
 *
 * for the ordinary linear predicate/list structures represented here, subject
 * to the behavior of the canonical expression grammar used by RHS expressions
 * and arguments.
 *
 * No grammar-level cardinality bound exists.
 *
 * Implementations MAY impose resource-exhaustion protections, parser budgets,
 * recursion protections, or deployment limits, but those MUST remain
 * implementation/resource policy rather than language semantics.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar must support, without syntax redesign:
 *
 *     one capability;
 *     many capabilities;
 *     arbitrarily many contract members;
 *     arbitrarily many set members;
 *     arbitrarily many negotiation members;
 *     deeply qualified names;
 *     parameterized capabilities;
 *     large predicate expressions;
 *     large argument lists;
 *     large source programs.
 *
 * No test may assert a universal maximum for:
 *
 *     capabilities;
 *     requirements;
 *     provisions;
 *     constraints;
 *     preferences;
 *     contracts;
 *     endpoints;
 *     nodes;
 *     links;
 *     devices.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical token streams and identical grammar/version context:
 *
 *     parse(A) == parse(A)
 *
 * structurally.
 *
 * Repeated parsing MUST NOT perform discovery or access external state.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Capability identity and version syntax remain owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Therefore this file must not duplicate capability version syntax.
 *
 * Networking aggregation remains owned by:
 *
 *     grammar/networking/networking.g4
 *
 * That aggregate imports this component as:
 *
 *     NetworkCapabilities
 *
 * The wider parser imports:
 *
 *     Networking
 *
 * through:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * `Zamani.g4` remains the complete-program composition root.
 *
 * ============================================================================
 * CANONICAL LOWERING CONTRACT
 * ============================================================================
 *
 * Source:
 *
 *     networking capability intent
 *
 *         |
 *         v
 *     frontend AST
 *
 *         |
 *         v
 *     semantic capability/resource model
 *
 *         |
 *         +--------------------+---------------------+
 *         |                    |                     |
 *     networking          distributed          hardware
 *         |                    |                     |
 *         +--------------------+---------------------+
 *                              |
 *                         target analysis
 *                              |
 *                    optimization / routing
 *                              |
 *                         scheduling
 *                              |
 *                       runtime / HAL
 *
 * If the capability participates in quantum computation:
 *
 *     semantic quantum model
 *             |
 *             v
 *         quantum::ir
 *
 * No direct grammar-to-quantum-IR dependency exists.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * The generated and consuming Zamani implementation must remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * No `unsafe` Rust is required by this grammar.
 *
 * ============================================================================
 * REQUIRED LEXICAL INTEGRATION
 * ============================================================================
 *
 * The current repository's canonical lexer already supplies:
 *
 *     REQUIRES
 *     CONSTRAINT
 *     PROPERTY
 *     IS
 *     AND
 *     OR
 *     NOT
 *     TRUE
 *     FALSE
 *
 * and the canonical comparison tokens:
 *
 *     LESS
 *     LESS_EQUAL
 *     GREATER
 *     GREATER_EQUAL
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *
 * However, the complete capability-contract syntax in this file additionally
 * requires these RESERVED keyword tokens:
 *
 *     PROVIDES
 *     PREFERENCE
 *     CAPABILITY_CONTRACT
 *     CAPABILITIES
 *     NEGOTIATE
 *
 * These MUST be added to the canonical:
 *
 *     grammar/lexer/keywords.g4
 *
 * before this grammar can be generated with the complete contract syntax.
 *
 * Required lexical spellings:
 *
 *     PROVIDES           : 'provides' ;
 *     PREFERENCE         : 'preference' ;
 *     CAPABILITY_CONTRACT: 'capability_contract' ;
 *     CAPABILITIES       : 'capabilities' ;
 *     NEGOTIATE          : 'negotiate' ;
 *
 * These are lexical tokens only.
 *
 * They do not perform semantic work.
 *
 * They do not perform capability discovery.
 *
 * They do not grant authority.
 *
 * They do not select hardware.
 *
 * They do not establish resource limits.
 *
 * They do not create a second capability registry.
 *
 * ============================================================================
 * REQUIRED NETWORKING INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/networking/networking.g4
 *
 * already imports:
 *
 *     NetworkCapabilities
 *
 * and exposes:
 *
 *     networkCapabilityConstruct
 *
 * through:
 *
 *     networkingConstruct
 *
 * Therefore this file does NOT require a new networking aggregate grammar.
 *
 * Do NOT create:
 *
 *     networking-capability-root.g4
 *     network-capabilities-root.g4
 *     NetworkCapabilityParser.g4
 *
 * as competing authorities.
 *
 * ============================================================================
 * REQUIRED ENDPOINT / CHANNEL / SERVICE INTEGRATION
 * ============================================================================
 *
 * Existing networking component grammars may consume:
 *
 *     networkCapabilityRequirement
 *     networkCapabilityProvision
 *     networkCapabilityConstraint
 *     networkCapabilityPreference
 *     networkCapabilityContract
 *     networkCapabilityPredicate
 *
 * through the NetworkCapabilities import.
 *
 * They MUST NOT copy the predicate implementation.
 *
 * They MUST NOT create alternative capability identities.
 *
 * They MUST NOT introduce transport-specific capability enumerations.
 *
 * ============================================================================
 * REQUIRED CORE CAPABILITY INTEGRATION
 * ============================================================================
 *
 * `capabilityReference` remains the sole source-level capability identity
 * boundary.
 *
 * This file MUST NOT define:
 *
 *     networkCapabilityName
 *     networkCapabilityVersion
 *
 * as independent competing identity/version systems.
 *
 * ============================================================================
 * REQUIRED EXPRESSION INTEGRATION
 * ============================================================================
 *
 * All ordinary RHS values and call arguments use:
 *
 *     expression
 *
 * from the canonical expression grammar.
 *
 * Do not create:
 *
 *     networkExpression
 *
 * merely to duplicate general Zamani expression syntax.
 *
 * ============================================================================
 * REQUIRED AST INTEGRATION
 * ============================================================================
 *
 * The frontend AST implementation must map every public rule here.
 *
 * There must be no successful parser path that produces syntactic information
 * which the frontend silently discards.
 *
 * At minimum:
 *
 *     networkCapabilityRequirement
 *     networkCapabilityProvision
 *     networkCapabilityConstraint
 *     networkCapabilityPreference
 *     networkCapabilityContract
 *     networkCapabilitySet
 *     networkCapabilityNegotiation
 *
 * require explicit AST mapping.
 *
 * ============================================================================
 * REQUIRED SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Every AST capability node must have an explicit semantic-analysis path.
 *
 * Unsupported semantic behavior must produce a structured diagnostic rather
 * than silently dropping the construct.
 *
 * ============================================================================
 * REQUIRED IR INTEGRATION
 * ============================================================================
 *
 * Capability constructs themselves do not create an IR.
 *
 * If a capability affects a compiled operation, the semantic capability model
 * must attach the requirement/constraint/preference to the canonical semantic
 * operation/resource context before IR lowering.
 *
 * ============================================================================
 * REQUIRED TEST INTEGRATION
 * ============================================================================
 *
 * Tests should be maintained under:
 *
 *     grammar/tests/networking/
 *
 * and, where the repository's existing structure dictates, the corresponding
 * validation/conformance directories.
 *
 * Required categories:
 *
 *     positive
 *     negative
 *     boundary
 *     scalability
 *     determinism
 *     compatibility
 *     cross-domain
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms must parse after lexical integration:
 *
 *     requires networking::reliable_delivery;
 *
 *     requires networking::latency <= latency_budget;
 *
 *     requires networking::throughput >= required_throughput;
 *
 *     requires networking::reliable_delivery and
 *              networking::ordered_delivery;
 *
 *     requires networking::reliable_delivery or
 *              networking::best_effort;
 *
 *     requires not networking::legacy_transport;
 *
 *     requires (
 *         networking::reliable_delivery and
 *         networking::ordered_delivery
 *     );
 *
 *     provides networking::message_delivery;
 *
 *     constraint networking::latency <= maximum_latency;
 *
 *     preference networking::low_latency;
 *
 *     requires networking::throughput(required_rate);
 *
 *     requires networking::reliable_delivery is true;
 *
 *     capabilities {
 *         networking::reliable_delivery;
 *         networking::ordered_delivery;
 *         networking::multicast;
 *     }
 *
 *     capability_contract {
 *         requires networking::reliable_delivery;
 *         provides networking::ordered_delivery;
 *         constraint networking::latency <= latency_budget;
 *         preference networking::low_latency;
 *     }
 *
 *     negotiate {
 *         networking::reliable_delivery;
 *         networking::best_effort;
 *     }
 *
 * Cross-domain:
 *
 *     requires quantum::communication;
 *
 *     requires security::confidentiality;
 *
 *     requires hardware::communication_fabric;
 *
 *     requires future::networking::new_capability;
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * These must fail syntactically:
 *
 *     requires;
 *
 *     provides;
 *
 *     constraint;
 *
 *     preference;
 *
 *     requires: networking::reliable_delivery;
 *
 *     requires networking::reliable_delivery <
 *     ;
 *
 *     requires networking::reliable_delivery and;
 *
 *     requires (networking::reliable_delivery;
 *
 *     requires networking::throughput(
 *
 *     capabilities {
 *         networking::reliable_delivery
 *     }
 *
 *     capability_contract {
 *         requires networking::reliable_delivery
 *     }
 *
 *     negotiate {
 *         networking::reliable_delivery
 *     }
 *
 * The tests MUST also verify that malformed capability constructs do not
 * accidentally parse as unrelated networking declarations.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Verify:
 *
 *     empty contract;
 *     empty capability set;
 *     empty negotiation block;
 *     one member;
 *     many members;
 *     deeply nested predicate groups;
 *     deeply qualified capability names;
 *     parameterized capability calls;
 *     large argument lists;
 *     large predicates;
 *     large source units.
 *
 * Empty constructs are syntactically representable because cardinality policy
 * is a semantic concern unless the language specification later makes a
 * particular construct non-empty by syntax.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Generate capability contracts whose member count grows with available test
 * resources.
 *
 * The grammar must not contain a finite upper bound.
 *
 * Test progressively:
 *
 *     1
 *     many
 *     very many
 *
 * members without changing grammar semantics.
 *
 * Do not encode a maximum into the grammar merely because a test fixture has
 * a finite size.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS conditions:
 *
 *     - no MAX_NETWORK_* language constants;
 *     - no MAX_ENDPOINTS;
 *     - no MAX_NODES;
 *     - no MAX_DEVICES;
 *     - no MAX_CPUS;
 *     - no MAX_GPUS;
 *     - no MAX_FPGAS;
 *     - no MAX_QPUS;
 *     - no MAX_MEMORY;
 *     - no MAX_THREADS;
 *     - no fixed bandwidth;
 *     - no fixed latency;
 *     - no fixed topology;
 *     - no fixed transport list;
 *     - no fixed vendor list.
 *
 * Numeric values in expressions remain program values.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This component is complete when:
 *
 * [x] Capability identity is open-world.
 *
 * [x] Capability version syntax is delegated to core/capabilities.g4.
 *
 * [x] General names are delegated to the canonical names grammar.
 *
 * [x] General expressions are delegated to the canonical expression grammar.
 *
 * [x] Requirements are syntactically distinct from provisions.
 *
 * [x] Requirements are syntactically distinct from preferences.
 *
 * [x] Constraints are syntactically distinct from preferences.
 *
 * [x] Negotiation is source intent only.
 *
 * [x] No capability discovery occurs during parsing.
 *
 * [x] No resource allocation occurs during parsing.
 *
 * [x] No routing occurs during parsing.
 *
 * [x] No scheduling occurs during parsing.
 *
 * [x] No target selection occurs during parsing.
 *
 * [x] No hardware assumptions occur during parsing.
 *
 * [x] No quantum-specific IR is created.
 *
 * [x] No self-recursive adapter rules exist.
 *
 * [x] Canonical lexer token names are used for comparison operators.
 *
 * [x] Source structure remains recoverable for AST construction.
 *
 * [x] Deterministic parsing is preserved.
 *
 * [x] No Rust actions exist.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] No universal resource limit is encoded.
 *
 * [ ] Required lexical tokens have been integrated into keywords.g4.
 *
 * [ ] Network component grammars consume the public rules without duplication.
 *
 * [ ] Frontend AST mappings exist for every public construct.
 *
 * [ ] Semantic mappings exist for every public construct.
 *
 * [ ] Conformance tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */