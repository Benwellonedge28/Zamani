/*
 * ============================================================================
 * Zamani Universal Data Deserialization Grammar
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/data/deserialization.g4
 *
 * STATUS
 * ------
 * CANONICAL COMPATIBILITY / INTEGRATION FACADE
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the stable deserialization-facing grammar boundary for
 * existing repository consumers while delegating the actual deserialization
 * syntax to the canonical:
 *
 *     grammar/data/serialization.g4
 *
 * Serialization and deserialization form one representation-conversion
 * semantic family.
 *
 * Therefore this file MUST NOT define an independent deserialization grammar.
 *
 * The canonical owner is:
 *
 *     grammar/data/serialization.g4
 *
 * This file exists only so that code, tests, documentation, or transitional
 * grammar composition that refers specifically to "deserialization" can
 * continue to use a stable named boundary without creating a competing
 * syntax authority.
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                     ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                           Data
 *                              |
 *                              v
 *                   serialization.g4
 *                              |
 *                 +------------+------------+
 *                 |                         |
 *                 v                         v
 *            serialization             deserialization
 *                 |                         |
 *                 +------------+------------+
 *                              |
 *                              v
 *                    domain-neutral AST
 *                              |
 *                              v
 *                     semantic analysis
 *                              |
 *                              v
 *                    canonical data model
 *                              |
 *                              v
 *                    compiler / runtime
 *
 * `serialization.g4` is the single syntax owner.
 *
 * ============================================================================
 * THIS FILE OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *   - the deserialization compatibility façade;
 *   - stable deserialization-facing entry-point aliases;
 *   - isolated deserialization grammar testing boundary;
 *   - migration documentation through grammar structure.
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - serialization syntax;
 *   - deserialization syntax implementation;
 *   - format syntax;
 *   - schema syntax;
 *   - type syntax;
 *   - expression syntax;
 *   - identifier syntax;
 *   - literal syntax;
 *   - compatibility algorithms;
 *   - schema evolution algorithms;
 *   - decoding;
 *   - codecs;
 *   - compression;
 *   - decompression;
 *   - encryption;
 *   - decryption;
 *   - integrity verification;
 *   - cryptographic verification;
 *   - storage;
 *   - filesystem access;
 *   - network access;
 *   - database access;
 *   - memory allocation;
 *   - streaming implementation;
 *   - resource allocation;
 *   - hardware discovery;
 *   - device selection;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL/hardware IR;
 *   - runtime execution.
 *
 * ============================================================================
 * SINGLE-OWNER INVARIANT
 * ============================================================================
 *
 * There MUST be exactly one grammar authority for deserialization syntax:
 *
 *     grammar/data/serialization.g4
 *
 * This file MUST NOT contain another implementation such as:
 *
 *     deserializationStatement
 *         : 'deserialize' ...
 *
 * with independently maintained alternatives.
 *
 * Such duplication would allow the two grammars to diverge.
 *
 * Instead:
 *
 *     deserialization.g4
 *             |
 *             v
 *     serialization.g4
 *             |
 *             v
 *     deserializationStatement
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This parser grammar consumes the repository's canonical lexer vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * MUST be used.
 *
 * The obsolete:
 *
 *     tokenVocab = Zamani;
 *
 * boundary MUST NOT be restored here.
 *
 * This file does not define lexer rules.
 *
 * ============================================================================
 * RUST / SAFETY CONTRACT
 * ============================================================================
 *
 * This is an ANTLR parser grammar.
 *
 * It contains:
 *
 *   - no embedded Rust;
 *   - no target-language actions;
 *   - no semantic predicates;
 *   - no unsafe operations;
 *   - no filesystem access;
 *   - no network access;
 *   - no environment access;
 *   - no runtime execution.
 *
 * The consuming Zamani implementation remains responsible for:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * compatibility.
 *
 * The implementation must use safe Rust.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Deserialization describes conversion from an external representation to a
 * logical Zamani value.
 *
 * It MUST NOT encode universal limits for:
 *
 *   - CPU count;
 *   - core count;
 *   - thread count;
 *   - GPU count;
 *   - FPGA count;
 *   - accelerator count;
 *   - QPU count;
 *   - qubit count;
 *   - node count;
 *   - device count;
 *   - memory capacity;
 *   - storage capacity;
 *   - network capacity;
 *   - tensor rank;
 *   - tensor dimensions;
 *   - register width;
 *   - buffer size;
 *   - record count;
 *   - field count;
 *   - collection cardinality;
 *   - stream cardinality;
 *   - schema count.
 *
 * In particular, this grammar must never introduce universal constants such
 * as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * A program-level quantity is semantic data.
 *
 * An implementation's available resources are determined downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This façade is deterministic.
 *
 * Parsing depends only upon:
 *
 *   - source text;
 *   - selected language/grammar version;
 *   - canonical lexer vocabulary;
 *   - parser composition.
 *
 * It must not depend upon:
 *
 *   - target hardware;
 *   - available memory;
 *   - runtime state;
 *   - filesystem state;
 *   - network state;
 *   - wall-clock time;
 *   - randomness;
 *   - environment variables.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * This façade introduces no new semantic node.
 *
 * The source span of the delegated construct belongs to the canonical
 * deserialization AST representation supplied by the frontend.
 *
 * The downstream AST must preserve:
 *
 *   - operation kind;
 *   - source expression;
 *   - source/representation expression;
 *   - format reference;
 *   - target type;
 *   - schema information;
 *   - options;
 *   - source span.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file introduces NO new AST node.
 *
 * The following canonical mapping remains authoritative:
 *
 *     deserializationStatement
 *         ->
 *     generic deserialization operation AST
 *
 *     deserializationExpression
 *         ->
 *     expression-level deserialization operation AST
 *
 *     dataDeserializationStmt
 *         ->
 *     canonical data deserialization operation
 *
 *     dataDeserializationExpression
 *         ->
 *     canonical data deserialization expression
 *
 * The façade aliases below therefore do not imply separate AST types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not this grammar, resolves:
 *
 *   - format identity;
 *   - format availability;
 *   - format version;
 *   - dialect;
 *   - schema identity;
 *   - schema version;
 *   - compatibility;
 *   - target type validity;
 *   - unknown-field policy;
 *   - duplicate-field policy;
 *   - null policy;
 *   - missing-field policy;
 *   - numeric representation;
 *   - canonicalization;
 *   - framing;
 *   - encoding;
 *   - integrity;
 *   - resource requirements;
 *   - capabilities;
 *   - security policy;
 *   - portability.
 *
 * The grammar only establishes syntactic structure.
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * The canonical data grammar currently exposes:
 *
 *     dataStmt
 *         |
 *         +-- dataSerializationStmt
 *
 * The canonical serialization grammar supplies:
 *
 *     dataSerializationStmt
 *
 * which includes both:
 *
 *     serialization
 *     deserialization
 *
 * Consequently this façade MUST NOT be added as a second sibling
 * implementation beneath `dataStmt`.
 *
 * Correct production composition is:
 *
 *     data.g4
 *          |
 *          v
 *     serialization.g4
 *          |
 *          +-- serializationStatement
 *          |
 *          +-- deserializationStatement
 *
 * This compatibility façade is available for consumers that explicitly need
 * a deserialization-named grammar boundary, but it is not another data
 * dispatcher.
 *
 * ============================================================================
 * SERIALIZATION INTEGRATION
 * ============================================================================
 *
 * The canonical sibling:
 *
 *     grammar/data/serialization.g4
 *
 * owns BOTH directions of representation conversion.
 *
 * This façade imports it rather than reproducing any of its rules.
 *
 * Therefore changes to:
 *
 *     format
 *     schema
 *     compatibility
 *     canonicalization
 *     framing
 *     encoding
 *     validation
 *     policies
 *     options
 *
 * are made once in serialization.g4.
 *
 * This file does not need to be edited when those canonical rules evolve.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * External formats remain open-world semantic identifiers.
 *
 * This façade does not enumerate:
 *
 *     JSON
 *     XML
 *     CBOR
 *     MessagePack
 *     Protobuf
 *     Avro
 *     Parquet
 *     Arrow
 *
 * as a closed set.
 *
 * Format recognition belongs to the canonical serialization grammar and
 * semantic/interoperability registry.
 *
 * Future formats therefore do not require this file to change.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Deserialized values may become:
 *
 *   - scalar values;
 *   - integers;
 *   - floating values;
 *   - vectors;
 *   - matrices;
 *   - tensors;
 *   - records;
 *   - collections;
 *   - application-defined values.
 *
 * This façade does not define those types.
 *
 * Canonical type syntax remains owned by the type system.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Deserialization may produce data associated with quantum computation where
 * the semantic type system permits it.
 *
 * This façade MUST NOT:
 *
 *   - allocate qubits;
 *   - identify physical qubits;
 *   - select a QPU;
 *   - select a gate set;
 *   - perform routing;
 *   - perform scheduling;
 *   - perform calibration;
 *   - perform QEC;
 *   - implement ZQN;
 *   - construct quantum::ir.
 *
 * The canonical quantum boundary remains:
 *
 *     semantic analysis
 *             |
 *             v
 *         quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Deserialized data may be consumed by HDL/hardware co-design semantics.
 *
 * This façade does not define:
 *
 *   - register widths;
 *   - bus widths;
 *   - physical addresses;
 *   - memory banks;
 *   - FPGA resources;
 *   - ASIC structures;
 *   - clock domains;
 *   - pipeline depth.
 *
 * Those remain downstream concerns.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Deserialization may consume:
 *
 *   - local values;
 *   - streams;
 *   - network results;
 *   - distributed data;
 *   - persistent data;
 *   - external endpoints.
 *
 * The source expression identifies the logical source.
 *
 * This façade does not choose:
 *
 *   - node;
 *   - process;
 *   - provider;
 *   - network route;
 *   - storage engine;
 *   - partition;
 *   - replica.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Deserialization may participate in the repository's resource/capability
 * system.
 *
 * Examples of semantic intent include:
 *
 *     requires capability("data.decode")
 *
 *     requires capability("streaming.decode")
 *
 *     requires memory >= required_memory
 *
 * These are not grammar-level machine limits.
 *
 * Capability availability and resource satisfaction are resolved downstream.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Deserialization is an important trust boundary.
 *
 * The grammar itself does not perform security validation.
 *
 * Semantic/runtime layers must be able to enforce policies for:
 *
 *   - untrusted input;
 *   - schema validation;
 *   - type validation;
 *   - integrity;
 *   - authentication;
 *   - authorization;
 *   - cryptographic verification;
 *   - resource exhaustion;
 *   - recursion/depth controls;
 *   - decompression safety;
 *   - object/reference resolution.
 *
 * Such implementation limits MUST remain environmental/security controls,
 * not universal language grammar limits.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This façade adds only delegation.
 *
 * It must not:
 *
 *   - reparse source text;
 *   - duplicate semantic analysis;
 *   - duplicate AST construction;
 *   - scan external representations;
 *   - perform decoding;
 *   - perform resource discovery.
 *
 * Therefore performance characteristics remain governed by the canonical
 * serialization grammar and parser implementation.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing consumers that refer to:
 *
 *     grammar/data/deserialization.g4
 *
 * retain a stable file boundary.
 *
 * The actual syntax authority moves to:
 *
 *     grammar/data/serialization.g4
 *
 * This allows future deserialization syntax changes to occur in exactly one
 * place.
 *
 * No legacy duplicate syntax is retained here.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests directly targeting this file verify delegation and compatibility.
 *
 * They MUST NOT duplicate the complete serialization/deserialization test
 * suite.
 *
 * REQUIRED POSITIVE CASES
 * -----------------------
 *
 *     deserialize payload using json as User;
 *
 *     deserialize payload using cbor as telemetry::Event;
 *
 *     deserialize payload using vendor::format as Data;
 *
 *     deserialize source using future::representation as Value;
 *
 * REQUIRED EXPRESSION CASES
 * -------------------------
 *
 *     deserialize(payload) using json as User
 *
 *     deserialize(stream) using vendor::format as telemetry::Event
 *
 * REQUIRED OPTION CASES
 * ---------------------
 *
 * Canonical option forms supported by serialization.g4 must remain accepted
 * through its delegated deserialization rules.
 *
 * REQUIRED NEGATIVE CASES
 * -----------------------
 *
 * malformed deserialize statements;
 *
 * missing source;
 *
 * missing format where the canonical syntax requires one;
 *
 * missing target type where required;
 *
 * malformed format references;
 *
 * malformed option expressions.
 *
 * Exact diagnostics belong to the canonical diagnostics system.
 *
 * REQUIRED SCALABILITY CASES
 * --------------------------
 *
 * Verify delegation for inputs containing:
 *
 *   - many options;
 *   - large logical schemas;
 *   - deeply composed types;
 *   - many fields;
 *   - large expressions;
 *   - qualified format names with many components.
 *
 * No test may define an artificial maximum.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Equivalent input must produce equivalent parse structures.
 *
 * Parsing must not vary according to:
 *
 *   - hardware;
 *   - available memory;
 *   - network;
 *   - runtime state;
 *   - filesystem state;
 *   - target provider.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also contains no fixed:
 *
 *     format count;
 *     schema count;
 *     field count;
 *     collection count;
 *     device count;
 *     resource count.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is parser-only.
 * [x] It consumes ZamaniLexer.
 * [x] It imports canonical serialization grammar.
 * [x] It owns no independent deserialization syntax.
 * [x] It introduces no second serialization/deserialization AST.
 * [x] It introduces no IR.
 * [x] It introduces no decoder.
 * [x] It introduces no codec.
 * [x] It introduces no storage implementation.
 * [x] It introduces no network implementation.
 * [x] It introduces no hardware implementation.
 * [x] It introduces no quantum implementation.
 * [x] It introduces no QEC implementation.
 * [x] It introduces no ZQN implementation.
 * [x] It introduces no routing/scheduling implementation.
 * [x] It introduces no resource limits.
 * [x] It introduces no unsafe Rust.
 * [x] It preserves a stable deserialization-facing boundary.
 * [x] Canonical syntax remains owned by serialization.g4.
 * [x] The data subsystem has one serialization/deserialization syntax owner.
 * [x] AST mapping remains canonical.
 * [x] Semantic validation remains downstream.
 * [x] POCO-REAF remains intact.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * CANONICAL PARSER GRAMMAR
 * ============================================================================
 *
 * This grammar is intentionally a façade.
 *
 * `serialization` is the canonical owner of:
 *
 *     serializationStatement
 *     deserializationStatement
 *     serializationExpression
 *     deserializationExpression
 *     dataSerializationStmt
 *     dataSerializationExpression
 *     serializationContractDeclaration
 *     format references
 *     schema references
 *     serialization options
 *     deserialization options
 *
 * ============================================================================
 */

parser grammar ZamaniDataDeserializationParser;

options {
    tokenVocab = ZamaniLexer;
}

import serialization;


/*
 * ============================================================================
 * ISOLATED DESERIALIZATION TEST ENTRY POINT
 * ============================================================================
 *
 * This rule is intended only for focused grammar/conformance testing.
 *
 * It is NOT a competing complete-program entry point.
 *
 * ============================================================================
 */

deserializationUnit
    : deserializationConstruct* EOF
    ;


/*
 * ============================================================================
 * DESERIALIZATION CONSTRUCT
 * ============================================================================
 *
 * Delegates directly to the canonical grammar.
 * ============================================================================
 */

deserializationConstruct
    : deserializationStatement
    | deserializationExpression
    ;


/*
 * ============================================================================
 * STABLE DATA-DOMAIN FACADE
 * ============================================================================
 *
 * These aliases provide a stable deserialization-facing API without defining
 * another implementation.
 *
 * ============================================================================
 */

dataDeserializationStmt
    : deserializationStatement
    ;

dataDeserializationExpression
    : deserializationExpression
    ;