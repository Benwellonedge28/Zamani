/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/interoperability/deserialization.g4
 *
 * GRAMMAR
 * -------
 * DeserializationInterop
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
 * This file defines the interoperability-facing deserialization boundary.
 *
 * IMPORTANT:
 *
 * This file is intentionally NOT a second deserialization grammar.
 *
 * Canonical serialization/deserialization syntax is owned by:
 *
 *     grammar/data/serialization.g4
 *
 * The existing:
 *
 *     grammar/data/deserialization.g4
 *
 * is the data-subsystem deserialization compatibility façade.
 *
 * This file is the interoperability-subsystem deserialization façade.
 *
 * Therefore the architecture is:
 *
 *     canonical serialization/deserialization syntax
 *                 |
 *                 v
 *     grammar/data/serialization.g4
 *                 |
 *        +--------+--------+
 *        |                 |
 *        v                 v
 * grammar/data/      this file
 * deserialization.g4
 *        |                 |
 *        +--------+--------+
 *                 |
 *                 v
 *        interoperability consumers
 *
 * No independent deserialization syntax is introduced here.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * The authority hierarchy is:
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
 *             +-----------------------------+
 *             |                             |
 *             v                             v
 * grammar/data/deserialization.g4   interoperability/deserialization.g4
 *             |                             |
 *             +--------------+--------------+
 *                            |
 *                            v
 *                    canonical Zamani AST
 *                            |
 *                            v
 *                    semantic analysis
 *                            |
 *          +-----------------+-----------------+
 *          |                 |                 |
 *          v                 v                 v
 *      classical          quantum           hardware
 *          |                 |                 |
 *          |                 v                 |
 *          |            quantum::ir            |
 *          |                                   |
 *          +-----------------+-----------------+
 *                            |
 *                            v
 *                    canonical semantic IR
 *                            |
 *                            v
 *                compiler / linker / runtime
 *
 * This file MUST NOT become an authority above or beside the canonical
 * serialization grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   1. Interoperability parser identity.
 *   2. Import of the canonical serialization grammar.
 *   3. Stable interoperability deserialization entry points.
 *   4. Construct-level interoperability aliases.
 *   5. Statement-level interoperability aliases.
 *   6. Expression-level interoperability aliases.
 *   7. Contract-level interoperability aliases.
 *   8. Source-format interoperability boundary naming.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - deserialization syntax;
 *   - serialization syntax;
 *   - serialization contracts;
 *   - format definitions;
 *   - schema definitions;
 *   - type definitions;
 *   - expressions;
 *   - identifiers;
 *   - qualified names;
 *   - codecs;
 *   - decoders;
 *   - schema resolution;
 *   - schema evolution;
 *   - validation algorithms;
 *   - compatibility algorithms;
 *   - storage;
 *   - filesystem access;
 *   - networking;
 *   - transport;
 *   - memory allocation;
 *   - resource discovery;
 *   - capability discovery;
 *   - hardware discovery;
 *   - device selection;
 *   - target selection;
 *   - runtime execution;
 *   - AST implementation;
 *   - semantic analysis;
 *   - canonical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - HAL;
 *   - backend implementation.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical syntax owner for serialization and
 * deserialization.
 *
 * That owner is:
 *
 *     grammar/data/serialization.g4
 *
 * Consequently this file MUST NOT independently define rules such as:
 *
 *     deserializationStatement
 *     deserializationExpression
 *     serializationStatement
 *     serializationExpression
 *     serializationContractDeclaration
 *
 * The names above belong to the imported canonical grammar.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Interoperability tooling sometimes needs a stable grammar boundary for
 * deserialization without depending directly on the internal organization of
 * the data grammar.
 *
 * This façade provides that boundary.
 *
 * It allows interoperability consumers to work with:
 *
 *     deserialization statements
 *     deserialization expressions
 *     serialization contracts
 *     format references
 *     schema references
 *     deserialization options
 *
 * without taking ownership of their syntax.
 *
 * If the canonical serialization grammar changes internally while preserving
 * the public semantic contract, interoperability consumers can continue using
 * this façade.
 *
 * ============================================================================
 * CANONICAL DATA BOUNDARY
 * ============================================================================
 *
 * The canonical data grammar owns:
 *
 *     serialization
 *     deserialization
 *     serialization expressions
 *     deserialization expressions
 *     serialization contracts
 *     format references
 *     schema references
 *     compatibility
 *     evolution
 *     canonicalization
 *     framing
 *     compression
 *     integrity
 *     encoding
 *     byte order
 *     null policies
 *     missing-value policies
 *     unknown-field policies
 *     duplicate-field policies
 *     metadata
 *     streaming
 *     chunking
 *     validation
 *     capabilities
 *     requirements
 *     preferences
 *     hints
 *     extensions
 *
 * This file delegates all of those concepts.
 *
 * ============================================================================
 * OPEN-WORLD FORMAT MODEL
 * ============================================================================
 *
 * This file MUST NOT enumerate a closed universe of formats.
 *
 * It MUST NOT define a grammar-level enumeration such as:
 *
 *     JSON
 *     XML
 *     CBOR
 *     Protobuf
 *     MessagePack
 *     Avro
 *     Parquet
 *     Arrow
 *
 * as the complete set of supported formats.
 *
 * Format identity is semantic data.
 *
 * Examples may include:
 *
 *     json
 *     cbor
 *     protobuf
 *     telemetry::binary
 *     vendor::format
 *     future::representation
 *
 * The actual availability of a format is determined downstream by semantic
 * analysis, providers, codecs, compatibility rules, and target environment.
 *
 * Future formats MUST NOT require modification of this façade merely to make
 * their names syntactically representable.
 *
 * ============================================================================
 * OPEN-WORLD SCHEMA MODEL
 * ============================================================================
 *
 * Schema identities are likewise open-world.
 *
 * This file MUST NOT enumerate:
 *
 *     User
 *     Event
 *     Tensor
 *     QuantumState
 *     HardwareDescription
 *
 * as a closed schema universe.
 *
 * Schemas remain owned by the data/type/schema semantic systems.
 *
 * ============================================================================
 * TYPE SYSTEM INTEGRATION
 * ============================================================================
 *
 * Deserialization may identify a target Zamani type.
 *
 * Conceptually:
 *
 *     deserialize payload using json as User;
 *
 * The type:
 *
 *     User
 *
 * remains owned by the canonical Zamani type system.
 *
 * This file MUST NOT introduce:
 *
 *     SerializationType
 *     DeserializationType
 *     JsonType
 *     BinaryType
 *     ForeignSerializationType
 *
 * or any second type universe.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This façade introduces NO new AST family.
 *
 * The expected path is:
 *
 *     interoperability deserialization entry
 *                 |
 *                 v
 *     canonical deserialization construct
 *                 |
 *                 v
 *     existing frontend AST
 *                 |
 *                 v
 *     semantic analysis
 *
 * The interoperability façade must not force the frontend AST to contain
 * interoperability-specific duplicate representations.
 *
 * Source spans from the delegated parser context MUST remain available to
 * the frontend and diagnostic systems.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *   - whether the referenced format exists;
 *   - whether the schema exists;
 *   - whether the target type is valid;
 *   - whether the source representation is compatible;
 *   - whether conversions are legal;
 *   - whether compatibility requirements are satisfied;
 *   - whether required capabilities exist;
 *   - whether resource requirements can be satisfied;
 *   - whether security policy permits the operation;
 *   - whether integrity requirements are satisfied;
 *   - whether the operation is permitted in its effect context.
 *
 * Parsing alone establishes none of these facts.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * It MUST NOT introduce:
 *
 *     DeserializationIR
 *     DeserializationInteropIR
 *     SerializationIR
 *     DataInteropIR
 *     QuantumSerializationIR
 *     HardwareSerializationIR
 *
 * The semantic operation is lowered through the repository's canonical
 * semantic/IR architecture.
 *
 * For quantum-related values, the canonical quantum boundary remains:
 *
 *     semantic analysis
 *             |
 *             v
 *         quantum::ir
 *
 * This file MUST NOT create another quantum representation.
 *
 * ============================================================================
 * SERIALIZATION / DESERIALIZATION DIRECTION
 * ============================================================================
 *
 * Deserialization is a representation-to-semantic-value operation.
 *
 * Conceptually:
 *
 *     external representation
 *             |
 *             v
 *       deserialization
 *             |
 *             v
 *       Zamani semantic value
 *
 * Serialization is the opposite direction:
 *
 *     Zamani semantic value
 *             |
 *             v
 *        serialization
 *             |
 *             v
 *     external representation
 *
 * This façade only provides the interoperability-facing entry points for
 * deserialization.
 *
 * It does not implement the conversion.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Deserialization is a trust boundary.
 *
 * The following distinctions MUST remain explicit:
 *
 *     syntactic acceptance
 *             !=
 *     structural validation
 *             !=
 *     schema validation
 *             !=
 *     semantic validation
 *             !=
 *     authorization
 *             !=
 *     trusted execution
 *
 * In particular:
 *
 *     deserialization != code execution
 *
 * This grammar performs none of:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     dynamic loading
 *     code execution
 *     codec execution
 *     cryptographic execution
 *
 * Untrusted serialized data remains untrusted until the appropriate semantic,
 * validation, integrity, and security layers accept it.
 *
 * ============================================================================
 * RESOURCE-EXHAUSTION CONTRACT
 * ============================================================================
 *
 * Deserialization implementations may require resources for:
 *
 *     memory
 *     storage
 *     CPU
 *     accelerator
 *     network
 *     recursion
 *     buffering
 *     decompression
 *     schema processing
 *
 * Such implementation/security controls MUST NOT become artificial grammar
 * limits.
 *
 * For example, the grammar must not define:
 *
 *     MAX_FIELDS
 *     MAX_BYTES
 *     MAX_DEPTH
 *     MAX_RECORDS
 *     MAX_ELEMENTS
 *     MAX_MESSAGES
 *
 * as universal language restrictions.
 *
 * An implementation may impose operational safety budgets where necessary.
 * Those budgets are environment/runtime/security policies, not language
 * semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This façade preserves:
 *
 *     Program_Once
 *          |
 *          v
 *     Compile_Once
 *          |
 *          v
 *     Run_Everywhere
 *          |
 *          v
 *     Run_Anywhere
 *          |
 *          v
 *     Run_Forever
 *
 * Deserialization syntax therefore MUST remain independent of:
 *
 *     CPU model
 *     GPU model
 *     FPGA model
 *     ASIC model
 *     QPU model
 *     accelerator model
 *     operating system
 *     pointer width
 *     register width
 *     memory capacity
 *     storage capacity
 *     device count
 *     node count
 *     network topology
 *     physical address
 *     physical qubit
 *     machine identifier
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * The following MUST NOT appear as universal language-level capacity limits:
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
 * This file additionally MUST NOT establish fixed maxima for:
 *
 *     formats
 *     schemas
 *     fields
 *     records
 *     collection elements
 *     message elements
 *     options
 *     metadata entries
 *     qualified-name components
 *     nested representation structures
 *
 * Repetition in the imported grammar remains open-ended.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Deserialization interoperability must preserve the distinction between:
 *
 *     requirement
 *         A condition that must be satisfied.
 *
 *     capability
 *         A property supplied by a realization.
 *
 *     constraint
 *         A restriction on valid realization.
 *
 *     preference
 *         Non-binding optimization guidance.
 *
 *     hint
 *         Non-binding implementation information.
 *
 *     realization
 *         A downstream implementation decision.
 *
 * For example:
 *
 *     requires capability("data.decode")
 *
 * describes a capability requirement.
 *
 * It does not select:
 *
 *     a decoder;
 *     a library;
 *     a CPU;
 *     a GPU;
 *     a device;
 *     a process;
 *     a runtime.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Deserialization may produce semantic values used by classical computation,
 * including:
 *
 *     scalars
 *     integers
 *     floating-point values
 *     vectors
 *     matrices
 *     tensors
 *     records
 *     collections
 *     streams
 *     application-defined types
 *
 * Their types remain owned by the canonical Zamani type/data systems.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Deserialized information may participate in quantum or hybrid computation
 * where the semantic type system permits it.
 *
 * This façade MUST NOT:
 *
 *     allocate qubits;
 *     select physical qubits;
 *     select a QPU;
 *     select a gate set;
 *     route quantum operations;
 *     schedule quantum operations;
 *     perform calibration;
 *     implement QEC;
 *     implement ZQN;
 *     construct quantum::ir.
 *
 * The canonical quantum architecture remains:
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
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Deserialized values may be used by:
 *
 *     HDL
 *     hardware
 *     accelerator
 *     co-design
 *     simulation
 *     verification
 *     deployment
 *
 * This file MUST NOT define:
 *
 *     register widths
 *     bus widths
 *     memory-bank counts
 *     physical addresses
 *     FPGA resources
 *     ASIC structures
 *     physical topology
 *     device identifiers
 *     clock domains
 *     pipeline depths
 *
 * Those are semantic and/or downstream implementation concerns.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Deserialization may provide:
 *
 *     tensors
 *     datasets
 *     models
 *     model metadata
 *     checkpoints
 *     inference results
 *     training artifacts
 *     streams
 *     provenance
 *
 * Framework-specific representations remain outside this grammar.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Deserialization may participate in:
 *
 *     message exchange
 *     persistent state
 *     snapshots
 *     checkpoints
 *     replication
 *     service interfaces
 *     distributed computation
 *
 * This file does not own:
 *
 *     transport
 *     network topology
 *     node selection
 *     replica placement
 *     retry algorithms
 *     scheduling
 *     distributed consensus
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * This façade is designed to coexist with:
 *
 *     grammar/interoperability/serialization.g4
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/foreign-functions.g4
 *     grammar/interoperability/abi.g4
 *     grammar/interoperability/c.g4
 *     grammar/interoperability/cpp.g4
 *     grammar/interoperability/python.g4
 *     grammar/interoperability/rust.g4
 *     grammar/interoperability/qasm.g4
 *     grammar/interoperability/qir.g4
 *     grammar/interoperability/hdl.g4
 *
 * These boundaries MUST remain complementary.
 *
 * This file does not redefine FFI, ABI, HDL, OpenQASM, QIR, or foreign
 * language syntax.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     canonical lexer
 *             |
 *             v
 *     canonical core grammar
 *             |
 *             v
 *     canonical data grammar
 *             |
 *             v
 *     this interoperability façade
 *             |
 *             v
 *     interoperability consumers
 *
 * NOT:
 *
 *     interoperability
 *          |
 *          +--> canonical lexer
 *          +--> runtime
 *          +--> filesystem
 *          +--> hardware
 *          +--> network
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar must parse deterministically for identical:
 *
 *     source text
 *     canonical token vocabulary
 *     grammar version
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend upon:
 *
 *     CPU availability
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     target device
 *
 * ============================================================================
 * SOURCE PROVENANCE
 * ============================================================================
 *
 * The façade MUST preserve the source location of the delegated canonical
 * construct.
 *
 * Source provenance is required for:
 *
 *     diagnostics
 *     IDE navigation
 *     semantic errors
 *     schema errors
 *     compatibility errors
 *     security diagnostics
 *     source maps
 *     audit trails
 *     tooling
 *
 * The façade MUST NOT introduce a wrapper that causes the underlying source
 * span to become inaccessible to downstream consumers.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * This façade adds aliases and delegation only.
 *
 * It MUST NOT:
 *
 *     reparse source;
 *     duplicate parsing;
 *     duplicate AST construction;
 *     decode external data;
 *     load schemas;
 *     resolve files;
 *     access networks;
 *     perform resource discovery;
 *     instantiate codecs.
 *
 * Performance remains governed by the canonical grammar and consuming
 * implementation.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing interoperability consumers may depend on:
 *
 *     grammar/interoperability/deserialization.g4
 *
 * This file provides a stable named boundary without copying the canonical
 * implementation.
 *
 * Changes to canonical deserialization syntax belong in:
 *
 *     grammar/data/serialization.g4
 *
 * Changes to the interoperability boundary itself belong here.
 *
 * Compatibility policy belongs to:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests for this façade verify:
 *
 *     delegation;
 *     parser composition;
 *     stable rule names;
 *     source-unit behavior;
 *     construct-level behavior;
 *     statement-level behavior;
 *     expression-level behavior;
 *     contract-level behavior;
 *     interoperability compatibility.
 *
 * The complete serialization/deserialization semantic test suite remains
 * owned by the canonical data subsystem.
 *
 * ============================================================================
 * REQUIRED POSITIVE CASES
 * ============================================================================
 *
 * These examples are semantic examples of constructs delegated to the
 * canonical grammar. They are not independently defined here.
 *
 *     deserialize payload using json as User;
 *
 *     deserialize payload using cbor as telemetry::Event;
 *
 *     deserialize payload using vendor::format as Data;
 *
 *     deserialize source using future::representation as Value;
 *
 * Expression forms, where supported by the canonical grammar:
 *
 *     deserialize(payload) using json as User
 *
 *     deserialize(stream) using vendor::format as telemetry::Event
 *
 * ============================================================================
 * REQUIRED NEGATIVE CASES
 * ============================================================================
 *
 * Interoperability conformance tests MUST include:
 *
 *     malformed deserialization constructs;
 *     missing source;
 *     missing format;
 *     missing target type;
 *     malformed format reference;
 *     malformed schema reference;
 *     malformed options;
 *     malformed expressions;
 *     invalid delimiter structure.
 *
 * Exact diagnostics remain owned by the canonical diagnostics architecture.
 *
 * ============================================================================
 * REQUIRED BOUNDARY CASES
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     empty isolated unit;
 *     one construct;
 *     many constructs;
 *     deeply qualified format names;
 *     deeply qualified schema names;
 *     large option sequences;
 *     large expressions;
 *     deeply composed types;
 *     large logical metadata structures;
 *     nested representation intent;
 *     streaming/chunking configuration.
 *
 * No test may establish an artificial universal maximum.
 *
 * ============================================================================
 * REQUIRED SCALABILITY CASES
 * ============================================================================
 *
 * Scalability testing must distinguish:
 *
 *     language capacity
 *
 * from:
 *
 *     implementation resource availability.
 *
 * The grammar must remain open-ended with respect to:
 *
 *     number of constructs;
 *     option count;
 *     schema complexity;
 *     type complexity;
 *     qualified-name depth;
 *     metadata size;
 *     logical stream configuration.
 *
 * Actual exhaustion of:
 *
 *     memory;
 *     compilation time;
 *     storage;
 *     transport capacity;
 *     runtime resources
 *
 * is an implementation/environment condition, not a grammar-level limit.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Equivalent input must produce equivalent parse structures.
 *
 * The result must not depend on:
 *
 *     machine size;
 *     hardware availability;
 *     network availability;
 *     filesystem state;
 *     runtime state;
 *     target provider.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no embedded Rust actions.
 *
 * The consuming implementation targets:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust edition 2021
 *
 * Zamani-owned Rust MUST remain safe Rust.
 *
 * No "unsafe" implementation is required by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is parser-only.
 * [x] It uses the canonical Zamani lexical vocabulary.
 * [x] It imports the canonical serialization grammar.
 * [x] It introduces no independent deserialization syntax.
 * [x] It introduces no serialization syntax.
 * [x] It introduces no format enumeration.
 * [x] It introduces no schema enumeration.
 * [x] It introduces no second type system.
 * [x] It introduces no second AST.
 * [x] It introduces no semantic model.
 * [x] It introduces no IR.
 * [x] It introduces no decoder.
 * [x] It introduces no codec.
 * [x] It introduces no storage implementation.
 * [x] It introduces no network implementation.
 * [x] It introduces no hardware implementation.
 * [x] It introduces no quantum implementation.
 * [x] It introduces no QEC implementation.
 * [x] It introduces no ZQN implementation.
 * [x] It introduces no routing implementation.
 * [x] It introduces no scheduling implementation.
 * [x] It introduces no artificial resource limits.
 * [x] It preserves open-world format identity.
 * [x] It preserves open-world schema identity.
 * [x] It preserves source provenance.
 * [x] It preserves deterministic parsing.
 * [x] It preserves POCO-REAF.
 * [x] It requires no unsafe Rust.
 * [x] It defines explicit interoperability entry points.
 * [x] It keeps canonical syntax ownership in grammar/data/serialization.g4.
 *
 * Repository-level completion additionally requires:
 *
 *     canonical grammar validation;
 *     frontend parser conformance;
 *     AST conformance;
 *     semantic conformance;
 *     interoperability conformance tests;
 *     compatibility tests.
 *
 * ============================================================================
 * CANONICAL PARSER GRAMMAR
 * ============================================================================
 *
 * This grammar is deliberately a façade.
 *
 * The imported grammar owns:
 *
 *     serializationUnit
 *     serializationConstruct
 *     serializationStatement
 *     deserializationStatement
 *     serializationExpression
 *     deserializationExpression
 *     serializationContractDeclaration
 *     dataSerializationStmt
 *     dataSerializationExpression
 *     dataSerializationContract
 *
 * This grammar only provides stable interoperability-facing aliases.
 *
 * ============================================================================
 */

parser grammar DeserializationInterop;

options {
    /*
     * The canonical Zamani lexer vocabulary is shared by all interoperability
     * parser façades.
     */
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL IMPORT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `serialization` is the canonical parser grammar defined by:
 *
 *     grammar/data/serialization.g4
 *
 * This is an ANTLR parser-grammar import.
 *
 * It is NOT textual inclusion and it does NOT create a second grammar.
 * ============================================================================
 */

import serialization;


/*
 * ============================================================================
 * PUBLIC DESERIALIZATION UNIT
 * ============================================================================
 *
 * This is intentionally NOT:
 *
 *     deserializeUnit
 *         : ...
 *
 * with independently copied syntax.
 *
 * It delegates to the canonical source-unit grammar where appropriate.
 *
 * The canonical serialization grammar owns the source-unit boundary and EOF.
 * ============================================================================
 */

deserializationInteroperabilityUnit
    : serializationUnit
    ;


/*
 * ============================================================================
 * CONSTRUCT-LEVEL FACADE
 * ============================================================================
 *
 * Provides a stable interoperability-facing construct boundary.
 *
 * The syntax remains owned by the canonical grammar.
 * ============================================================================
 */

deserializationInteroperabilityConstruct
    : serializationConstruct
    ;


/*
 * ============================================================================
 * DESERIALIZATION-ONLY CONSTRUCT
 * ============================================================================
 *
 * Explicit deserialization boundary for interoperability consumers.
 * ============================================================================
 */

deserializationInteroperability
    : deserializationStatement
    | deserializationExpression
    ;


/*
 * ============================================================================
 * STATEMENT-LEVEL FACADE
 * ============================================================================
 */

deserializationInteroperabilityStatement
    : deserializationStatement
    ;


/*
 * ============================================================================
 * EXPRESSION-LEVEL FACADE
 * ============================================================================
 */

deserializationInteroperabilityExpression
    : deserializationExpression
    ;


/*
 * ============================================================================
 * DATA-FACING COMPATIBILITY ALIASES
 * ============================================================================
 *
 * These aliases intentionally converge with the existing data façade.
 *
 * No syntax is defined here.
 * ============================================================================
 */

dataDeserializationInteroperabilityStatement
    : dataDeserializationStmt
    ;


dataDeserializationInteroperabilityExpression
    : dataDeserializationExpression
    ;


/*
 * ============================================================================
 * CONTRACT-LEVEL FACADE
 * ============================================================================
 *
 * A serialization contract can contain deserialization policy and therefore
 * belongs to the canonical serialization grammar rather than to a separate
 * deserialization grammar.
 * ============================================================================
 */

deserializationInteroperabilityContract
    : dataSerializationContract
    ;


/*
 * ============================================================================
 * EXPLICIT SERIALIZATION-CONTRACT ALIAS
 * ============================================================================
 *
 * This name makes the relationship clear to interoperability tooling while
 * preserving canonical ownership.
 * ============================================================================
 */

deserializationInteroperabilitySerializationContract
    : serializationContractDeclaration
    ;


/*
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * The production invariant is:
 *
 *     ONE canonical data syntax owner
 *             +
 *     ONE canonical serialization/deserialization semantic family
 *             +
 *     ONE canonical AST
 *             +
 *     ONE canonical semantic model
 *             +
 *     stable interoperability aliases
 *             +
 *     open-world format identity
 *             +
 *     open-world schema identity
 *             +
 *     target-independent resource semantics
 *             +
 *     deterministic parsing
 *             +
 *     safe Rust
 *             =
 *     production deserialization interoperability
 *
 * Deserialization remains a data/representation concern.
 *
 * Quantum semantics remain in:
 *
 *     quantum::ir
 *
 * Hardware semantics remain in:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * Resource and capability realization remain downstream.
 *
 * The compiler/runtime determines the concrete realization.
 *
 * No physical machine limitation becomes a Zamani language limitation.
 *
 * ============================================================================
 */