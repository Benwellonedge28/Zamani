/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/networking/streaming.g4
 *
 * GRAMMAR
 * -------
 * NetworkingStreaming
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns the SOURCE-LEVEL LOGICAL STREAM CONTRACT.
 *
 * A stream describes an unbounded, potentially incremental sequence/data-flow
 * abstraction. It describes intent and relationships, not a physical
 * transport or runtime object.
 *
 * A stream may ultimately be realized through:
 *
 *     in-process data flow
 *     shared memory
 *     IPC
 *     networking
 *     distributed communication
 *     accelerator fabrics
 *     storage-backed pipelines
 *     quantum/classical interfaces
 *     hardware interfaces
 *     future communication substrates
 *
 * The grammar deliberately remains independent of the eventual realization.
 *
 *
 * ARCHITECTURAL POSITION
 * ----------------------
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     NetworkingStreaming
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract validation
 *          +--> policy validation
 *          +--> provenance
 *          |
 *          v
 *     networking semantic model
 *          |
 *          +--> classical semantics
 *          +--> distributed semantics
 *          +--> data semantics
 *          +--> hybrid semantics
 *          +--> hardware intent
 *          +--> quantum/classical communication metadata
 *          |
 *          v
 *     canonical IR / semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir when quantum computation is involved
 *          +--> HDL/hardware representation where appropriate
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     lowering
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     resilience / recovery
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 *
 * OWNS
 * ----
 *
 *     - logical stream declaration;
 *     - stream identity;
 *     - stream type annotation;
 *     - stream properties;
 *     - stream nested configuration;
 *     - stream references;
 *     - source/destination relationships as stream metadata;
 *     - channel/protocol/message/request/response relationships as references;
 *     - stream-level declarative requirements;
 *     - stream-level declarative constraints;
 *     - stream-level preferences/capability metadata expressed as properties;
 *     - stream attributes.
 *
 *
 * DOES NOT OWN
 * ------------
 *
 *     - sockets;
 *     - addresses;
 *     - endpoints;
 *     - channels;
 *     - messages;
 *     - requests;
 *     - responses;
 *     - services;
 *     - protocols;
 *     - service discovery;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - transport implementation;
 *     - serialization;
 *     - compression;
 *     - encryption implementation;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - runtime stream objects;
 *     - concurrency channels;
 *     - distributed actors;
 *     - hardware resources;
 *     - physical network topology;
 *     - quantum operations;
 *     - quantum state;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - a second IR.
 *
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/types/types.g4
 *     grammar/expressions/expressions.g4
 *     grammar/core/attributes.g4
 *
 *
 * EXPORTS
 * -------
 *
 *     networkStreamingConstruct
 *     networkStreamDeclaration
 *     networkStreamTypeAnnotation
 *     networkStreamBody
 *     networkStreamMember
 *     networkStreamProperty
 *     networkStreamPropertyName
 *     networkStreamPropertyValue
 *     networkStreamNestedBlock
 *     networkStreamNestedBlockName
 *     networkStreamNestedMember
 *     networkStreamReference
 *     networkStreamSourceReference
 *     networkStreamDestinationReference
 *     networkStreamChannelReference
 *     networkStreamProtocolReference
 *     networkStreamMessageReference
 *     networkStreamRequestReference
 *     networkStreamResponseReference
 *     networkStreamTypeReference
 *     optionalNetworkStreamBody
 *     optionalNetworkStreamTypeAnnotation
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/networking/networking.g4
 *
 * The aggregate consumes:
 *
 *     networkStreamingConstruct
 *
 * The aggregate adapter remains:
 *
 *     networkingStream
 *         : networkStreamingConstruct
 *         ;
 *
 * No change to that boundary is required by this rewrite.
 *
 *
 * RELATED NETWORKING OWNERS
 * -------------------------
 *
 *     addresses.g4
 *         address syntax
 *
 *     endpoints.g4
 *         logical communication endpoints
 *
 *     channels.g4
 *         logical networking channels
 *
 *     messages.g4
 *         message declarations and fields
 *
 *     protocols.g4
 *         protocol contracts
 *
 *     requests.g4
 *         request contracts
 *
 *     responses.g4
 *         response contracts
 *
 *     services.g4
 *         service contracts
 *
 *     sockets.g4
 *         logical socket contracts
 *
 *     service-discovery.g4
 *         discovery semantics
 *
 *     routing.g4
 *         routing intent
 *
 *     network-capabilities.g4
 *         networking capability declarations
 *
 * Streaming references these concepts by canonical names rather than
 * reproducing their grammar.
 *
 *
 * AST CONTRACT
 * ------------
 *
 * This grammar produces parser contexts only.
 *
 * The domain-neutral frontend AST owns the actual representation.
 *
 * The AST representation for a stream must preserve:
 *
 *     - declaration source span;
 *     - stream name;
 *     - optional type;
 *     - attributes;
 *     - ordered property declarations;
 *     - nested configuration structure;
 *     - property expressions;
 *     - qualified references;
 *     - source ordering.
 *
 * Property names remain symbolic until semantic analysis.
 *
 *
 * SEMANTIC CONTRACT
 * -----------------
 *
 * Semantic analysis determines:
 *
 *     - whether the stream name is valid;
 *     - whether referenced symbols exist;
 *     - whether a referenced object has the expected kind;
 *     - whether the stream type is valid;
 *     - whether properties are legal for the selected stream semantics;
 *     - whether requirements can be satisfied;
 *     - whether constraints are satisfiable;
 *     - whether capabilities exist;
 *     - whether policies permit the requested behavior;
 *     - whether effects are compatible;
 *     - whether provenance requirements are satisfied.
 *
 * The parser deliberately does not perform these checks.
 *
 *
 * TYPE CONTRACT
 * -------------
 *
 * Stream payload types use the canonical:
 *
 *     typeExpression
 *
 * rule from `Types`.
 *
 * This grammar MUST NOT define:
 *
 *     Stream<T>
 *     Tensor<T>
 *     Message<T>
 *     Dataset<T>
 *
 * as independent universal type constructors.
 *
 * They remain ordinary canonical types resolved by the type system.
 *
 *
 * EFFECT CONTRACT
 * --------------
 *
 * A stream declaration has no parser-defined effect by itself.
 *
 * Semantic analysis may derive effects such as:
 *
 *     network
 *     io
 *     distributed
 *     communication
 *     mutation
 *     randomness
 *     foreign
 *     measurement
 *
 * according to the stream's resolved meaning and downstream realization.
 *
 * This grammar does not define a second effect system.
 *
 *
 * CAPABILITY CONTRACT
 * ------------------
 *
 * Capability requirements are represented structurally through canonical
 * expressions/properties and interpreted by semantic analysis.
 *
 * Examples include:
 *
 *     requires: capability("network.streaming");
 *
 *     capability: capability("network.backpressure");
 *
 *     requires: capability("network.reliable_delivery");
 *
 * The grammar does not inspect whether a target actually provides a
 * capability.
 *
 *
 * RESOURCE CONTRACT
 * -----------------
 *
 * Stream requirements may express symbolic resource relationships, for
 * example:
 *
 *     requires: memory >= required_memory;
 *
 *     requires: throughput >= required_throughput;
 *
 *     constraint: latency <= required_latency;
 *
 * No physical capacity is encoded by this grammar.
 *
 *
 * CONTRACT CONTRACT
 * -----------------
 *
 * Stream properties may participate in canonical contract analysis.
 *
 * This grammar does not create a competing contract language.
 *
 * `requires`, `ensures`, `invariant`, `assume`, `guarantee`, and `property`
 * semantics remain owned by the repository's validation/contract subsystem.
 *
 * If such concepts occur as stream property names, semantic analysis decides
 * whether the property is a valid contract attachment.
 *
 *
 * POLICY CONTRACT
 * ---------------
 *
 * Stream properties may reference policy concepts, for example:
 *
 *     policy: network::reliable;
 *
 *     security::policy: secure_stream;
 *
 * Policy interpretation belongs to the canonical policy/security subsystem.
 *
 * This file does not implement authorization or policy evaluation.
 *
 *
 * PROVENANCE CONTRACT
 * -------------------
 *
 * Attributes and symbolic properties may carry provenance metadata.
 *
 * Provenance semantics remain owned by the repository-wide provenance system.
 *
 * The parser preserves source structure; it does not manufacture provenance
 * records.
 *
 *
 * IR CONTRACT
 * -----------
 *
 * This grammar owns NO IR.
 *
 * Streams are lowered into the canonical semantic/IR architecture.
 *
 * A stream carrying or coordinating quantum computation may contribute
 * semantic information to the quantum pipeline, but this grammar never
 * constructs `quantum::ir` directly.
 *
 *
 * QUANTUM BOUNDARY
 * ----------------
 *
 * Streams may represent:
 *
 *     measurement-result flow;
 *     classical control flow;
 *     quantum-classical coordination;
 *     telemetry;
 *     distributed quantum computation metadata.
 *
 * This grammar MUST NOT define:
 *
 *     qubit;
 *     gate;
 *     circuit;
 *     quantum state;
 *     QEC;
 *     physical quantum topology.
 *
 * Such semantics belong to the quantum subsystem.
 *
 *
 * HDL BOUNDARY
 * ------------
 *
 * A stream may carry hardware/HDL data or metadata.
 *
 * This grammar does not define:
 *
 *     signals;
 *     clocks;
 *     registers;
 *     wires;
 *     timing;
 *     synthesis;
 *     physical ports.
 *
 * HDL ownership remains in `grammar/hdl/`.
 *
 *
 * BACKEND BOUNDARY
 * ----------------
 *
 * Backends determine how a logical stream is realized.
 *
 * Possible realizations include:
 *
 *     local;
 *     shared-memory;
 *     IPC;
 *     network;
 *     distributed;
 *     accelerator;
 *     hardware;
 *     quantum-classical;
 *     simulated;
 *     future substrates.
 *
 * The stream grammar makes no realization choice.
 *
 *
 * SCALABILITY CONTRACT
 * --------------------
 *
 * All stream collections are unbounded at the language level.
 *
 * There is no grammar-defined maximum for:
 *
 *     streams;
 *     members;
 *     nested blocks;
 *     nesting depth;
 *     qualified-name depth;
 *     properties;
 *     references;
 *     payload size;
 *     payload dimensions;
 *     event count;
 *     rate;
 *     throughput;
 *     buffering;
 *     endpoints;
 *     channels;
 *     nodes;
 *     devices;
 *     processors;
 *     memory;
 *     network size.
 *
 * Repetition uses `*` or `+`.
 *
 * Practical limits are implementation/resource limits, never language
 * semantics.
 *
 *
 * HARD-CODING PROHIBITION
 * -----------------------
 *
 * This grammar contains no capacity constants and no target-specific IDs.
 *
 * It MUST NOT introduce universal constants for:
 *
 *     stream count;
 *     buffer count;
 *     buffer size;
 *     throughput;
 *     bandwidth;
 *     latency;
 *     endpoint count;
 *     channel count;
 *     node count;
 *     device count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     memory;
 *     tensor rank.
 *
 *
 * OPEN-WORLD CONTRACT
 * -------------------
 *
 * Stream property names are intentionally open-world:
 *
 *     qualifiedName : expression ;
 *
 * This allows future streaming concepts to be introduced by semantic
 * registries, dialects, capabilities, policies, or specifications without
 * creating a new parser branch for every property.
 *
 * Examples of possible semantic properties include:
 *
 *     source
 *     destination
 *     producer
 *     consumer
 *     message
 *     protocol
 *     channel
 *     request
 *     response
 *     ordering
 *     delivery
 *     reliability
 *     durability
 *     replay
 *     retention
 *     buffering
 *     backpressure
 *     flow_control
 *     rate
 *     throughput
 *     latency
 *     timeout
 *     partitioning
 *     affinity
 *     locality
 *     requires
 *     constraint
 *     prefer
 *     capability
 *     policy
 *     provenance
 *     metadata
 *     observability
 *
 * These names are deliberately NOT enumerated here.
 *
 *
 * DETERMINISM
 * -----------
 *
 * Parsing depends only on:
 *
 *     source text;
 *     canonical lexer vocabulary;
 *     grammar version;
 *     selected language compatibility configuration.
 *
 * The grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     hardware discovery;
 *     environment inspection;
 *     random generation;
 *     time-dependent decision;
 *     target selection.
 *
 *
 * SAFETY / RUST CONTRACT
 * ----------------------
 *
 * This ANTLR grammar contains:
 *
 *     - no embedded Rust;
 *     - no target-language actions;
 *     - no semantic predicates;
 *     - no unsafe operations;
 *     - no runtime execution.
 *
 * The Rust frontend remains compatible with Rust 1.97+ / Edition 2021 and
 * uses safe Rust.
 *
 *
 * TEST CONTRACT
 * -------------
 *
 * Positive tests must cover:
 *
 *     stream telemetry;
 *     stream telemetry: Measurement;
 *     stream telemetry { source: sensor; };
 *     stream telemetry {
 *         source: sensor;
 *         destination: analyzer;
 *         message: Measurement;
 *         protocol: network::stream;
 *         channel: telemetry_channel;
 *     };
 *
 * Negative tests must cover:
 *
 *     missing stream name;
 *     missing colon in a property;
 *     missing property expression;
 *     missing semicolon;
 *     malformed qualified name;
 *     malformed nested block;
 *     malformed type annotation.
 *
 * Boundary tests must cover:
 *
 *     stream + channel;
 *     stream + message;
 *     stream + protocol;
 *     stream + request;
 *     stream + response;
 *     stream + endpoint;
 *     stream + service;
 *     stream + capabilities;
 *     stream + distributed computation;
 *     stream + quantum/classical metadata;
 *     stream + HDL/hardware metadata.
 *
 * Scalability tests must use:
 *
 *     arbitrarily many properties;
 *     arbitrarily many nested blocks;
 *     arbitrarily deep qualified names;
 *     symbolic resource expressions;
 *     symbolic throughput/latency requirements;
 *     large cross-domain declarations.
 *
 * Compatibility tests must verify that adding a new semantic property does
 * not require modifying this grammar.
 *
 *
 * COMPLETION CRITERIA
 * -------------------
 *
 * This file is complete when:
 *
 *     [x] exactly one parser grammar owns networking streams;
 *     [x] exactly one public stream construction boundary exists;
 *     [x] `networkStreamingConstruct` remains stable;
 *     [x] the canonical STREAM lexer token is consumed;
 *     [x] canonical names are reused;
 *     [x] canonical types are reused;
 *     [x] canonical expressions are reused;
 *     [x] canonical attributes are reused;
 *     [x] no lexer rules are defined;
 *     [x] no duplicate parser rules exist;
 *     [x] no transport grammar is duplicated;
 *     [x] no endpoint grammar is duplicated;
 *     [x] no channel grammar is duplicated;
 *     [x] no message grammar is duplicated;
 *     [x] no request grammar is duplicated;
 *     [x] no response grammar is duplicated;
 *     [x] no service grammar is duplicated;
 *     [x] no protocol grammar is duplicated;
 *     [x] no routing grammar is duplicated;
 *     [x] no runtime behavior is encoded;
 *     [x] no target capacity is encoded;
 *     [x] no physical hardware assumption is encoded;
 *     [x] no competing IR is introduced;
 *     [x] the grammar remains open-world;
 *     [x] all repetitions remain resource-unbounded at language level;
 *     [x] the aggregate networking grammar can consume it unchanged.
 *
 *
 * ============================================================================
 * ANTLR GRAMMAR
 * ============================================================================
 */

parser grammar NetworkingStreaming;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC COMPOSITION BOUNDARY
 * ============================================================================
 *
 * This is the only rule the networking aggregate needs to consume.
 *
 * A reference is intentionally NOT a top-level networking construct.
 *
 * A bare qualified name is an expression/name and must not accidentally become
 * a networking declaration merely because this grammar is active.
 * ============================================================================
 */

networkStreamingConstruct
    : networkStreamDeclaration
    ;


/*
 * ============================================================================
 * STREAM DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     stream telemetry;
 *
 *     stream telemetry: Measurement;
 *
 *     stream telemetry {
 *         source: sensor;
 *         destination: analyzer;
 *     };
 *
 *     stream measurements: Stream<Measurement> {
 *         message: Measurement;
 *         protocol: network::stream;
 *     };
 *
 * The declaration body is optional.
 *
 * The declaration itself remains purely logical.
 * ============================================================================
 */

networkStreamDeclaration
    : attribute*
      STREAM
      qualifiedName
      networkStreamTypeAnnotation?
      networkStreamBody?
      SEMI?
    ;


/*
 * ============================================================================
 * STREAM TYPE ANNOTATION
 * ============================================================================
 *
 * Type ownership remains entirely with Types.typeExpression.
 * ============================================================================
 */

networkStreamTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * STREAM BODY
 * ============================================================================
 *
 * A stream body contains zero or more declarative members.
 *
 * There is intentionally no fixed member count.
 * ============================================================================
 */

networkStreamBody
    : LBRACE
      networkStreamMember*
      RBRACE
    ;


/*
 * ============================================================================
 * STREAM MEMBER
 * ============================================================================
 *
 * Attributes can decorate either a property or nested configuration block.
 *
 * The common attribute prefix is factored so both alternatives have the same
 * attachment semantics.
 * ============================================================================
 */

networkStreamMember
    : attribute*
      networkStreamMemberCore
    ;


networkStreamMemberCore
    : networkStreamProperty
    | networkStreamNestedBlock
    ;


/*
 * ============================================================================
 * STREAM PROPERTY
 * ============================================================================
 *
 * Canonical property form:
 *
 *     name: expression;
 *
 * The name is open-world and is resolved semantically.
 * ============================================================================
 */

networkStreamProperty
    : networkStreamPropertyName
      COLON
      networkStreamPropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * STREAM PROPERTY NAME
 * ============================================================================
 */

networkStreamPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * STREAM PROPERTY VALUE
 * ============================================================================
 *
 * The entire canonical Zamani expression language is available.
 *
 * This permits:
 *
 *     names;
 *     literals;
 *     arithmetic;
 *     comparisons;
 *     boolean expressions;
 *     calls;
 *     collections;
 *     symbolic resource quantities;
 *     capability expressions;
 *     policy expressions;
 *     uncertainty;
 *     reasoning;
 *     quantum/classical values;
 *     future expression extensions.
 * ============================================================================
 */

networkStreamPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * NESTED STREAM BLOCK
 * ============================================================================
 *
 * Nested blocks provide structural extensibility without defining a finite
 * catalogue of streaming features.
 *
 * Example:
 *
 *     stream telemetry {
 *         buffering {
 *             policy: adaptive;
 *             capacity: required_capacity;
 *         };
 *
 *         flow_control {
 *             mode: dynamic;
 *         };
 *     };
 *
 * Meaning is resolved semantically.
 * ============================================================================
 */

networkStreamNestedBlock
    : networkStreamNestedBlockName
      LBRACE
      networkStreamNestedMember*
      RBRACE
      SEMI?
    ;


networkStreamNestedBlockName
    : qualifiedName
    ;


networkStreamNestedMember
    : attribute*
      networkStreamNestedMemberCore
    ;


networkStreamNestedMemberCore
    : networkStreamProperty
    | networkStreamNestedBlock
    ;


/*
 * ============================================================================
 * STREAM REFERENCE
 * ============================================================================
 *
 * This rule is reusable by downstream grammars and tooling.
 *
 * It is deliberately NOT an alternative of networkStreamingConstruct.
 *
 * A stream reference names an already-declared logical stream.
 *
 * It does not open, close, send, receive, allocate, or execute anything.
 * ============================================================================
 */

networkStreamReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * STABLE SEMANTIC REFERENCE ADAPTERS
 * ============================================================================
 *
 * These adapters provide stable rule names to semantic/frontend consumers.
 *
 * They contain no duplicated syntax from the referenced networking grammars.
 *
 * Semantic analysis verifies that each name actually denotes the expected
 * declaration kind.
 * ============================================================================
 */

networkStreamSourceReference
    : qualifiedName
    ;


networkStreamDestinationReference
    : qualifiedName
    ;


networkStreamChannelReference
    : qualifiedName
    ;


networkStreamProtocolReference
    : qualifiedName
    ;


networkStreamMessageReference
    : qualifiedName
    ;


networkStreamRequestReference
    : qualifiedName
    ;


networkStreamResponseReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * STREAM TYPE REFERENCE
 * ============================================================================
 *
 * Adapter to the canonical type system.
 * ============================================================================
 */

networkStreamTypeReference
    : typeExpression
    ;


/*
 * ============================================================================
 * OPTIONAL WRAPPERS
 * ============================================================================
 *
 * These wrappers are intentionally small compatibility/composition boundaries.
 *
 * They do not create additional syntax.
 * ============================================================================
 */

optionalNetworkStreamBody
    : networkStreamBody?
    ;


optionalNetworkStreamTypeAnnotation
    : networkStreamTypeAnnotation?
    ;