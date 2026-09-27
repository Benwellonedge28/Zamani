/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/addresses.g4
 *
 * Grammar:
 *     Addresses
 *
 * Status:
 *     Production parser grammar contract
 *
 * Purpose:
 *     Canonical parser-level grammar for logical and address-descriptor
 *     declarations used by Zamani networking semantics.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust:
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust
 *     - No actions
 *     - No semantic predicates
 *     - No filesystem access
 *     - No network access
 *     - No environment access
 *     - No hardware access
 *     - No runtime callbacks
 *     - No randomness
 *     - No unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     grammar/networking/networking.g4
 *          |
 *          v
 *     Addresses
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> portability analysis
 *          +--> networking-address validation
 *          |
 *          v
 *     canonical networking semantic model
 *          |
 *          +--> networking IR
 *          +--> distributed semantics
 *          +--> classical semantics
 *          +--> hardware intent
 *          +--> runtime realization
 *          |
 *          v
 *     discovery / routing / placement / scheduling / deployment
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * This grammar describes SOURCE-LEVEL ADDRESS INTENT.
 *
 * It does NOT determine physical address allocation or network realization.
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * An address in Zamani is a logical source-level communication locator or
 * address descriptor.
 *
 * It is NOT intrinsically:
 *
 *     - an IPv4 address;
 *     - an IPv6 address;
 *     - a MAC address;
 *     - a socket;
 *     - a port;
 *     - a network interface;
 *     - a NIC;
 *     - a router;
 *     - a switch;
 *     - a physical node;
 *     - a machine;
 *     - a process;
 *     - a thread;
 *     - a GPU;
 *     - an FPGA;
 *     - a QPU;
 *     - a physical qubit;
 *     - a cloud instance;
 *     - a provider-specific resource.
 *
 * A semantic subsystem may interpret an address descriptor as one of those
 * things when the program explicitly requests such semantics and the target
 * supports them.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - address declarations;
 *     - address declaration names;
 *     - address declaration bodies;
 *     - address members;
 *     - address properties;
 *     - address nested configuration;
 *     - address references;
 *     - address aliases expressed through the address declaration form;
 *     - address-descriptor structure;
 *     - stable parser entry points for address constructs.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - Unicode identifier rules;
 *     - keywords;
 *     - literals;
 *     - operators;
 *     - general expressions;
 *     - types;
 *     - modules;
 *     - endpoints;
 *     - protocols;
 *     - channels;
 *     - services;
 *     - sockets;
 *     - routing;
 *     - service discovery;
 *     - distributed placement;
 *     - topology;
 *     - network interfaces;
 *     - DNS;
 *     - IP allocation;
 *     - MAC allocation;
 *     - transport implementations;
 *     - serialization;
 *     - cryptography;
 *     - authentication;
 *     - authorization;
 *     - hardware discovery;
 *     - resource allocation;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Address syntax MUST remain independent of the physical scale of execution.
 *
 * The grammar contains NO universal limits for:
 *
 *     MAX_ADDRESSES
 *     MAX_ADDRESS_LENGTH
 *     MAX_ADDRESS_COMPONENTS
 *     MAX_ADDRESS_DEPTH
 *     MAX_ADDRESS_FAMILY
 *     MAX_ENDPOINTS
 *     MAX_NETWORKS
 *     MAX_INTERFACES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CONNECTIONS
 *     MAX_NETWORK_SIZE
 *     MAX_TOPOLOGY_SIZE
 *     MAX_ADDRESS_WIDTH
 *     MAX_HOSTS
 *     MAX_PORTS
 *
 * No such constants are represented by parser rules.
 *
 * Repeated address members use ANTLR repetition operators.
 *
 * Practical limits may arise from:
 *
 *     - source size;
 *     - compiler memory;
 *     - parser implementation;
 *     - semantic-analysis resources;
 *     - runtime resources;
 *     - target capabilities;
 *     - deployment policies.
 *
 * Those are implementation/resource limits, not language-level address
 * limitations.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define universal physical address constants such as:
 *
 *     MAX_IPV4_BITS
 *     MAX_IPV6_BITS
 *     MAX_MAC_BITS
 *     MAX_ADDRESS_BYTES
 *     MAX_NETWORKS
 *     MAX_NODES
 *     MAX_INTERFACES
 *     MAX_PORTS
 *     MAX_ENDPOINTS
 *
 * It MUST NOT define physical identifiers such as:
 *
 *     interface_0
 *     node_0
 *     router_0
 *     switch_0
 *     nic_0
 *     device_0
 *
 * as language-level address semantics.
 *
 * A user may still use such spellings as ordinary identifiers or source data.
 * This grammar assigns them no implicit physical meaning.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Address declarations may contain properties expressing:
 *
 *     - logical identity;
 *     - address family;
 *     - scheme;
 *     - locality;
 *     - scope;
 *     - capability requirements;
 *     - resource requirements;
 *     - constraints;
 *     - preferences;
 *     - security requirements;
 *     - metadata;
 *     - application-specific address semantics.
 *
 * The parser does NOT determine which category a property belongs to.
 *
 * Semantic analysis determines:
 *
 *     - property meaning;
 *     - expected type;
 *     - validity;
 *     - capability implications;
 *     - resource implications;
 *     - security implications;
 *     - portability;
 *     - target realization.
 *
 * ============================================================================
 * OPEN-WORLD ADDRESS MODEL
 * ============================================================================
 *
 * The grammar intentionally does NOT enumerate address families.
 *
 * It does NOT contain closed alternatives such as:
 *
 *     IPV4
 *     IPV6
 *     MAC
 *     BLUETOOTH
 *     INFINIBAND
 *     RDMA
 *     QUIC
 *     DNS
 *     HTTP
 *     UNIX_SOCKET
 *
 * Such concepts remain semantic names.
 *
 * Therefore future address families can be introduced without modifying this
 * grammar merely because a new networking technology exists.
 *
 * Examples of source-level semantic names that may be represented:
 *
 *     network::ipv4
 *     network::ipv6
 *     network::mac
 *     network::dns
 *     network::logical
 *     network::service
 *     network::fabric
 *     vendor::future_address
 *
 * The parser does not assign those names physical meaning.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This grammar defines NO lexer rules.
 *
 * It therefore does not define:
 *
 *     IDENTIFIER
 *     STRING
 *     INTEGER
 *     FLOAT
 *     COLON
 *     SEMI
 *     LBRACE
 *     RBRACE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     DOUBLE_COLON
 *
 * Those remain owned by the canonical lexical architecture.
 *
 * ============================================================================
 * CONTEXTUAL ADDRESS MARKER
 * ============================================================================
 *
 * The current lexical architecture does not require a globally reserved
 * ADDRESS keyword.
 *
 * Therefore this grammar deliberately uses a contextual marker:
 *
 *     addressMarker
 *         : identifier
 *         ;
 *
 * The semantic layer validates that the marker denotes the address
 * declaration construct in an address-declaration context.
 *
 * This follows the same architectural pattern already used by
 * networking/endpoints.g4 for its contextual `endpoint` marker.
 *
 * This avoids adding another global keyword solely for networking.
 *
 * If a future language version reserves `address` lexically, this grammar's
 * semantic model does not need to change; only the lexical compatibility
 * layer changes.
 *
 * ============================================================================
 * DECLARATION MODEL
 * ============================================================================
 *
 * The canonical declaration forms are:
 *
 *     address name;
 *
 *     address name = expression;
 *
 *     address name {
 *         property: expression;
 *     }
 *
 *     address name : address::family = expression;
 *
 * A body may optionally be followed by a semicolon:
 *
 *     address name {
 *         family: network::ipv6;
 *     };
 *
 * The grammar therefore supports both block-oriented and semicolon-oriented
 * declaration styles without creating separate declaration concepts.
 *
 * ============================================================================
 * ADDRESS DECLARATION SEMANTICS
 * ============================================================================
 *
 * An address declaration introduces a named source-level address descriptor.
 *
 * It does not allocate an address.
 *
 * It does not contact a network.
 *
 * It does not perform discovery.
 *
 * It does not reserve an interface.
 *
 * It does not bind a socket.
 *
 * It does not select a machine.
 *
 * It does not select a node.
 *
 * It does not select a physical device.
 *
 * ============================================================================
 * ADDRESS KIND
 * ============================================================================
 *
 * An optional type-like clause may describe the logical address family:
 *
 *     address server : network::logical;
 *
 *     address host : network::ipv4;
 *
 *     address fabric : network::fabric;
 *
 * The value after `:` is a canonical qualified name.
 *
 * This is a semantic classification, not a parser-enforced finite enumeration.
 *
 * ============================================================================
 * ADDRESS INITIALIZATION
 * ============================================================================
 *
 * An address may be initialized from the canonical expression grammar:
 *
 *     address server = "service.example";
 *
 *     address service = logical::service("compute");
 *
 *     address local = address_source;
 *
 *     address remote = network::resolve("service");
 *
 * The parser does not resolve or evaluate these expressions.
 *
 * Evaluation and resolution remain downstream.
 *
 * ============================================================================
 * ADDRESS BODY
 * ============================================================================
 *
 * An address body contains zero or more address members.
 *
 * There is deliberately no fixed member count.
 *
 * Members are structural and extensible.
 *
 * Example:
 *
 *     address compute {
 *         family: network::logical;
 *         value: "compute";
 *         scope: application;
 *         locality: region;
 *         requires: capability("network.reliable");
 *         prefers: locality;
 *     }
 *
 * ============================================================================
 * GENERIC PROPERTY MODEL
 * ============================================================================
 *
 * Address properties intentionally use an open key space.
 *
 * The grammar does NOT enumerate:
 *
 *     family
 *     scheme
 *     value
 *     host
 *     port
 *     scope
 *     locality
 *     interface
 *     protocol
 *     security
 *     capability
 *     requires
 *     prefers
 *     metadata
 *
 * as dedicated parser keywords.
 *
 * They are ordinary identifiers in property position.
 *
 * This avoids:
 *
 *     - keyword explosion;
 *     - future compatibility breaks;
 *     - vendor coupling;
 *     - address-family coupling;
 *     - protocol coupling;
 *     - physical-topology coupling.
 *
 * Semantic analysis may recognize well-known property names.
 *
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * Canonical property:
 *
 *     key: expression;
 *
 * Examples:
 *
 *     family: network::ipv6;
 *     value: "example";
 *     scope: application;
 *     locality: region;
 *     requires: capability("network.reliable");
 *
 * The property value is always parsed by the common expression grammar.
 *
 * ============================================================================
 * NESTED ADDRESS CONFIGURATION
 * ============================================================================
 *
 * Nested blocks provide structured extension without introducing additional
 * networking keywords.
 *
 * Example:
 *
 *     address service {
 *         discovery {
 *             scheme: logical;
 *             name: "compute";
 *         }
 *
 *         security {
 *             requires: security::authentication;
 *         }
 *     }
 *
 * Nested block interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * ADDRESS REFERENCES
 * ============================================================================
 *
 * A reusable address reference is a canonical qualified name:
 *
 *     server
 *     network::server
 *     services::compute::address
 *
 * This grammar performs NO lookup.
 *
 * Name resolution belongs to the semantic phase.
 *
 * ============================================================================
 * ADDRESS REFERENCE LIST
 * ============================================================================
 *
 * Lists are unbounded at the grammar level:
 *
 *     addressReferences
 *         : addressReference (COMMA addressReference)* COMMA?
 *         ;
 *
 * No fixed number of addresses is encoded.
 *
 * ============================================================================
 * INLINE ADDRESS DESCRIPTORS
 * ============================================================================
 *
 * The grammar also provides a stable parser-level form for address
 * descriptors that need to appear inside another networking construct:
 *
 *     addressDescriptor
 *
 * The descriptor consists of a semantic address family/name and an optional
 * expression value.
 *
 * Examples:
 *
 *     network::logical
 *
 *     network::logical("service")
 *
 *     network::ipv4("192.0.2.1")
 *
 *     network::ipv6("2001:db8::1")
 *
 * These examples are syntactic forms only.
 *
 * Whether a string is actually a valid IPv4/IPv6 representation is a semantic
 * validation responsibility.
 *
 * This is important because lexical grammar must not duplicate or hard-code
 * every network address format.
 *
 * ============================================================================
 * WHY IP/MAC LITERALS ARE NOT HARD-CODED HERE
 * ============================================================================
 *
 * A tempting design would define rules such as:
 *
 *     ipv4Address
 *         : INTEGER DOT INTEGER DOT INTEGER DOT INTEGER
 *         ;
 *
 * or:
 *
 *     ipv6Address
 *         : ...
 *
 * This is deliberately NOT the canonical Zamani design.
 *
 * Such rules would:
 *
 *     - make one set of address families privileged;
 *     - require lexer/parser changes for future address families;
 *     - mix lexical representation with semantic validity;
 *     - encourage fixed-width assumptions;
 *     - make vendor/future address models harder to support;
 *     - create multiple address authorities;
 *     - reduce POCO-REAF extensibility.
 *
 * Address values are therefore source expressions.
 *
 * Semantic networking validation can provide precise validators for:
 *
 *     IPv4
 *     IPv6
 *     MAC
 *     URI
 *     DNS
 *     service identifiers
 *     fabric identifiers
 *     future address families
 *
 * without changing this grammar.
 *
 * ============================================================================
 * PORT SEPARATION
 * ============================================================================
 *
 * A port is NOT an address.
 *
 * This grammar does not create a dedicated universal port grammar.
 *
 * An address may carry a port-like semantic property:
 *
 *     address service {
 *         host: "example";
 *         port: 443;
 *     }
 *
 * or:
 *
 *     address service = network::endpoint("example", 443);
 *
 * The semantic networking model determines whether a property is actually a
 * transport port, service identifier, logical channel, or another concept.
 *
 * This prevents address syntax from becoming coupled to sockets/transports.
 *
 * ============================================================================
 * HOST / PATH / URI SEPARATION
 * ============================================================================
 *
 * A URI, hostname, filesystem path, module path, qualified name, and logical
 * network address are distinct semantic concepts even when their source
 * representations look similar.
 *
 * This grammar therefore does not reinterpret:
 *
 *     ./foo
 *     /foo/bar
 *     module::name
 *     https://example
 *
 * automatically as one another.
 *
 * Explicit address declarations or address descriptors establish the semantic
 * context.
 *
 * ============================================================================
 * ENDPOINT INTEGRATION
 * ============================================================================
 *
 * Endpoints are owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * An endpoint may refer to an address using:
 *
 *     endpointReference
 *
 * or through its generic property expression:
 *
 *     address: server;
 *
 * Address declarations therefore remain independently reusable.
 *
 * This grammar MUST NOT redefine endpoint declarations.
 *
 * ============================================================================
 * PROTOCOL INTEGRATION
 * ============================================================================
 *
 * Protocols are owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * This grammar does not define:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transport names
 *
 * A protocol may reference an address semantically.
 *
 * Protocol realization remains downstream.
 *
 * ============================================================================
 * CHANNEL INTEGRATION
 * ============================================================================
 *
 * Networking channels are owned by:
 *
 *     grammar/networking/channels.g4
 *
 * Address declarations may be used as channel source/destination values.
 *
 * This grammar does not define channel behavior.
 *
 * ============================================================================
 * SERVICE INTEGRATION
 * ============================================================================
 *
 * Services are owned by:
 *
 *     grammar/networking/services.g4
 *
 * A service may expose one or more logical addresses.
 *
 * Address declarations remain independent of service implementation.
 *
 * ============================================================================
 * SERVICE DISCOVERY INTEGRATION
 * ============================================================================
 *
 * Address discovery is NOT performed by this grammar.
 *
 * A source program may express:
 *
 *     address compute {
 *         discovery: network::service;
 *         name: "compute";
 *     }
 *
 * The semantic/runtime discovery subsystem decides how the logical address is
 * resolved.
 *
 * This may involve:
 *
 *     - local discovery;
 *     - distributed discovery;
 *     - DNS;
 *     - service registries;
 *     - capability registries;
 *     - runtime-provided discovery;
 *     - future discovery systems.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed computing owns:
 *
 *     - node membership;
 *     - placement;
 *     - replication;
 *     - distributed scheduling;
 *     - cluster semantics;
 *     - distributed recovery.
 *
 * An address may be used by those systems, but it does not become a node
 * declaration merely because it is used for communication.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware owns physical network realization.
 *
 * This grammar MUST NOT define:
 *
 *     NIC
 *     MAC allocator
 *     switch
 *     router
 *     physical link
 *     bus
 *     FPGA pin
 *     ASIC pin
 *     physical topology
 *     physical interface index
 *
 * Hardware-specific meaning is introduced only downstream through explicit
 * semantic capabilities and target realization.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Networking may participate in quantum/classical hybrid computation.
 *
 * This grammar does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     LogicalQubitId
 *     quantum gate sets
 *     quantum topology
 *     calibration
 *     pulse definitions
 *     QEC
 *     ZQN
 *
 * If an address is used to coordinate quantum computation, the networking
 * semantics integrate with the existing canonical:
 *
 *     quantum::ir
 *
 * boundary downstream.
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Security requirements may be represented as ordinary expressions:
 *
 *     security: security::confidentiality;
 *     requires: security::authentication;
 *     trust: security::trusted;
 *
 * This grammar does NOT define:
 *
 *     - cryptographic algorithms;
 *     - key management;
 *     - certificate validation;
 *     - identity providers;
 *     - authorization engines;
 *     - trust evaluation.
 *
 * Those belong to the security and semantic layers.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Address properties may express requirements:
 *
 *     requires: capability("network.reliable");
 *
 *     requires: capability("network.low_latency");
 *
 *     requires: capability("network.ipv6");
 *
 * The parser only captures the expression.
 *
 * Semantic/resource analysis determines whether the requirement is valid and
 * whether a target can satisfy it.
 *
 * No machine-size assumption is encoded.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST preserve source spans for:
 *
 *     - address declaration;
 *     - address marker;
 *     - address name;
 *     - address kind;
 *     - initializer;
 *     - body;
 *     - every property key;
 *     - every property value;
 *     - every nested block;
 *     - every address reference.
 *
 * This grammar does not depend on the Rust AST implementation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should provide enough structure for a domain-neutral AST to
 * represent:
 *
 *     AddressDeclaration
 *         marker
 *         name
 *         kind?
 *         initializer?
 *         body?
 *         span
 *
 *     AddressProperty
 *         key
 *         value
 *         span
 *
 *     AddressNestedBlock
 *         key
 *         members
 *         span
 *
 *     AddressReference
 *         qualifiedName
 *         span
 *
 *     AddressDescriptor
 *         family/name
 *         arguments/value?
 *         span
 *
 * The AST MUST NOT directly represent:
 *
 *     PhysicalIpAddress
 *     PhysicalMacAddress
 *     PhysicalInterface
 *     RouterAllocation
 *     NetworkRoute
 *     DevicePlacement
 *
 * unless those are introduced as separate semantic-domain nodes downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - validating the contextual address marker;
 *     - validating address declaration names;
 *     - detecting duplicate declarations;
 *     - resolving address references;
 *     - resolving address families;
 *     - validating address values;
 *     - validating property schemas;
 *     - validating address-family-specific semantics;
 *     - validating capability requirements;
 *     - validating resource requirements;
 *     - validating security requirements;
 *     - checking portability;
 *     - checking address compatibility with endpoints;
 *     - checking compatibility with protocols;
 *     - checking compatibility with channels;
 *     - resolving discovery requirements;
 *     - determining whether physical realization is required.
 *
 * The parser MUST NOT perform any of these operations.
 *
 * ============================================================================
 * ADDRESS-FAMILY SEMANTICS
 * ============================================================================
 *
 * Address-family names remain open-world.
 *
 * For example:
 *
 *     network::ipv4
 *     network::ipv6
 *     network::mac
 *     network::dns
 *     network::logical
 *     custom::address_family
 *
 * are all syntactically equivalent qualified names.
 *
 * Semantic analysis may attach specialized validation to recognized families.
 *
 * An unrecognized family may be:
 *
 *     - rejected;
 *     - deferred;
 *     - handled by a registered dialect;
 *     - treated as an opaque logical family;
 *
 * according to the language's versioned semantic policy.
 *
 * The parser remains unchanged.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define an IR.
 *
 * Address declarations lower into the canonical semantic networking model.
 *
 * That model may lower into:
 *
 *     - networking IR;
 *     - distributed IR;
 *     - classical IR;
 *     - deployment descriptors;
 *     - hardware/network intent;
 *     - runtime address objects.
 *
 * If networking participates in a quantum computation, the established:
 *
 *     quantum::ir
 *
 * boundary remains canonical for quantum semantics.
 *
 * No address grammar construct may introduce a second quantum IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use address semantics for:
 *
 *     - capability negotiation;
 *     - target selection;
 *     - address-family lowering;
 *     - discovery planning;
 *     - routing planning;
 *     - communication scheduling;
 *     - deployment generation;
 *     - interoperability generation.
 *
 * These operations occur AFTER parsing and semantic analysis.
 *
 * The grammar itself remains target-independent.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may eventually:
 *
 *     - resolve a logical address;
 *     - discover a service;
 *     - bind an endpoint;
 *     - establish communication;
 *     - monitor availability;
 *     - reconnect;
 *     - migrate communication;
 *     - select a realization.
 *
 * None of these operations occur during parsing.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer vocabulary;
 *     parser grammar;
 *
 * this grammar must produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     - current network state;
 *     - DNS;
 *     - filesystem state;
 *     - hardware;
 *     - deployment;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - available CPUs;
 *     - available GPUs;
 *     - available QPUs;
 *     - network topology.
 *
 * ============================================================================
 * ERROR BEHAVIOR
 * ============================================================================
 *
 * Malformed address syntax must remain parser-visible.
 *
 * The parser MUST NOT silently reinterpret malformed input as another address
 * category.
 *
 * Semantic errors such as:
 *
 *     invalid IPv4 value;
 *     invalid IPv6 value;
 *     unsupported address family;
 *     unavailable capability;
 *     unresolved logical address;
 *
 * are NOT syntax errors and must be reported downstream by the appropriate
 * semantic subsystem.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar is intentionally additive and open-world.
 *
 * Existing networking syntax remains unaffected because:
 *
 *     - no existing lexical token is renamed;
 *     - no existing lexer rule is duplicated;
 *     - no protocol syntax is redefined;
 *     - no endpoint syntax is redefined;
 *     - no channel syntax is redefined;
 *     - address names remain ordinary identifiers;
 *     - address values use canonical expressions.
 *
 * Future address families do not require grammar changes merely because their
 * names are new.
 *
 * Breaking changes require the repository's normal:
 *
 *     compatibility/
 *
 * versioning and migration process.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The validation/test architecture must cover at minimum:
 *
 * POSITIVE:
 *
 *     address server;
 *
 *     address server = "service.example";
 *
 *     address server : network::logical;
 *
 *     address server : network::ipv4 = "192.0.2.1";
 *
 *     address server {
 *         family: network::ipv6;
 *         value: "2001:db8::1";
 *     }
 *
 *     address compute {
 *         requires: capability("network.reliable");
 *         prefers: locality;
 *     }
 *
 *     address service {
 *         discovery {
 *             scheme: network::logical;
 *             name: "compute";
 *         }
 *     }
 *
 *     address a;
 *     address b = a;
 *
 *     address a = network::logical("compute");
 *
 *     address a : vendor::future_address {
 *         value: "opaque-address";
 *     }
 *
 * REFERENCES:
 *
 *     endpoint worker {
 *         address: compute;
 *     }
 *
 *     endpoint worker {
 *         address: network::logical("compute");
 *     }
 *
 * NEGATIVE:
 *
 *     address;
 *
 *     address = "value";
 *
 *     address server : ;
 *
 *     address server = ;
 *
 *     address server {
 *         property:
 *     }
 *
 *     address server {
 *         property
 *     }
 *
 * Boundary/scalability:
 *
 *     - arbitrarily many address declarations;
 *     - arbitrarily many properties;
 *     - arbitrarily deep logical qualified names subject to implementation
 *       resources;
 *     - arbitrarily many nested configuration blocks;
 *     - arbitrarily many address references;
 *     - arbitrarily large source-level address values supported by the
 *       canonical literal/expression implementation.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO MAX_ADDRESSES
 *     NO MAX_ADDRESS_LENGTH
 *     NO MAX_IPV4
 *     NO MAX_IPV6
 *     NO MAX_MAC
 *     NO MAX_HOSTS
 *     NO MAX_NETWORKS
 *     NO MAX_NODES
 *     NO MAX_INTERFACES
 *     NO MAX_DEVICES
 *     NO MAX_PORTS
 *     NO MAX_TOPOLOGY
 *
 * It contains no:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     register width
 *     tensor rank
 *
 * as language-level limits.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] ownership is defined;
 *     [x] non-ownership is defined;
 *     [x] lexical dependencies are explicit;
 *     [x] stable parser entry point exists;
 *     [x] address declaration syntax exists;
 *     [x] address references exist;
 *     [x] address descriptors exist;
 *     [x] generic properties exist;
 *     [x] nested configuration exists;
 *     [x] initialization exists;
 *     [x] open-world address families exist;
 *     [x] no fixed physical address syntax is required;
 *     [x] no machine limits exist;
 *     [x] AST contract is defined;
 *     [x] semantic contract is defined;
 *     [x] IR contract is defined;
 *     [x] compiler/runtime boundaries are defined;
 *     [x] deterministic parsing is defined;
 *     [x] compatibility policy is defined;
 *     [x] testing contract is defined;
 *     [x] hard-coding audit is defined.
 *
 * Repository integration remains a composition concern:
 *
 *     networking.g4
 *         imports Addresses
 *
 * and:
 *
 *     networkingConstruct
 *         includes addressConstruct
 *
 * The canonical root parser then consumes networking.g4 through its existing
 * networking integration boundary.
 *
 * ============================================================================
 */

parser grammar Addresses;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable networking composition boundary.
 *
 * `networking.g4` should consume `addressConstruct`.
 */
addressConstruct
    : addressDeclaration
    | addressReference
    | addressDescriptor
    ;


/*
 * ============================================================================
 * ADDRESS DECLARATION
 * ============================================================================
 *
 * Supported forms:
 *
 *     address name;
 *
 *     address name = expression;
 *
 *     address name : family;
 *
 *     address name : family = expression;
 *
 *     address name {
 *         ...
 *     }
 *
 *     address name : family {
 *         ...
 *     }
 *
 *     address name : family = expression {
 *         ...
 *     }
 *
 * The semantic layer determines whether a particular combination is valid.
 *
 * The parser preserves the structural distinction without embedding semantic
 * policy into the grammar.
 */
addressDeclaration
    : attribute*
      visibility?
      addressMarker
      identifier
      addressKindClause?
      addressInitializer?
      addressBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL ADDRESS MARKER
 * ============================================================================
 *
 * `address` remains a contextual identifier in the current lexical architecture.
 *
 * Semantic analysis validates its declaration meaning.
 */
addressMarker
    : identifier
    ;


/*
 * ============================================================================
 * ADDRESS KIND / FAMILY
 * ============================================================================
 *
 * Examples:
 *
 *     : network::logical
 *     : network::ipv4
 *     : network::ipv6
 *     : vendor::future_address
 *
 * The grammar does not enumerate families.
 */
addressKindClause
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * ADDRESS INITIALIZER
 * ============================================================================
 *
 * Values are canonical Zamani expressions.
 *
 * No address-family-specific literal syntax is imposed here.
 */
addressInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * ADDRESS BODY
 * ============================================================================
 */
addressBody
    : LBRACE
      addressMember*
      RBRACE
    ;


/*
 * ============================================================================
 * ADDRESS MEMBER
 * ============================================================================
 */
addressMember
    : addressProperty
    | addressNestedBlock
    ;


/*
 * ============================================================================
 * ADDRESS PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     key: expression;
 *
 * Property names are ordinary identifiers.
 */
addressProperty
    : attribute*
      identifier
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * ADDRESS NESTED BLOCK
 * ============================================================================
 *
 * Canonical form:
 *
 *     key {
 *         ...
 *     }
 *
 * The semantic layer determines what the nested section means.
 */
addressNestedBlock
    : attribute*
      identifier
      LBRACE
      addressMember*
      RBRACE
      SEMI?
    ;


/*
 * ============================================================================
 * ADDRESS REFERENCE
 * ============================================================================
 *
 * A reference is a canonical qualified name.
 *
 * No lookup is performed by the parser.
 */
addressReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ADDRESS REFERENCE LIST
 * ============================================================================
 */
addressReferences
    : addressReference
      (COMMA addressReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL ADDRESS REFERENCE LIST
 * ============================================================================
 */
optionalAddressReferences
    : addressReferences?
    ;


/*
 * ============================================================================
 * ADDRESS DESCRIPTOR
 * ============================================================================
 *
 * A descriptor provides an explicit address-family/name and an optional value.
 *
 * Examples:
 *
 *     network::logical
 *
 *     network::logical("compute")
 *
 *     network::ipv4("192.0.2.1")
 *
 *     vendor::future_address("opaque")
 *
 * The family itself remains an open qualified name.
 */
addressDescriptor
    : qualifiedName
      addressDescriptorArguments?
    ;


/*
 * ============================================================================
 * ADDRESS DESCRIPTOR ARGUMENTS
 * ============================================================================
 *
 * Values are ordinary expressions.
 *
 * This allows future address families to define their own semantic argument
 * schemas without changing this grammar.
 */
addressDescriptorArguments
    : LPAREN
      addressArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * ADDRESS ARGUMENT LIST
 * ============================================================================
 */
addressArgumentList
    : addressArgument
      (COMMA addressArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * ADDRESS ARGUMENT
 * ============================================================================
 *
 * Named and positional arguments are both supported.
 *
 * Examples:
 *
 *     network::logical("compute")
 *
 *     network::endpoint(
 *         host = "example",
 *         port = 443
 *     )
 *
 * The semantic layer determines whether named arguments are legal for a
 * particular address family.
 */
addressArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * ADDRESS VALUE
 * ============================================================================
 *
 * Reusable expression boundary for higher-level networking grammars.
 *
 * This intentionally delegates to the canonical expression grammar.
 */
addressValue
    : expression
    ;


/*
 * ============================================================================
 * ADDRESS FAMILY REFERENCE
 * ============================================================================
 *
 * Explicitly named semantic family.
 *
 * This is equivalent to a qualified name but gives higher-level networking
 * grammars a stable semantic rule name.
 */
addressFamilyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * ADDRESS FAMILY REFERENCE LIST
 * ============================================================================
 */
addressFamilyReferences
    : addressFamilyReference
      (COMMA addressFamilyReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * ADDRESS PROPERTY KEY
 * ============================================================================
 *
 * Property keys remain ordinary identifiers.
 */
addressPropertyKey
    : identifier
    ;


/*
 * ============================================================================
 * ADDRESS PROPERTY VALUE
 * ============================================================================
 *
 * Canonical expression boundary.
 */
addressPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * ADDRESS PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * This reusable rule is provided for higher-level networking grammars that
 * need to embed address-like property syntax without duplicating the grammar.
 *
 * Example:
 *
 *     address: server;
 *
 *     destination: network::logical("compute");
 */
addressPropertyAssignment
    : addressPropertyKey
      COLON
      addressPropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * ADDRESS QUALIFIER
 * ============================================================================
 *
 * Optional semantic qualifier represented by a qualified name.
 *
 * This is intentionally not tied to:
 *
 *     IPv4
 *     IPv6
 *     MAC
 *     DNS
 *     URI
 *     transport
 *     provider
 *     hardware.
 */
addressQualifier
    : qualifiedName
    ;


/*
 * ============================================================================
 * ADDRESS QUALIFIER LIST
 * ============================================================================
 */
addressQualifiers
    : addressQualifier
      (COMMA addressQualifier)*
      COMMA?
    ;


/*
 * ============================================================================
 * ADDRESS DECLARATION LIST
 * ============================================================================
 *
 * No fixed declaration count is imposed.
 *
 * This rule is useful for networking composition and validation grammars.
 */
addressDeclarations
    : addressDeclaration*
    ;


/*
 * ============================================================================
 * OPTIONAL ADDRESS DECLARATION LIST
 * ============================================================================
 */
optionalAddressDeclarations
    : addressDeclarations
    ;