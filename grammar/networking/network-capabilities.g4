/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Networking Capability Grammar
 * ============================================================================
 *
 * File:
 *     grammar/networking/network-capabilities.g4
 *
 * Grammar:
 *     NetworkingCapabilities
 *
 * Purpose:
 *     Canonical source-level grammar for networking-specific capability
 *     references, requirements, constraints, preferences, and capability
 *     predicates.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No environment inspection.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX that composes general Zamani capabilities
 * into networking-specific declarations and contracts.
 *
 * It does NOT own the general capability language.
 *
 * General capability identity/version syntax remains owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Networking-specific interpretation is represented by qualified capability
 * names and expressions.
 *
 * Examples:
 *
 *     networking::reliable_delivery
 *     networking::ordered_delivery
 *     networking::multicast
 *     networking::low_latency
 *     networking::high_throughput
 *     networking::secure_transport
 *     networking::quantum_communication
 *     networking::hardware_fabric
 *     future::networking::new_capability
 *
 * This grammar deliberately does NOT enumerate a closed list of capabilities.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - networking capability references;
 *     - networking capability requirements;
 *     - networking capability provisions;
 *     - networking capability predicates;
 *     - networking capability constraints;
 *     - networking capability preferences;
 *     - networking capability groups;
 *     - networking capability composition;
 *     - networking capability metadata;
 *     - networking capability conditions;
 *     - networking capability applicability clauses;
 *     - networking-specific capability contracts.
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
 *     - network discovery;
 *     - endpoint declarations;
 *     - channel declarations;
 *     - service declarations;
 *     - protocol declarations;
 *     - message schemas;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - transport implementation;
 *     - socket implementation;
 *     - IP/MAC addressing;
 *     - physical topology;
 *     - cryptographic implementation;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - runtime execution;
 *     - resilience;
 *     - backend selection.
 *
 * ============================================================================
 * FUNDAMENTAL SEPARATION
 * ============================================================================
 *
 * Capability:
 *
 *     What can an execution environment or communication substrate do?
 *
 * Requirement:
 *
 *     What must a valid realization provide?
 *
 * Provision:
 *
 *     What capability does a declaration expose?
 *
 * Constraint:
 *
 *     What conditions restrict valid realizations?
 *
 * Preference:
 *
 *     Which valid realization is preferred?
 *
 * Resource:
 *
 *     What physical/logical resource is available or requested?
 *
 * Target:
 *
 *     What execution context is being described?
 *
 * These concepts MUST remain separate.
 *
 * In particular:
 *
 *     networking::reliable_delivery
 *
 * MUST NOT imply:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     a particular socket
 *     a particular machine
 *     a particular network
 *     a particular number of nodes
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Networking capability syntax describes semantic communication requirements,
 * not temporary physical implementation details.
 *
 * A source program can therefore remain unchanged when the implementation
 * changes between:
 *
 *     in-process communication
 *     shared memory
 *     IPC
 *     local networking
 *     distributed networking
 *     accelerator fabric
 *     quantum networking
 *     future communication substrate
 *
 * The realization is selected downstream from the source grammar.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This file MUST NOT contain rules such as:
 *
 *     reliableCapability
 *     tcpCapability
 *     udpCapability
 *     quicCapability
 *     ethernetCapability
 *     infinibandCapability
 *     mpiCapability
 *     rdmaCapability
 *
 * as closed enumerations.
 *
 * Instead:
 *
 *     qualifiedName
 *
 * represents capability identity.
 *
 * This permits future capabilities without modifying this grammar.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *     - number of capabilities;
 *     - number of requirements;
 *     - number of provisions;
 *     - number of constraints;
 *     - number of preferences;
 *     - number of capability groups;
 *     - number of predicates;
 *     - number of operands;
 *     - number of networking declarations;
 *     - number of endpoints;
 *     - number of channels;
 *     - number of services;
 *     - number of protocols;
 *     - number of participants;
 *     - capability namespace depth.
 *
 * All collections use ANTLR repetition operators.
 *
 * Practical limits belong to:
 *
 *     - parser resource policy;
 *     - compiler resource policy;
 *     - semantic analysis;
 *     - resource management;
 *     - target capabilities;
 *     - runtime.
 *
 * ============================================================================
 * HARD-CODING POLICY
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_NETWORK_CAPABILITIES
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_SERVICES
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_NETWORK_SIZE
 *     MAX_PROTOCOLS
 *
 * Numeric values appearing inside expressions are program-level semantic
 * values, not grammar-level hardware limits.
 *
 * ============================================================================
 * NETWORKING / RESOURCE BOUNDARY
 * ============================================================================
 *
 * This file may reference resource/capability expressions.
 *
 * It MUST NOT define resource allocation syntax.
 *
 * For example, a networking requirement may express:
 *
 *     requires networking::throughput >= required_rate;
 *
 * but the grammar does not decide:
 *
 *     which NIC
 *     which router
 *     which device
 *     which transport
 *     which physical link
 *     which number of machines
 *
 * Resource realization belongs downstream.
 *
 * ============================================================================
 * NETWORKING / PROTOCOL BOUNDARY
 * ============================================================================
 *
 * Protocol identity belongs to:
 *
 *     grammar/networking/protocols.g4
 *
 * This file may reference protocol-related capabilities:
 *
 *     networking::reliable_delivery
 *
 * but does not define protocol syntax.
 *
 * ============================================================================
 * NETWORKING / CHANNEL BOUNDARY
 * ============================================================================
 *
 * Channel syntax belongs to:
 *
 *     grammar/networking/channels.g4
 *
 * This file may constrain channel capabilities but does not create channels.
 *
 * ============================================================================
 * NETWORKING / ENDPOINT BOUNDARY
 * ============================================================================
 *
 * Endpoint syntax belongs to:
 *
 *     grammar/networking/endpoints.g4
 *
 * This file may express endpoint capability requirements but does not define
 * endpoint identity or physical addressing.
 *
 * ============================================================================
 * NETWORKING / SERVICE BOUNDARY
 * ============================================================================
 *
 * Service syntax belongs to:
 *
 *     grammar/networking/services.g4
 *
 * A service may consume networking capability constructs defined here.
 *
 * This file does not redefine service declarations.
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING BOUNDARY
 * ============================================================================
 *
 * This grammar may be consumed by distributed networking declarations.
 *
 * It MUST NOT define:
 *
 *     nodes
 *     placement
 *     replication
 *     consensus
 *     cluster membership
 *     distributed scheduling
 *
 * Those belong to:
 *
 *     grammar/distributed/
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Networking capability references may name security capabilities:
 *
 *     security::confidentiality
 *     security::authentication
 *     security::integrity
 *
 * This file does not define cryptographic algorithms or security semantics.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking capabilities may describe quantum communication capabilities:
 *
 *     quantum::communication
 *     quantum::entanglement_distribution
 *     quantum::quantum_networking
 *
 * These are names only.
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumState
 *     quantum topology
 *     calibration
 *     QEC
 *     ZQN faults
 *
 * If networking interacts with quantum computation, semantic lowering
 * eventually reaches the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-facing networking capabilities may be referenced symbolically.
 *
 * Examples:
 *
 *     hardware::communication_fabric
 *     hardware::dma
 *     accelerator::networking
 *
 * This grammar does not define:
 *
 *     pins
 *     wires
 *     buses
 *     clocks
 *     FPGA routing
 *     ASIC cells
 *     physical topology
 *     fixed bus width
 *     fixed device count
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no I/O;
 *     - no runtime access;
 *     - no network access;
 *     - no hardware inspection;
 *     - no randomness.
 *
 * Equal token streams under the same grammar version produce equivalent parse
 * structures.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT define lexer rules.
 *
 * General capability syntax already relies on the canonical CAPABILITY keyword
 * and the canonical qualified-name model. The lexer defines the lexical
 * distinction between language keywords and identifiers. 
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Required conceptual dependencies:
 *
 *     Core
 *         - identifier
 *         - qualifiedName
 *         - attributes
 *
 *     Expressions
 *         - expression
 *
 *     Capabilities
 *         - capabilityReference
 *         - capabilityName
 *         - capabilityVersionClause
 *
 * Networking consumers:
 *
 *     endpoints.g4
 *     channels.g4
 *     protocols.g4
 *     services.g4
 *
 * This file MUST reuse capability syntax rather than reproduce the general
 * capability grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *     - declaration kind;
 *     - capability identity;
 *     - capability version requirement;
 *     - logical operator structure;
 *     - expressions;
 *     - attributes;
 *     - source ordering;
 *     - source spans.
 *
 * The AST MUST NOT contain:
 *
 *     - physical device identity;
 *     - resolved hardware;
 *     - runtime capability tokens;
 *     - resource allocation;
 *     - routing decisions;
 *     - scheduling decisions;
 *     - backend selection.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability names;
 *     - validating namespaces;
 *     - validating versions;
 *     - determining capability meaning;
 *     - checking capability availability;
 *     - evaluating capability predicates;
 *     - checking requirement satisfaction;
 *     - checking capability conflicts;
 *     - evaluating constraints;
 *     - ranking preferences;
 *     - integrating resource requirements;
 *     - integrating security requirements;
 *     - integrating distributed requirements;
 *     - determining target feasibility.
 *
 * None of those decisions belong to this parser grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * The semantic layer may lower its AST into the canonical semantic
 * representation used by the repository.
 *
 * If a capability affects quantum computation:
 *
 *     networking capability AST
 *          ->
 *     semantic analysis
 *          ->
 *     quantum semantic lowering
 *          ->
 *     quantum::ir
 *
 * The grammar never constructs quantum::ir.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use networking capability semantics to:
 *
 *     - validate target compatibility;
 *     - select legal implementation strategies;
 *     - constrain routing;
 *     - constrain scheduling;
 *     - select communication mechanisms;
 *     - select serialization strategies;
 *     - determine security obligations;
 *     - determine distributed execution requirements.
 *
 * Capability preferences MUST remain distinguishable from mandatory
 * requirements.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns:
 *
 *     - capability discovery;
 *     - environment inspection;
 *     - capability negotiation;
 *     - endpoint resolution;
 *     - transport selection;
 *     - service binding;
 *     - network establishment;
 *     - runtime adaptation.
 *
 * This grammar performs none of these actions.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may use this grammar for:
 *
 *     - syntax highlighting;
 *     - completion;
 *     - navigation;
 *     - capability references;
 *     - diagnostics;
 *     - documentation generation;
 *     - semantic queries;
 *     - dependency visualization.
 *
 * Tooling MUST NOT infer a physical machine merely from a capability name.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Capability identity is represented by qualified names.
 *
 * Therefore adding:
 *
 *     future::networking::new_capability
 *
 * does not require a grammar modification.
 *
 * Removing or renaming an existing capability is a semantic compatibility
 * concern and must be handled through the language compatibility system.
 *
 * ============================================================================
 */


/* ============================================================================
 * PARSER DECLARATION
 * ========================================================================== */

parser grammar NetworkingCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Expressions, Capabilities;


/* ============================================================================
 * PUBLIC COMPOSITION ENTRY POINT
 * ========================================================================== */

/*
 * Stable integration boundary for networking grammars.
 *
 * Consumers such as endpoints, channels, protocols and services should use
 * `networkCapabilityConstruct` rather than copying these rules.
 */
networkCapabilityConstruct
    : networkCapabilityRequirement
    | networkCapabilityProvision
    | networkCapabilityConstraint
    | networkCapabilityPreference
    | networkCapabilityPredicate
    | networkCapabilityGroup
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * CAPABILITY REFERENCE
 * ========================================================================== */

/*
 * A networking capability reference delegates identity/version syntax to the
 * canonical capability grammar.
 *
 * Examples:
 *
 *     networking::reliable_delivery
 *     networking::ordered_delivery
 *     quantum::communication
 *     security::confidentiality
 *     future::networking::new_capability
 */
networkCapabilityReference
    : capabilityReference
    ;


/* ============================================================================
 * CAPABILITY REQUIREMENT
 * ========================================================================== */

/*
 * A requirement is mandatory semantic intent.
 *
 * It does not allocate a resource.
 *
 * Examples:
 *
 *     requires networking::reliable_delivery;
 *
 *     requires networking::ordered_delivery;
 *
 *     requires quantum::communication;
 */
networkCapabilityRequirement
    : REQUIRES
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * CAPABILITY PROVISION
 * ========================================================================== */

/*
 * A provision states that a logical declaration exposes a capability.
 *
 * It does not grant runtime authorization.
 *
 * It does not prove that a physical target supports the capability.
 */
networkCapabilityProvision
    : PROVIDES
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * CAPABILITY CONSTRAINT
 * ========================================================================== */

/*
 * A constraint restricts legal realizations.
 *
 * Constraints are distinct from requirements and preferences.
 *
 * Examples:
 *
 *     constraint networking::latency < required_latency;
 *
 *     constraint networking::transport != forbidden_transport;
 *
 * Expressions are intentionally used for values rather than introducing
 * machine-specific grammar.
 */
networkCapabilityConstraint
    : CONSTRAINT
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * CAPABILITY PREFERENCE
 * ========================================================================== */

/*
 * A preference is advisory.
 *
 * A compiler/runtime may choose another valid realization if the preference
 * cannot be satisfied or conflicts with stronger requirements.
 */
networkCapabilityPreference
    : PREFERENCE
      networkCapabilityPredicate
      SEMI
    ;


/* ============================================================================
 * CAPABILITY PREDICATE
 * ========================================================================== */

/*
 * Capability predicates form the central semantic expression boundary.
 *
 * This deliberately does NOT enumerate networking capabilities.
 *
 * The capability identity remains open-world.
 */
networkCapabilityPredicate
    : networkCapabilityDisjunction
    ;


/* ============================================================================
 * DISJUNCTION
 * ========================================================================== */

networkCapabilityDisjunction
    : networkCapabilityConjunction
      (OR networkCapabilityConjunction)*
    ;


/* ============================================================================
 * CONJUNCTION
 * ========================================================================== */

networkCapabilityConjunction
    : networkCapabilityPrimary
      (AND networkCapabilityPrimary)*
    ;


/* ============================================================================
 * PRIMARY
 * ========================================================================== */

networkCapabilityPrimary
    : networkCapabilityReference
    | networkCapabilityComparison
    | networkCapabilityCall
    | networkCapabilityPresence
    | networkCapabilityGroup
    | NOT networkCapabilityPrimary
    ;


/* ============================================================================
 * CAPABILITY COMPARISON
 * ========================================================================== */

/*
 * Comparisons permit semantic capability/resource predicates without hardcoding
 * physical limits into the grammar.
 *
 * Examples:
 *
 *     networking::latency < max_latency
 *
 *     networking::throughput >= required_throughput
 *
 *     networking::reliability >= required_reliability
 *
 * The meaning of these values is semantic analysis.
 */
networkCapabilityComparison
    : networkCapabilityValue
      networkCapabilityComparator
      expression
    ;


/* ============================================================================
 * COMPARISON OPERATORS
 * ========================================================================== */

networkCapabilityComparator
    : LT
    | LE
    | GT
    | GE
    | EQ
    | NE
    ;


/* ============================================================================
 * CAPABILITY VALUE
 * ========================================================================== */

/*
 * A value may be:
 *
 *     - a capability reference;
 *     - a qualified semantic property;
 *     - an expression-resolved value.
 *
 * No physical representation is implied.
 */
networkCapabilityValue
    : networkCapabilityReference
    ;


/* ============================================================================
 * CAPABILITY CALL / PARAMETERIZATION
 * ========================================================================== */

/*
 * Parameterized capabilities are intentionally open-ended.
 *
 * Example:
 *
 *     networking::throughput(required_rate)
 *
 *     networking::latency(maximum)
 *
 *     networking::delivery(reliable)
 *
 * The semantic layer decides whether the referenced capability is actually
 * callable/parameterized.
 */
networkCapabilityCall
    : networkCapabilityReference
      LPAREN
      networkCapabilityArgumentList?
      RPAREN
    ;


/* ============================================================================
 * CAPABILITY ARGUMENT LIST
 * ========================================================================== */

networkCapabilityArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * CAPABILITY PRESENCE
 * ========================================================================== */

/*
 * Explicit presence testing allows a networking contract to distinguish:
 *
 *     capability exists
 *
 * from:
 *
 *     capability has a particular value.
 *
 * The semantic layer determines whether `is` is valid for a particular
 * capability domain.
 */
networkCapabilityPresence
    : networkCapabilityReference
      IS
      networkCapabilityPresenceValue
    ;


/* ============================================================================
 * PRESENCE VALUES
 * ========================================================================== */

networkCapabilityPresenceValue
    : TRUE
    | FALSE
    ;


/* ============================================================================
 * CAPABILITY GROUP
 * ========================================================================== */

/*
 * Groups provide a structural boundary for multiple capability predicates.
 *
 * There is no fixed number of predicates.
 */
networkCapabilityGroup
    : LPAREN
      networkCapabilityPredicate
      RPAREN
    ;


/* ============================================================================
 * CAPABILITY METADATA
 * ========================================================================== */

/*
 * Metadata is deliberately generic.
 *
 * It does not create a second metadata language.
 */
networkCapabilityMetadata
    : PROPERTY
      identifier
      networkCapabilityMetadataValue?
      SEMI
    ;


/* ============================================================================
 * METADATA VALUE
 * ========================================================================== */

networkCapabilityMetadataValue
    : COLON
      expression
    | ASSIGN
      expression
    ;


/* ============================================================================
 * CAPABILITY SET
 * ========================================================================== */

/*
 * A capability set is useful where an endpoint/channel/service exposes a
 * collection of capabilities.
 *
 * Example:
 *
 *     capabilities {
 *         networking::reliable_delivery;
 *         networking::ordered_delivery;
 *         networking::multicast;
 *     }
 *
 * The set has no fixed cardinality.
 */
networkCapabilitySet
    : CAPABILITIES
      LBRACE
      networkCapabilitySetMember*
      RBRACE
    ;


/* ============================================================================
 * CAPABILITY SET MEMBER
 * ========================================================================== */

networkCapabilitySetMember
    : networkCapabilityReference
      SEMI
    | networkCapabilityProvision
    | networkCapabilityRequirement
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * CAPABILITY NEGOTIATION INTENT
 * ========================================================================== */

/*
 * Negotiation is expressed as source intent only.
 *
 * Runtime negotiation is explicitly outside this grammar.
 *
 * Example:
 *
 *     negotiate {
 *         networking::reliable_delivery
 *         or
 *         networking::best_effort;
 *     }
 *
 * The semantic layer determines compatibility and selection policy.
 */
networkCapabilityNegotiation
    : NEGOTIATE
      LBRACE
      networkCapabilityNegotiationMember*
      RBRACE
    ;


/* ============================================================================
 * NEGOTIATION MEMBER
 * ========================================================================== */

networkCapabilityNegotiationMember
    : networkCapabilityPredicate
      SEMI
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * CAPABILITY REQUIREMENT GROUP
 * ========================================================================== */

/*
 * Groups mandatory requirements without imposing a fixed number.
 */
networkCapabilityRequirementGroup
    : REQUIRES
      LBRACE
      networkCapabilityRequirementMember*
      RBRACE
    ;


/* ============================================================================
 * REQUIREMENT GROUP MEMBER
 * ========================================================================== */

networkCapabilityRequirementMember
    : networkCapabilityPredicate
      SEMI
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * CAPABILITY PROVISION GROUP
 * ========================================================================== */

/*
 * Groups provided capabilities.
 */
networkCapabilityProvisionGroup
    : PROVIDES
      LBRACE
      networkCapabilityProvisionMember*
      RBRACE
    ;


/* ============================================================================
 * PROVISION GROUP MEMBER
 * ========================================================================== */

networkCapabilityProvisionMember
    : networkCapabilityReference
      SEMI
    | networkCapabilityPredicate
      SEMI
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * CAPABILITY CONTRACT
 * ========================================================================== */

/*
 * This is the principal integration boundary for endpoints, channels and
 * services.
 *
 * Example:
 *
 *     capability_contract {
 *         requires networking::reliable_delivery;
 *         provides networking::ordered_delivery;
 *         constraint networking::latency < maximum_latency;
 *         preference networking::low_latency;
 *     }
 */
networkCapabilityContract
    : CAPABILITY_CONTRACT
      LBRACE
      networkCapabilityContractMember*
      RBRACE
    ;


/* ============================================================================
 * CONTRACT MEMBER
 * ========================================================================== */

networkCapabilityContractMember
    : networkCapabilityRequirement
    | networkCapabilityProvision
    | networkCapabilityConstraint
    | networkCapabilityPreference
    | networkCapabilityNegotiation
    | networkCapabilityMetadata
    ;


/* ============================================================================
 * ENDPOINT INTEGRATION
 * ========================================================================== */

/*
 * Endpoint grammars may consume this rule:
 *
 *     endpointCapabilityContract
 *
 * without importing networking implementation details.
 */
endpointCapabilityContract
    : networkCapabilityContract
    ;


/* ============================================================================
 * CHANNEL INTEGRATION
 * ========================================================================== */

/*
 * Channel grammars may consume this rule to express communication
 * capabilities without defining capabilities themselves.
 */
channelCapabilityContract
    : networkCapabilityContract
    ;


/* ============================================================================
 * SERVICE INTEGRATION
 * ========================================================================== */

/*
 * Service grammars may consume this rule for service-wide communication
 * capabilities.
 */
serviceCapabilityContract
    : networkCapabilityContract
    ;


/* ============================================================================
 * PROTOCOL INTEGRATION
 * ========================================================================== */

/*
 * Protocols may consume the capability predicate without acquiring ownership
 * of capability declarations.
 */
protocolCapabilityContract
    : networkCapabilityContract
    ;