/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/protocols.g4
 *
 * Grammar:
 *     Protocols
 *
 * Purpose:
 *     Canonical production parser grammar for protocol declarations,
 *     protocol references, protocol composition, protocol requirements,
 *     protocol capabilities, and protocol preferences.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, hardware access, runtime calls,
 *     or unsafe code.
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
 *     Protocols parser
 *          |
 *          v
 *     Networking frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> requirement analysis
 *          +--> constraint analysis
 *          +--> security analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical semantic networking model
 *          |
 *          +--> distributed execution
 *          +--> hardware/network realization
 *          +--> resource selection
 *          +--> compilation
 *          +--> runtime
 *
 * The grammar does NOT directly construct or select a runtime protocol.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - protocol declaration syntax;
 *     - protocol names;
 *     - protocol references;
 *     - protocol composition syntax;
 *     - protocol version requirements as source-level expressions;
 *     - protocol capabilities as source-level declarations;
 *     - protocol requirements;
 *     - protocol constraints;
 *     - protocol preferences;
 *     - protocol metadata;
 *     - protocol inheritance/extension intent;
 *     - protocol bindings to logical networking constructs;
 *     - protocol options represented as expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier spelling;
 *     - qualified-name spelling;
 *     - general expressions;
 *     - message schemas;
 *     - serialization;
 *     - endpoint declarations;
 *     - channel declarations;
 *     - service declarations;
 *     - physical topology;
 *     - routing;
 *     - packet scheduling;
 *     - sockets;
 *     - ports;
 *     - IP addresses;
 *     - MAC addresses;
 *     - network interfaces;
 *     - device discovery;
 *     - hardware discovery;
 *     - transport implementation;
 *     - cryptographic algorithms;
 *     - authentication;
 *     - authorization;
 *     - trust implementation;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A protocol in Zamani is a semantic communication contract.
 *
 * It does not inherently mean:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MPI
 *     RDMA
 *     Ethernet
 *     InfiniBand
 *     a particular quantum-network protocol
 *     a particular vendor protocol
 *
 * Such names may be referenced by source programs, but their realization is
 * determined downstream.
 *
 * Therefore:
 *
 *     protocol requirement
 *
 * is distinct from:
 *
 *     protocol implementation
 *
 * and:
 *
 *     protocol implementation
 *
 * is distinct from:
 *
 *     physical network realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level limits exist for:
 *
 *     - protocol declarations;
 *     - protocol references;
 *     - protocol versions;
 *     - protocol capabilities;
 *     - protocol requirements;
 *     - protocol constraints;
 *     - protocol preferences;
 *     - protocol composition depth;
 *     - protocol options;
 *     - protocol metadata;
 *     - message types;
 *     - endpoints;
 *     - channels;
 *     - services;
 *     - nodes;
 *     - network size.
 *
 * ANTLR repetition operators are intentionally unbounded.
 *
 * Practical limits belong to compiler/resource policy and runtime resources,
 * not to source grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no target-specific code;
 *     - no random behavior;
 *     - no environment inspection;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access.
 *
 * Parsing is therefore deterministic and environment-independent.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required stable token:
 *
 *     PROTOCOL
 *
 * representing:
 *
 *     protocol
 *
 * This is intentionally a reserved networking declaration keyword.
 *
 * Protocol implementation names such as:
 *
 *     tcp
 *     udp
 *     quic
 *     mpi
 *     rdma
 *     http
 *     custom_transport
 *
 * remain ordinary identifiers/qualified names unless the language specification
 * explicitly reserves them.
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar consumes:
 *
 *     Names
 *     Expressions
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *
 * Expressions owns:
 *
 *     expression
 *
 * This grammar MUST NOT redefine those rules.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * Protocols is a leaf parser grammar.
 *
 * The networking aggregate grammar must import this grammar and consume:
 *
 *     protocolDeclaration
 *     protocolReference
 *
 * The aggregate networking grammar MUST NOT redefine those rules.
 *
 * The aggregate should therefore contain an integration path equivalent to:
 *
 *     networkingMember
 *         : endpointDeclaration
 *         | channelDeclaration
 *         | serviceDeclaration
 *         | protocolDeclaration
 *         | ...
 *         ;
 *
 * ============================================================================
 */

parser grammar Protocols;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * PUBLIC ENTRY POINTS
 * ========================================================================== */

/*
 * Complete protocol declaration.
 *
 * Example:
 *
 *     protocol reliable_stream {
 *         requires: ordered;
 *         requires: reliable;
 *     }
 */
protocolDeclaration
    : PROTOCOL protocolName protocolDeclarationBody?
      SEMICOLON?
    ;


/*
 * Protocol reference.
 *
 * A protocol reference identifies an existing logical protocol without
 * declaring a new one.
 *
 * Examples:
 *
 *     tcp
 *     custom::transport
 *     quantum::network::transport
 */
protocolReference
    : qualifiedName
    ;


/*
 * A protocol name is deliberately represented by the canonical name system.
 */
protocolName
    : qualifiedName
    ;


/* ============================================================================
 * PROTOCOL DECLARATION BODY
 * ========================================================================== */

protocolDeclarationBody
    : LBRACE protocolMember* RBRACE
    ;


protocolMember
    : protocolExtendsDeclaration
    | protocolVersionDeclaration
    | protocolProvidesDeclaration
    | protocolRequiresDeclaration
    | protocolConstraintDeclaration
    | protocolPreferenceDeclaration
    | protocolCapabilityDeclaration
    | protocolBindingDeclaration
    | protocolOptionDeclaration
    | protocolMetadataDeclaration
    | protocolInterfaceDeclaration
    | protocolCompositionDeclaration
    ;


/* ============================================================================
 * INHERITANCE / EXTENSION
 * ========================================================================== */

/*
 * Declares protocol-level extension intent.
 *
 * Example:
 *
 *     extends base::reliable
 *
 * This does not mean implementation inheritance.
 *
 * Semantic analysis determines whether extension is valid.
 */
protocolExtendsDeclaration
    : EXTENDS protocolReference protocolReferenceListTail? SEMICOLON
    ;


protocolReferenceListTail
    : COMMA protocolReference (COMMA protocolReference)*
    ;


/* ============================================================================
 * VERSION
 * ========================================================================== */

/*
 * Version syntax is intentionally expression-based.
 *
 * The grammar does not prescribe:
 *
 *     semantic versioning;
 *     integer-only versions;
 *     vendor version formats;
 *     protocol revision limits.
 *
 * Those policies belong to protocol/version semantic analysis.
 */
protocolVersionDeclaration
    : VERSIONING COLON expression SEMICOLON
    ;


/*
 * Explicit protocol version requirement.
 *
 * This permits a protocol declaration to distinguish:
 *
 *     declared version
 *
 * from:
 *
 *     required compatible version.
 */
protocolVersionRequirement
    : versionRequirementMarker COLON expression SEMICOLON
    ;


versionRequirementMarker
    : identifier
    ;


/* ============================================================================
 * PROVIDES
 * ========================================================================== */

/*
 * Declares semantic capabilities provided by the protocol.
 *
 * Examples:
 *
 *     provides: ordered_delivery;
 *     provides: reliable_delivery;
 *     provides: streaming;
 */
protocolProvidesDeclaration
    : PROVIDES protocolValueExpressionList SEMICOLON
    ;


/* ============================================================================
 * REQUIREMENTS
 * ========================================================================== */

/*
 * Requirements are mandatory semantic conditions.
 *
 * Examples:
 *
 *     requires: reliable;
 *     requires: ordered;
 *     requires: bidirectional;
 *     requires: secure_channel;
 *
 * A requirement does not allocate a physical resource.
 */
protocolRequiresDeclaration
    : REQUIRES protocolValueExpressionList SEMICOLON
    ;


/*
 * A reusable protocol requirement expression.
 */
protocolRequirement
    : REQUIRES COLON expression SEMICOLON
    ;


/* ============================================================================
 * CONSTRAINTS
 * ========================================================================== */

/*
 * Constraints restrict valid realizations without selecting a concrete
 * implementation.
 *
 * Example:
 *
 *     constraint: latency < desired_latency;
 */
protocolConstraintDeclaration
    : constraintMarker COLON expression SEMICOLON
    ;


constraintMarker
    : CONSTRAINT
    ;


/* ============================================================================
 * PREFERENCES
 * ========================================================================== */

/*
 * Preferences are optimization hints.
 *
 * They are weaker than requirements and constraints.
 *
 * Example:
 *
 *     prefer: low_latency;
 */
protocolPreferenceDeclaration
    : PREFER protocolValueExpressionList SEMICOLON
    ;


/* ============================================================================
 * CAPABILITIES
 * ========================================================================== */

/*
 * A protocol capability declaration describes a semantic capability exposed
 * by the protocol.
 *
 * Example:
 *
 *     capability: multicast;
 *     capability: ordered_delivery;
 */
protocolCapabilityDeclaration
    : CAPABILITY protocolValueExpressionList SEMICOLON
    ;


/* ============================================================================
 * BINDINGS
 * ========================================================================== */

/*
 * A binding associates a logical protocol with another semantic entity.
 *
 * This may later be interpreted by:
 *
 *     endpoint analysis;
 *     service analysis;
 *     channel analysis;
 *     distributed execution;
 *     hardware/network realization.
 *
 * It does NOT establish a physical address.
 */
protocolBindingDeclaration
    : BIND protocolBindingTarget COLON expression SEMICOLON
    ;


protocolBindingTarget
    : protocolBindingName
    ;


protocolBindingName
    : identifier
    ;


/* ============================================================================
 * OPTIONS
 * ========================================================================== */

/*
 * Generic protocol options deliberately use expressions.
 *
 * This prevents the grammar from embedding a finite vendor-specific option
 * vocabulary.
 *
 * Examples:
 *
 *     option framing: framing_mode;
 *     option ordering: ordered;
 *     option custom::feature: value;
 */
protocolOptionDeclaration
    : OPTION identifier COLON expression SEMICOLON
    ;


/*
 * Namespaced option key.
 *
 * Allows:
 *
 *     custom::transport::feature
 *
 * without hard-coding future protocol namespaces.
 */
protocolQualifiedOptionDeclaration
    : OPTION qualifiedName COLON expression SEMICOLON
    ;


/* ============================================================================
 * METADATA
 * ========================================================================== */

/*
 * Metadata is deliberately expression-based.
 *
 * Metadata has no inherent runtime meaning.
 *
 * Tooling, semantic analysis and downstream compilation may interpret it.
 */
protocolMetadataDeclaration
    : METADATA identifier COLON expression SEMICOLON
    ;


/*
 * Namespaced metadata key.
 */
protocolQualifiedMetadataDeclaration
    : METADATA qualifiedName COLON expression SEMICOLON
    ;


/* ============================================================================
 * INTERFACE DECLARATIONS
 * ========================================================================== */

/*
 * A protocol interface describes logical operations exposed by a protocol.
 *
 * It does not define:
 *
 *     sockets;
 *     packets;
 *     system calls;
 *     memory layouts;
 *     ABI;
 *     physical transport.
 */
protocolInterfaceDeclaration
    : INTERFACE identifier protocolInterfaceBody?
      SEMICOLON?
    ;


protocolInterfaceBody
    : LBRACE protocolInterfaceMember* RBRACE
    ;


protocolInterfaceMember
    : protocolOperationDeclaration
    | protocolEventDeclaration
    | protocolInterfaceRequirement
    | protocolInterfaceCapability
    | protocolInterfaceMetadata
    ;


protocolOperationDeclaration
    : FN identifier
      LPAREN optionalExpressionList RPAREN
      protocolReturnType?
      SEMICOLON
    ;


protocolReturnType
    : THIN_ARROW expression
    ;


protocolEventDeclaration
    : EVENT identifier
      LPAREN optionalExpressionList RPAREN
      SEMICOLON
    ;


protocolInterfaceRequirement
    : REQUIRES COLON expression SEMICOLON
    ;


protocolInterfaceCapability
    : CAPABILITY COLON expression SEMICOLON
    ;


protocolInterfaceMetadata
    : METADATA identifier COLON expression SEMICOLON
    ;


/* ============================================================================
 * PROTOCOL COMPOSITION
 * ========================================================================== */

/*
 * Protocol composition allows a protocol to be described as requiring or
 * combining other protocol contracts.
 *
 * Example:
 *
 *     compose transport::reliable,
 *             transport::ordered;
 *
 * Composition is semantic composition, not physical network topology.
 */
protocolCompositionDeclaration
    : COMPOSE protocolReferenceList SEMICOLON
    ;


protocolReferenceList
    : protocolReference
      (COMMA protocolReference)*
    ;


/* ============================================================================
 * VALUE EXPRESSIONS
 * ========================================================================== */

/*
 * Generic protocol value list.
 *
 * No fixed number of protocol values is allowed or required.
 */
protocolValueExpressionList
    : expression
      (COMMA expression)*
    ;


/* ============================================================================
 * VERSION / COMPATIBILITY EXPRESSIONS
 * ========================================================================== */

/*
 * Compatibility requirement is intentionally expressed as a normal source
 * expression.
 *
 * This permits semantic layers to support:
 *
 *     exact version
 *     minimum version
 *     maximum version
 *     compatible range
 *     capability-based compatibility
 *     dialect-specific compatibility
 *
 * without baking a finite version scheme into the grammar.
 */
protocolCompatibilityDeclaration
    : COMPATIBLE COLON expression SEMICOLON
    ;


/* ============================================================================
 * PROTOCOL REFERENCES IN GENERIC CONTEXTS
 * ========================================================================== */

/*
 * A protocol selector remains a logical name.
 *
 * It is NOT:
 *
 *     an IP address;
 *     a port;
 *     a socket;
 *     a physical interface;
 *     a machine ID;
 *     a device ID.
 */
protocolSelector
    : protocolReference
    ;


/* ============================================================================
 * PROTOCOL REQUIREMENT BLOCK
 * ========================================================================== */

/*
 * Reusable nested protocol policy block.
 *
 * Example:
 *
 *     policy {
 *         requires: reliable;
 *         capability: ordered;
 *         prefer: low_latency;
 *     }
 *
 * Policy semantics belong downstream.
 */
protocolPolicyBlock
    : POLICY LBRACE protocolPolicyMember* RBRACE
    ;


protocolPolicyMember
    : protocolRequiresDeclaration
    | protocolConstraintDeclaration
    | protocolPreferenceDeclaration
    | protocolCapabilityDeclaration
    | protocolCompatibilityDeclaration
    | protocolMetadataDeclaration
    ;


/* ============================================================================
 * PROTOCOL REFERENCE BLOCK
 * ========================================================================== */

/*
 * A reference can carry requirements/preferences without becoming a protocol
 * declaration.
 *
 * Example:
 *
 *     use protocol::reliable {
 *         requires: ordered;
 *         prefer: low_latency;
 *     }
 *
 * This remains logical intent.
 */
protocolUseDeclaration
    : USE protocolReference protocolUseBody?
      SEMICOLON?
    ;


protocolUseBody
    : LBRACE protocolUseMember* RBRACE
    ;


protocolUseMember
    : protocolRequiresDeclaration
    | protocolConstraintDeclaration
    | protocolPreferenceDeclaration
    | protocolCapabilityDeclaration
    | protocolCompatibilityDeclaration
    | protocolMetadataDeclaration
    ;


/* ============================================================================
 * PROTOCOL CONSTRAINT EXPRESSIONS
 * ========================================================================== */

/*
 * Constraint expressions remain ordinary Zamani expressions.
 *
 * This allows future resource/capability systems to interpret:
 *
 *     latency
 *     reliability
 *     ordering
 *     throughput
 *     availability
 *     energy
 *     locality
 *
 * without making those properties machine constants in this grammar.
 */
protocolConstraintExpression
    : expression
    ;


/* ============================================================================
 * PROTOCOL PREFERENCE EXPRESSIONS
 * ========================================================================== */

protocolPreferenceExpression
    : expression
    ;


/* ============================================================================
 * PROTOCOL CAPABILITY EXPRESSIONS
 * ========================================================================== */

protocolCapabilityExpression
    : expression
    ;


/* ============================================================================
 * PROTOCOL REQUIREMENT EXPRESSIONS
 * ========================================================================== */

protocolRequirementExpression
    : expression
    ;


/* ============================================================================
 * CANONICAL GENERIC MEMBERS
 * ============================================================================
 *
 * These aliases make the public protocol grammar easier for downstream
 * networking composition layers to consume without redefining expressions.
 */

protocolRequirementValue
    : protocolRequirementExpression
    ;


protocolConstraintValue
    : protocolConstraintExpression
    ;


protocolPreferenceValue
    : protocolPreferenceExpression
    ;


protocolCapabilityValue
    : protocolCapabilityExpression
    ;