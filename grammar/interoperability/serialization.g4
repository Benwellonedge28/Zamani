/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/interoperability/serialization.g4
 *
 * STATUS
 * ------
 * PRODUCTION INTEROPERABILITY FACADE
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the interoperability-facing boundary for Zamani serialization
 * and deserialization syntax.
 *
 * IMPORTANT:
 *
 * This file is NOT a second serialization grammar.
 *
 * The canonical serialization/deserialization syntax is owned exclusively by:
 *
 *     grammar/data/serialization.g4
 *
 * That grammar already owns:
 *
 *     serialization;
 *     deserialization;
 *     serialization expressions;
 *     deserialization expressions;
 *     serialization contracts;
 *     format references;
 *     schema references;
 *     compatibility;
 *     evolution;
 *     canonicalization;
 *     framing;
 *     compression;
 *     integrity;
 *     encoding;
 *     byte order;
 *     null/missing policies;
 *     unknown/duplicate-field policies;
 *     metadata;
 *     streaming;
 *     chunking;
 *     validation;
 *     capability requirements;
 *     resource requirements;
 *     preferences;
 *     hints;
 *     extensible properties.
 *
 * This file exposes those existing constructs through the interoperability
 * namespace without redefining them.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 *     grammar/DESIGN.md
 *             |
 *             v
 *     grammar/specification/
 *             |
 *             v
 *     grammar/spec/data.md
 *             |
 *             v
 *     grammar/data/serialization.g4
 *             |
 *             +------------------------------+
 *             |                              |
 *             v                              v
 *     grammar/data/deserialization.g4   this file
 *             |                              |
 *             +---------------+--------------+
 *                             |
 *                             v
 *                  canonical Zamani parser
 *                             |
 *                             v
 *                    domain-neutral AST
 *                             |
 *                             v
 *                    semantic analysis
 *                             |
 *             +---------------+----------------+
 *             |               |                |
 *             v               v                v
 *          classical       quantum          hardware
 *             |               |                |
 *             |               v                |
 *             |          quantum::ir           |
 *             |                                |
 *             +---------------+----------------+
 *                             |
 *                             v
 *                    canonical semantic model
 *                             |
 *                             v
 *                           IR
 *                             |
 *                             v
 *                compiler / linker / runtime
 *
 * This file never becomes a competing authority.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Serialization and deserialization form ONE semantic representation-
 * conversion family.
 *
 * The single syntax owner is:
 *
 *     grammar/data/serialization.g4
 *
 * The existing:
 *
 *     grammar/data/deserialization.g4
 *
 * is a compatibility façade.
 *
 * This file is an interoperability façade.
 *
 * Consequently there must never be independent implementations of:
 *
 *     serializationStatement
 *     deserializationStatement
 *     serializationExpression
 *     deserializationExpression
 *     serializationContractDeclaration
 *
 * in this file.
 *
 * If canonical serialization syntax changes, this file remains stable unless
 * the public interoperability boundary itself changes.
 *
 * ============================================================================
 * WHAT THIS FILE OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *   1. interoperability parser identity;
 *   2. import of the canonical serialization grammar;
 *   3. stable interoperability entry points;
 *   4. isolated interoperability conformance entry points;
 *   5. semantic delegation boundaries;
 *   6. interoperability-level source provenance.
 *
 * ============================================================================
 * WHAT THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - serialization syntax;
 *   - deserialization syntax;
 *   - formats;
 *   - codecs;
 *   - encoders;
 *   - decoders;
 *   - schemas;
 *   - schema evolution algorithms;
 *   - compatibility algorithms;
 *   - compression;
 *   - decompression;
 *   - encryption;
 *   - decryption;
 *   - cryptographic verification;
 *   - integrity implementation;
 *   - storage;
 *   - filesystems;
 *   - databases;
 *   - networking;
 *   - transport;
 *   - memory allocation;
 *   - resource discovery;
 *   - hardware discovery;
 *   - device selection;
 *   - scheduling;
 *   - routing;
 *   - runtime execution;
 *   - AST implementation;
 *   - semantic analysis;
 *   - canonical IR definitions;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Serialization belongs to the data representation boundary.
 *
 * Interoperability needs a stable named boundary because serialized
 * representations can cross:
 *
 *     Zamani
 *       |
 *       +-- classical systems
 *       +-- quantum systems
 *       +-- hybrid systems
 *       +-- HDL/hardware systems
 *       +-- AI/ML systems
 *       +-- distributed systems
 *       +-- networking systems
 *       +-- foreign languages
 *       +-- external tools
 *       +-- persistent artifacts
 *       +-- future computational domains
 *
 * The interoperability subsystem therefore needs a stable way to identify
 * serialization operations without copying the canonical data grammar.
 *
 * ============================================================================
 * FORMAT OPEN-WORLD RULE
 * ============================================================================
 *
 * Formats are semantic identifiers.
 *
 * This file MUST NOT enumerate a finite set of formats.
 *
 * It MUST NOT define:
 *
 *     json
 *     xml
 *     cbor
 *     protobuf
 *     messagepack
 *
 * as a closed grammar-level list.
 *
 * The canonical serialization grammar already represents format identity
 * through the shared qualified-name mechanism.
 *
 * Therefore future formats remain representable without changing this file.
 *
 * Examples of semantic format identities include:
 *
 *     json
 *     cbor
 *     protobuf
 *     vendor::format
 *     organization::format
 *     domain::representation
 *     future::format
 *
 * Whether a format actually exists is a semantic/tooling/provider concern,
 * not a parser concern.
 *
 * ============================================================================
 * SCHEMA OPEN-WORLD RULE
 * ============================================================================
 *
 * Schemas are also semantic identifiers.
 *
 * This file MUST NOT enumerate:
 *
 *     User
 *     Event
 *     Tensor
 *     QuantumState
 *     HardwareDescription
 *
 * as a fixed schema vocabulary.
 *
 * Existing canonical schema/type/name rules remain authoritative.
 *
 * ============================================================================
 * SERIALIZATION IS NOT A TYPE SYSTEM
 * ============================================================================
 *
 * Serialization syntax may refer to a target type during deserialization:
 *
 *     deserialize payload using json as User;
 *
 * The type itself remains owned by the canonical Zamani type system.
 *
 * This grammar MUST NOT create:
 *
 *     SerializationType
 *     DeserializationType
 *     JsonType
 *     BinaryType
 *
 * or another type universe.
 *
 * ============================================================================
 * SERIALIZATION IS NOT AN IR
 * ============================================================================
 *
 * This file introduces no:
 *
 *     SerializationIR
 *     SerializationInteropIR
 *     DataSerializationIR
 *     QuantumSerializationIR
 *     HardwareSerializationIR
 *
 * Serialization operations are represented by the existing canonical
 * frontend/semantic structures and then lowered through the repository's
 * canonical data/interoperability pipeline.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Serialization may represent values associated with quantum computation.
 *
 * However:
 *
 *     serialization != quantum semantics
 *
 * and:
 *
 *     serialization != quantum::ir
 *
 * A quantum computation continues to follow:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * Serialization may subsequently operate on an explicitly serializable
 * semantic representation where such conversion is supported.
 *
 * This file MUST NOT introduce:
 *
 *     physical qubit identifiers;
 *     gate enumerations;
 *     QPU identifiers;
 *     QEC implementation;
 *     device topology;
 *     pulse representation;
 *     backend-specific state formats.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical values may be serialized without requiring a separate
 * interoperability language.
 *
 * Examples include:
 *
 *     scalar values;
 *     collections;
 *     records;
 *     tensors;
 *     datasets;
 *     streams;
 *     program metadata;
 *     compiler artifacts.
 *
 * The actual representation conversion is downstream.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Serialization may be used to represent hardware descriptions, hardware
 * configuration data, telemetry, verification artifacts, simulation state,
 * design metadata, or other explicitly serializable semantic objects.
 *
 * This file does NOT define:
 *
 *     FPGA formats;
 *     ASIC formats;
 *     register layouts;
 *     physical addresses;
 *     device IDs;
 *     bus widths;
 *     memory capacities;
 *     physical topology.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Serialization may apply to:
 *
 *     tensors;
 *     datasets;
 *     models;
 *     model metadata;
 *     checkpoints;
 *     training artifacts;
 *     inference results;
 *     streams;
 *     provenance.
 *
 * Framework-specific representations remain outside this grammar.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Serialization can participate in:
 *
 *     message exchange;
 *     persistent state;
 *     checkpoints;
 *     replication;
 *     snapshots;
 *     service interfaces;
 *     distributed computation.
 *
 * This grammar does not own:
 *
 *     network topology;
 *     node discovery;
 *     transport;
 *     retry algorithms;
 *     replication algorithms;
 *     scheduling.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Serialization syntax must remain independent of physical machine size.
 *
 * It MUST NOT impose language-level limits on:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     accelerator count;
 *     QPU count;
 *     qubit count;
 *     node count;
 *     device count;
 *     memory capacity;
 *     storage capacity;
 *     network capacity;
 *     thread count;
 *     tensor rank;
 *     tensor dimensions;
 *     register width;
 *     buffer count;
 *     field count;
 *     schema size;
 *     message count.
 *
 * The following names are explicitly forbidden as universal language limits:
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
 * A number appearing in a program remains program semantics.
 *
 * A physical capacity remains a resource/capability concern.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Serialization interoperability must preserve the distinction between:
 *
 *     REQUIREMENT
 *         A condition that must be satisfied.
 *
 *     CAPABILITY
 *         A capability supplied by a realization.
 *
 *     CONSTRAINT
 *         A restriction on legal implementations.
 *
 *     PREFERENCE
 *         Non-binding optimization guidance.
 *
 *     HINT
 *         Non-binding implementation information.
 *
 *     REALIZATION
 *         A downstream implementation decision.
 *
 * This grammar does not collapse these concepts.
 *
 * ============================================================================
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * Serialization syntax is intentionally unbounded with respect to:
 *
 *     option count;
 *     contract members;
 *     metadata;
 *     qualified-name depth;
 *     schema references;
 *     expression complexity;
 *     logical streaming configuration;
 *     nested representations.
 *
 * Repetition is delegated to the canonical serialization grammar.
 *
 * "Infinity" means:
 *
 *     no artificial language-level serialization ceiling.
 *
 * It does NOT mean:
 *
 *     infinite memory;
 *     infinite storage;
 *     infinite bandwidth;
 *     infinite execution time.
 *
 * Actual resource exhaustion is handled by downstream implementation and
 * explicit resource policy.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar is deterministic with respect to:
 *
 *     source text;
 *     canonical lexical vocabulary;
 *     selected grammar version;
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU availability;
 *     GPU availability;
 *     FPGA availability;
 *     QPU availability;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     runtime state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing serialized-data syntax does not imply trust in serialized data.
 *
 * In particular:
 *
 *     syntactic acceptance != structural validation
 *     structural validation != semantic validation
 *     semantic validation != authorization
 *     deserialization != code execution
 *
 * Deserialized input MUST remain untrusted until the appropriate downstream
 * validation and security phases accept it.
 *
 * This grammar introduces no:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     dynamic library loading;
 *     code execution;
 *     cryptographic execution;
 *     backend execution.
 *
 * ============================================================================
 * SOURCE PROVENANCE
 * ============================================================================
 *
 * The interoperability façade must preserve the source span of the delegated
 * canonical serialization construct.
 *
 * Source provenance is required for:
 *
 *     diagnostics;
 *     IDE navigation;
 *     source maps;
 *     semantic errors;
 *     schema errors;
 *     compatibility errors;
 *     tooling;
 *     audit trails.
 *
 * No interoperability wrapper may discard the location of the underlying
 * serialization operation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file introduces NO new AST node family.
 *
 * The expected mapping is:
 *
 *     interoperability serialization entry
 *             |
 *             v
 *     canonical serialization grammar
 *             |
 *             v
 *     existing generic serialization/deserialization AST
 *             |
 *             v
 *     semantic analysis
 *
 * In particular, do NOT introduce:
 *
 *     InteropSerializationNode
 *     InteropDeserializationNode
 *     SerializationInteropAst
 *
 * merely because the source entered through interoperability.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for resolving:
 *
 *     format identity;
 *     schema identity;
 *     target type;
 *     compatibility;
 *     version;
 *     evolution;
 *     canonicalization;
 *     framing;
 *     compression;
 *     integrity;
 *     encoding;
 *     byte order;
 *     null policy;
 *     missing-field policy;
 *     unknown-field policy;
 *     duplicate-field policy;
 *     validation policy;
 *     capability requirements;
 *     resource requirements;
 *     preferences;
 *     hints.
 *
 * This grammar does not evaluate any of those concepts.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * No IR is defined here.
 *
 * The semantic result is lowered through the repository's canonical
 * data/interoperability representation.
 *
 * Serialization must not create:
 *
 *     a second data IR;
 *     a second interoperability IR;
 *     a second quantum IR;
 *     a hardware-specific serialization IR.
 *
 * For quantum-related data, the canonical quantum semantic path remains:
 *
 *     quantum semantics -> quantum::ir
 *
 * For hardware-related data:
 *
 *     hardware semantics -> canonical hardware/HDL representation
 *
 * Serialization remains a representation boundary around those semantic
 * objects rather than replacing them.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler pipeline remains:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     target/runtime realization
 *
 * This grammar only participates at the parsing boundary.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime serialization/deserialization mechanisms remain outside the grammar.
 *
 * The runtime may provide:
 *
 *     codecs;
 *     streaming;
 *     storage;
 *     transport;
 *     checkpointing;
 *     persistence;
 *     IPC;
 *     distributed communication.
 *
 * None of these mechanisms are invoked by this parser grammar.
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * This boundary can coexist with:
 *
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/foreign-functions.g4
 *     grammar/interoperability/foreign-types.g4
 *     grammar/interoperability/calling-conventions.g4
 *     grammar/interoperability/abi.g4
 *     grammar/interoperability/c.g4
 *     grammar/interoperability/cpp.g4
 *     grammar/interoperability/python.g4
 *     grammar/interoperability/rust.g4
 *     grammar/interoperability/wasm.g4
 *     grammar/interoperability/qasm.g4
 *     grammar/interoperability/qir.g4
 *     grammar/interoperability/hdl.g4
 *
 * Those grammars own their respective interoperability boundaries.
 *
 * This file does not import or duplicate them.
 *
 * ============================================================================
 * DATA-SUBSYSTEM INTEGRATION
 * ============================================================================
 *
 * The canonical owner is:
 *
 *     grammar/data/serialization.g4
 *
 * Existing:
 *
 *     grammar/data/deserialization.g4
 *
 * remains a compatibility façade.
 *
 * Therefore the dependency direction is:
 *
 *     interoperability/serialization.g4
 *                |
 *                v
 *          data/serialization.g4
 *
 * and never:
 *
 *     data/serialization.g4
 *                |
 *                v
 *     interoperability/serialization.g4
 *
 * This prevents a cyclic grammar authority.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * This grammar contains no lexer rules.
 *
 * The parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * supplied by the repository's canonical lexical system.
 *
 * This file MUST NOT introduce:
 *
 *     SERIALIZATION_FORMAT
 *     SERIALIZATION_SCHEMA
 *     SERIALIZATION_TYPE
 *     JSON_TOKEN
 *     CBOR_TOKEN
 *     PROTOBUF_TOKEN
 *
 * or another interoperability-specific lexer vocabulary.
 *
 * ============================================================================
 * ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * `serialization` is an existing parser grammar:
 *
 *     grammar/data/serialization.g4
 *
 * This file imports that parser grammar directly.
 *
 * No combined grammar is imported.
 *
 * No lexer grammar is duplicated.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * `serializationInteroperabilityUnit`
 *
 * is the isolated interoperability conformance entry point.
 *
 * It delegates to the canonical serialization unit.
 *
 * `serializationInteroperabilityConstruct`
 *
 * is a stable construct-level façade for interoperability consumers that
 * operate on individual serialization constructs.
 *
 * No syntax is added by either rule.
 *
 * ============================================================================
 * ISOLATED TESTING
 * ============================================================================
 *
 * Tests for this file must verify delegation rather than recreate the entire
 * serialization test suite.
 *
 * Required positive cases include canonical forms such as:
 *
 *     serialize value using json;
 *
 *     serialize value to destination using vendor::format;
 *
 *     deserialize payload using json as User;
 *
 *     deserialize payload using vendor::format as telemetry::Event;
 *
 *     serialize(value) using cbor;
 *
 *     deserialize(payload) using future::representation as Value;
 *
 * Required contract cases include:
 *
 *     serialization contracts;
 *     schema identity;
 *     version identity;
 *     compatibility;
 *     canonicalization;
 *     framing;
 *     compression;
 *     integrity;
 *     encoding;
 *     byte order;
 *     metadata;
 *     streaming;
 *     chunking;
 *     requirements;
 *     capabilities;
 *     preferences;
 *     hints.
 *
 * Required negative cases include:
 *
 *     malformed serialization;
 *     malformed deserialization;
 *     missing format;
 *     malformed format reference;
 *     missing target type for canonical deserialization syntax;
 *     malformed option lists;
 *     malformed contract members.
 *
 * These tests belong under:
 *
 *     grammar/tests/interoperability/
 *
 * and must reuse canonical data serialization tests where possible.
 *
 * ============================================================================
 * BOUNDARY TESTING
 * ============================================================================
 *
 * Boundary tests must include:
 *
 *     empty construct sequences;
 *     one construct;
 *     many constructs;
 *     deeply qualified format names;
 *     deeply qualified schema names;
 *     large option lists;
 *     large expressions;
 *     deeply nested types;
 *     large logical metadata structures;
 *     large streaming/chunking expressions.
 *
 * No test may establish a language-level maximum.
 *
 * ============================================================================
 * SCALABILITY TESTING
 * ============================================================================
 *
 * Scalability testing must distinguish:
 *
 *     grammar capacity
 *
 * from:
 *
 *     implementation resource exhaustion.
 *
 * The grammar must remain structurally open-ended.
 *
 * Large inputs may eventually be limited by:
 *
 *     available memory;
 *     parser implementation resources;
 *     compiler policy;
 *     operating-system limits;
 *     downstream storage;
 *     transport capacity.
 *
 * Such limits must not be encoded as grammar constants.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Compatibility responsibility is split:
 *
 *     syntax compatibility
 *         -> grammar/data/serialization.g4
 *
 *     deserialization compatibility façade
 *         -> grammar/data/deserialization.g4
 *
 *     interoperability compatibility boundary
 *         -> this file
 *
 *     schema compatibility
 *         -> semantic/schema subsystem
 *
 *     format compatibility
 *         -> format/provider implementation
 *
 *     ABI compatibility
 *         -> interoperability/abi.g4 and downstream ABI machinery
 *
 *     runtime compatibility
 *         -> runtime implementation
 *
 * This file must not implement compatibility algorithms.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no format enumeration;
 *     no schema enumeration;
 *     no fixed field count;
 *     no fixed option count;
 *     no fixed message count;
 *     no fixed resource count;
 *     no fixed device count;
 *     no fixed memory capacity;
 *     no fixed tensor rank;
 *     no fixed register width;
 *     no fixed network size.
 *
 * Specifically absent as language limits:
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
 * Numeric values appearing in serialized program data remain ordinary
 * program semantics.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * This façade adds only delegation.
 *
 * It must NOT:
 *
 *     duplicate parsing;
 *     reparse source;
 *     copy serialized payloads;
 *     allocate representation buffers;
 *     instantiate codecs;
 *     perform schema resolution;
 *     perform semantic validation;
 *     access external resources.
 *
 * Large serialization payloads are handled by the downstream serialization
 * implementation.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar embeds no Rust.
 *
 * Generated/consuming Rust code must support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     edition 2021
 *
 * and must remain safe Rust.
 *
 * The repository should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * where appropriate.
 *
 * This grammar does not require unsafe operations.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is parser-only.
 * [x] It consumes ZamaniLexer.
 * [x] It imports canonical data serialization grammar.
 * [x] It introduces no serialization syntax of its own.
 * [x] It introduces no deserialization syntax of its own.
 * [x] It introduces no format enumeration.
 * [x] It introduces no schema enumeration.
 * [x] It introduces no second AST.
 * [x] It introduces no semantic model.
 * [x] It introduces no IR.
 * [x] It introduces no codec.
 * [x] It introduces no encoder.
 * [x] It introduces no decoder.
 * [x] It introduces no storage implementation.
 * [x] It introduces no network implementation.
 * [x] It introduces no hardware implementation.
 * [x] It introduces no quantum implementation.
 * [x] It introduces no QEC implementation.
 * [x] It introduces no ZQN implementation.
 * [x] It introduces no routing/scheduling implementation.
 * [x] It imposes no artificial resource limits.
 * [x] It preserves open-world format identity.
 * [x] It preserves open-world schema identity.
 * [x] It preserves source provenance.
 * [x] It preserves deterministic parsing.
 * [x] It preserves POCO-REAF.
 * [x] It requires no unsafe Rust.
 * [x] It has explicit AST integration.
 * [x] It has explicit semantic integration.
 * [x] It has explicit IR integration.
 * [x] It has explicit compiler integration.
 * [x] It has explicit runtime integration.
 * [x] It has explicit interoperability integration.
 * [x] It has explicit testing requirements.
 *
 * Repository-level completion additionally requires the generated parser,
 * frontend AST, semantic analyzer, and conformance tests to agree with the
 * contracts documented above.
 *
 * ============================================================================
 * CANONICAL GRAMMAR FACADE
 * ============================================================================
 */

parser grammar SerializationInterop;

options {
    /*
     * Canonical parser-facing lexical vocabulary.
     *
     * Do not introduce a second lexer vocabulary for interoperability.
     */
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL IMPORT
 * ============================================================================
 *
 * The lowercase grammar name is intentional and matches:
 *
 *     grammar/data/serialization.g4
 *
 * This is a parser-grammar import, not a textual include.
 *
 * ============================================================================
 */

import serialization;


/*
 * ============================================================================
 * PUBLIC INTEROPERABILITY UNIT
 * ============================================================================
 *
 * The canonical `serializationUnit` already owns its complete source-unit
 * boundary and EOF.
 *
 * Therefore this façade delegates directly rather than adding another EOF.
 *
 * This avoids two competing source-unit definitions.
 * ============================================================================
 */

serializationInteroperabilityUnit
    : serializationUnit
    ;


/*
 * ============================================================================
 * CONSTRUCT-LEVEL INTEROPERABILITY FACADE
 * ============================================================================
 *
 * This rule exposes individual canonical serialization/deserialization
 * constructs without redefining their syntax.
 *
 * It is useful to interoperability tooling that works construct-by-construct.
 *
 * ============================================================================
 */

serializationInteroperabilityConstruct
    : serializationConstruct
    ;


/*
 * ============================================================================
 * STATEMENT-LEVEL FACADE
 * ============================================================================
 *
 * Stable interoperability aliases for statement consumers.
 *
 * No new syntax is introduced.
 * ============================================================================
 */

serializationInteroperabilityStatement
    : dataSerializationStmt
    ;


/*
 * ============================================================================
 * EXPRESSION-LEVEL FACADE
 * ============================================================================
 *
 * Stable interoperability aliases for expression consumers.
 *
 * No new syntax is introduced.
 * ============================================================================
 */

serializationInteroperabilityExpression
    : dataSerializationExpression
    ;


/*
 * ============================================================================
 * CONTRACT-LEVEL FACADE
 * ============================================================================
 *
 * Stable interoperability alias for reusable serialization contracts.
 *
 * The canonical contract grammar remains the sole syntax authority.
 * ============================================================================
 */

serializationInteroperabilityContract
    : dataSerializationContract
    ;


/*
 * ============================================================================
 * EXPLICIT SERIALIZATION FACADE
 * ============================================================================
 *
 * These aliases make the two semantic directions explicit to interoperability
 * consumers while continuing to delegate to the canonical grammar.
 * ============================================================================
 */

serializationInteroperabilitySerialize
    : serializationStatement
    ;


serializationInteroperabilityDeserialize
    : deserializationStatement
    ;


/*
 * ============================================================================
 * EXPRESSION-DIRECTION FACADE
 * ============================================================================
 */

serializationInteroperabilitySerializeExpression
    : serializationExpression
    ;


serializationInteroperabilityDeserializeExpression
    : deserializationExpression
    ;


/*
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar is intentionally small in executable grammar surface.
 *
 * Its production-readiness comes from architectural correctness:
 *
 *     ONE canonical serialization syntax
 *                 +
 *     ONE canonical deserialization syntax
 *                 +
 *     ONE canonical data semantic model
 *                 +
 *     ONE canonical downstream representation
 *                 +
 *     stable interoperability aliases
 *                 +
 *     open-world format identity
 *                 +
 *     target-independent resource semantics
 *                 +
 *     deterministic parsing
 *                 +
 *     safe Rust
 *                 =
 *     production serialization interoperability
 *
 * Serialization remains a representation concern.
 *
 * Quantum semantics remain in quantum::ir.
 *
 * Hardware semantics remain in the hardware/HDL subsystem.
 *
 * Resource and capability realization remain downstream.
 *
 * The source program therefore remains portable across:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     quantum-classical systems
 *     HPC systems
 *     distributed systems
 *     cloud systems
 *     future architectures
 *
 * subject to:
 *
 *     program semantics;
 *     explicit requirements;
 *     target capabilities;
 *     available resources;
 *     implementation policy.
 *
 * No physical limitation becomes a language limitation.
 *
 * ============================================================================
 */