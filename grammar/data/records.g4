/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/data/records.g4
 *
 * Grammar:
 *     ZamaniDataRecords
 *
 * Status:
 *     PRODUCTION / DATA-DOMAIN RECORD ADAPTER
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust and requires no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the DATA-DOMAIN integration point for logical records.
 *
 * IMPORTANT:
 *
 * This file does NOT define a second record language.
 *
 * The canonical source-level record syntax is owned by:
 *
 *     grammar/declarations/records.g4
 *
 * That grammar owns:
 *
 *     recordDeclaration
 *
 * This file adapts that canonical declaration into the data-domain grammar
 * through:
 *
 *     dataRecordDeclaration
 *
 * Therefore:
 *
 *     record syntax
 *          |
 *          v
 *     declarations/records.g4
 *          |
 *          v
 *     data/records.g4
 *          |
 *          v
 *     data dispatcher
 *
 * There is exactly ONE concrete record syntax.
 *
 * ============================================================================
 * WHY THIS FILE IS AN ADAPTER
 * ============================================================================
 *
 * A record is simultaneously:
 *
 *     - a language declaration;
 *     - a logical data structure;
 *     - a possible schema;
 *     - a possible classical value;
 *     * a possible quantum-containing value;
 *     * a possible hybrid value;
 *     * a possible distributed value;
 *     * a possible AI/data value;
 *     * a possible HDL/software co-design value.
 *
 * The declaration grammar owns the declaration semantics.
 *
 * The data grammar owns the fact that records participate in the data domain.
 *
 * Keeping these responsibilities separate prevents:
 *
 *     declarations/records.g4
 *     data/records.g4
 *
 * from evolving into two incompatible record syntaxes.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the data-domain record integration rule;
 *     - the public `dataRecordDeclaration` adapter;
 *     - the relationship between data-domain record syntax and the canonical
 *       declaration record syntax;
 *     - data-domain documentation of the record boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - record declaration syntax;
 *     - record names;
 *     - record fields;
 *     - record generic parameters;
 *     - record where clauses;
 *     - record inheritance;
 *     - attributes;
 *     - visibility;
 *     - modifiers;
 *     - identifiers;
 *     - qualified names;
 *     - type expressions;
 *     - expressions;
 *     - AST definitions;
 *     - semantic validation;
 *     - memory layout;
 *     - serialization;
 *     - storage;
 *     - database behavior;
 *     - networking;
 *     - distributed placement;
 *     - hardware placement;
 *     - quantum allocation;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - scheduling;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY INVARIANT
 * ============================================================================
 *
 * There MUST be exactly one concrete source-level record declaration grammar.
 *
 * Canonical owner:
 *
 *     grammar/declarations/records.g4
 *
 * Canonical rule:
 *
 *     recordDeclaration
 *
 * This file MUST NOT redefine:
 *
 *     recordDeclaration
 *     recordBody
 *     recordField
 *     recordFieldList
 *     recordFieldSeparator
 *     recordInheritanceClause
 *
 * Doing so would create competing record languages.
 *
 * ============================================================================
 * CANONICAL PIPELINE
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
 *     declarations
 *          |
 *          v
 *     grammar/declarations/records.g4
 *          |
 *          v
 *     recordDeclaration
 *          |
 *          v
 *     grammar/data/records.g4
 *          |
 *          v
 *     dataRecordDeclaration
 *          |
 *          v
 *     Data dispatcher
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
 *          v
 *     canonical semantic representation
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *     classical             quantum::ir          HDL/hardware
 *     semantics              consumers             semantics
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         canonical IR/lowering
 *                                 |
 *                      optimization / scheduling
 *                                 |
 *                      resource / capability resolution
 *                                 |
 *                           target realization
 *
 * ============================================================================
 * ROOT COMPOSITION
 * ============================================================================
 *
 * The repository's canonical parser hierarchy is:
 *
 *     grammar/Zamani.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             +--> Declarations
 *             +--> Data
 *             +--> Types
 *             +--> Expressions
 *             +--> other domains
 *
 * `ZamaniParser.g4` remains the universal parser composition root.
 *
 * This file is NOT a program root.
 *
 * This file does NOT consume EOF.
 *
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore this grammar consumes:
 *
 *     ZamaniLexer
 *
 * and MUST NOT introduce:
 *
 *     Zamani
 *     ZamaniTokens
 *     custom identifiers
 *     custom punctuation
 *     local keyword definitions.
 *
 * ============================================================================
 * DECLARATION GRAMMAR DEPENDENCY
 * ============================================================================
 *
 * The canonical declaration record grammar is:
 *
 *     ZamaniDeclarationRecords
 *
 * and its public concrete rule is:
 *
 *     recordDeclaration
 *
 * This adapter imports that grammar.
 *
 * The imported declaration grammar remains responsible for all concrete
 * declaration syntax.
 *
 * ============================================================================
 * DATA-DOMAIN ADAPTER
 * ============================================================================
 *
 * The sole public rule of this file is:
 *
 *     dataRecordDeclaration
 *
 * It delegates directly to:
 *
 *     recordDeclaration
 *
 * No syntax is added.
 *
 * This means that the following are identical language constructs regardless
 * of whether they enter through the declaration or data composition path:
 *
 *     record User {
 *         id: int;
 *     }
 *
 *     record Measurement<T> {
 *         value: T;
 *     }
 *
 * Data-domain consumers therefore never need to reconstruct record syntax
 * themselves.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This adapter does not create an AST node.
 *
 * The AST mapping is inherited from the canonical declaration record grammar.
 *
 * Conceptually:
 *
 *     dataRecordDeclaration
 *          |
 *          v
 *     recordDeclaration
 *          |
 *          v
 *     canonical record AST
 *          |
 *          v
 *     semantic record/data model
 *
 * The adapter MUST NOT introduce:
 *
 *     DataRecordNode
 *     RecordDataNode
 *     DataRecordAst
 *     DataRecordIR
 *
 * merely because the declaration was encountered through the data domain.
 *
 * The same source construct must have one canonical AST identity.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This file performs no semantic validation.
 *
 * Semantic analysis owns:
 *
 *     - record-name resolution;
 *     - field-name uniqueness;
 *     - generic parameter validation;
 *     - where-clause validation;
 *     - inheritance validation;
 *     - recursive-type validation;
 *     - visibility;
 *     - type checking;
 *     - ownership;
 *     - effects;
 *     - capability requirements;
 *     - resource requirements;
 *     - portability;
 *     - target compatibility.
 *
 * The data domain may subsequently interpret a validated record as:
 *
 *     - logical data;
 *     - schema;
 *     - classical aggregate;
 *     - tensor-containing aggregate;
 *     - quantum-containing aggregate;
 *     - distributed value;
 *     - serialized value;
 *     - hardware/software co-design data.
 *
 * None of those interpretations changes the source grammar.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Record field types remain owned by the canonical type system.
 *
 * This adapter therefore does not reference or duplicate:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *     generic type syntax
 *     array syntax
 *     tensor syntax
 *     quantum type syntax
 *     resource type syntax
 *
 * A record can therefore evolve automatically with the canonical type system.
 *
 * Examples of semantically valid possibilities include:
 *
 *     record Classical<T> {
 *         value: T;
 *     }
 *
 *     record QuantumData {
 *         state: Qubit;
 *     }
 *
 *     record TensorData<T> {
 *         value: Tensor<T>;
 *     }
 *
 *     record Hybrid<T> {
 *         classical: T;
 *         quantum: Qubit;
 *     }
 *
 * The exact validity of these types belongs to semantic/type analysis.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Records may contain quantum-domain types.
 *
 * This file does NOT:
 *
 *     - allocate qubits;
 *     - enumerate physical qubits;
 *     - select a QPU;
 *     - select a gate set;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform calibration;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * The canonical quantum path remains:
 *
 *     record syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum analysis
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the single canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Records may contain:
 *
 *     scalars
 *     integers
 *     floating-point values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     numerical values
 *     application-defined types
 *
 * The data grammar does not impose widths, dimensions, ranks, or storage
 * layouts.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Records may be used as logical data structures in hardware/software
 * co-design.
 *
 * This adapter does not define:
 *
 *     wire widths
 *     register widths
 *     bus widths
 *     physical addresses
 *     memory banks
 *     FPGA resources
 *     ASIC structures
 *     clock domains
 *     pipeline depth.
 *
 * Those are owned by HDL/hardware semantics and downstream lowering.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Records can be transported, replicated, partitioned, persisted, or
 * processed across distributed systems.
 *
 * This grammar does not specify:
 *
 *     node count
 *     partition count
 *     replica count
 *     network topology
 *     placement
 *     provider
 *     storage backend.
 *
 * Those decisions are downstream.
 *
 * ============================================================================
 * SERIALIZATION INTEGRATION
 * ============================================================================
 *
 * Record structure may be consumed by:
 *
 *     grammar/data/serialization.g4
 *
 * but this adapter does not define a wire representation.
 *
 * It does not define:
 *
 *     JSON
 *     CBOR
 *     binary encoding
 *     byte order
 *     padding
 *     alignment
 *     ABI layout
 *     physical offsets.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * A logical record is not a memory layout.
 *
 * This grammar does not define:
 *
 *     sizeof(record)
 *     alignment
 *     packing
 *     addresses
 *     cache placement
 *     NUMA placement
 *     accelerator memory placement
 *     register allocation.
 *
 * Such decisions are made after semantic analysis.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This adapter preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * because records describe logical data rather than physical realization.
 *
 * A record may be realized on:
 *
 *     tiny embedded targets
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     distributed systems
 *     HPC systems
 *     quantum-classical systems
 *     future computational substrates
 *
 * subject to semantic compatibility and available resources.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file introduces no finite language-level limits.
 *
 * It contains no limits for:
 *
 *     MAX_RECORDS
 *     MAX_FIELDS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_RECORD_SIZE
 *     MAX_NESTING
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Cardinality is inherited from the canonical declaration grammar and
 * therefore remains unbounded at the language level.
 *
 * Practical limits are implementation/resource constraints and must never be
 * converted into language semantics.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A record declaration itself does not select hardware.
 *
 * Source-level requirements may be expressed elsewhere through the canonical
 * resource/capability system, for example:
 *
 *     requires capability("tensor.compute")
 *
 *     requires capability("quantum.measurement")
 *
 *     requires memory >= required_memory
 *
 *     requires qubits >= required_qubits
 *
 * Those expressions describe requirements.
 *
 * They do not establish universal compiler limits.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This adapter is deterministic.
 *
 * Given the same:
 *
 *     source
 *     language version
 *     lexer vocabulary
 *     parser composition
 *
 * it produces the same parse structure.
 *
 * It must not depend on:
 *
 *     hardware
 *     runtime state
 *     network state
 *     filesystem state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     target availability.
 *
 * ============================================================================
 * SOURCE ORDER / SOURCE SPANS
 * ============================================================================
 *
 * The canonical declaration grammar owns source ordering.
 *
 * This adapter must preserve:
 *
 *     - record declaration order;
 *     - field order;
 *     - generic parameter order;
 *     - inheritance order;
 *     - source spans.
 *
 * No sorting, normalization, deduplication, or target-specific rewriting
 * occurs here.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the canonical declaration record grammar.
 *
 * Examples include malformed:
 *
 *     record declarations;
 *     generic parameters;
 *     where clauses;
 *     inheritance;
 *     record bodies;
 *     fields;
 *     field types.
 *
 * Semantic errors belong downstream.
 *
 * This adapter must not duplicate diagnostics for errors already owned by
 * `recordDeclaration`.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Record syntax versioning belongs to:
 *
 *     grammar/compatibility/
 *     grammar/specification/
 *     grammar/spec/
 *
 * This file does not introduce an alternative legacy record syntax.
 *
 * If record syntax changes:
 *
 *     canonical declaration grammar
 *          |
 *          v
 *     compatibility policy
 *          |
 *          v
 *     data adapter
 *
 * The adapter remains a stable semantic boundary.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     hardware discovery
 *     environment inspection
 *     credential access
 *     runtime execution.
 *
 * No embedded actions or semantic predicates are used.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * The consuming Zamani implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This adapter adds one delegation layer only.
 *
 * It must not:
 *
 *     - recursively reparse records;
 *     - scan source text;
 *     - construct duplicate AST structures;
 *     - perform semantic lookups;
 *     - inspect hardware;
 *     - perform runtime discovery.
 *
 * Complexity is therefore inherited from the canonical record grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Required repository relationship:
 *
 *     grammar/declarations/records.g4
 *             |
 *             | recordDeclaration
 *             v
 *     grammar/data/records.g4
 *             |
 *             | dataRecordDeclaration
 *             v
 *     grammar/data/data.g4
 *             |
 *             | dataDeclaration
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *     grammar/Zamani.g4
 *
 * The data dispatcher MUST delegate its record alternative to:
 *
 *     dataRecordDeclaration
 *
 * It must not retain an independent implementation such as:
 *
 *     dataRecordDecl
 *
 * with another record grammar.
 *
 * ============================================================================
 * REQUIRED DATA-DISPATCHER INTEGRATION
 * ============================================================================
 *
 * `grammar/data/data.g4` currently contains its own:
 *
 *     dataRecordDecl
 *
 * That rule is a competing implementation and should be replaced by:
 *
 *     dataRecordDeclaration
 *
 * supplied by this adapter.
 *
 * The resulting data declaration dispatch should conceptually be:
 *
 *     dataDeclaration
 *         : dataSchemaDecl
 *         | dataRecordDeclaration
 *         | dataCollectionDecl
 *         | ...
 *         ;
 *
 * The old `dataRecordDecl` implementation should not remain as another
 * record syntax authority.
 *
 * ============================================================================
 * REQUIRED DECLARATION-DISPATCHER INTEGRATION
 * ============================================================================
 *
 * `grammar/declarations/declarations.g4` remains the universal declaration
 * dispatcher.
 *
 * The canonical record declaration remains owned by:
 *
 *     grammar/declarations/records.g4
 *
 * The data adapter must not be added as a second universal declaration
 * alternative.
 *
 * One source declaration -> one canonical declaration AST.
 *
 * ============================================================================
 * NO AST REWORK REQUIRED
 * ============================================================================
 *
 * Because this file delegates to the existing canonical record declaration,
 * adding or changing data-domain consumers does not require modifying this
 * adapter's syntax.
 *
 * Type changes remain owned by the type system.
 *
 * Record semantic changes remain owned by declarations/records.g4 and the
 * semantic layer.
 *
 * Serialization changes remain owned by serialization.g4.
 *
 * Distributed changes remain owned by distributed/resource layers.
 *
 * Quantum changes remain owned by quantum/type/IR layers.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file's direct parser tests should verify delegation rather than
 * duplicate the complete record test suite.
 *
 * REQUIRED POSITIVE CASES:
 *
 *     record Empty {}
 *
 *     record User {
 *         id: int;
 *     }
 *
 *     record User {
 *         id: int,
 *         name: string,
 *     }
 *
 *     record Measurement<T> {
 *         value: T;
 *     }
 *
 *     record Derived extends Base {
 *         value: int;
 *     }
 *
 * REQUIRED STRUCTURAL CASES:
 *
 *     empty record
 *     one field
 *     multiple fields
 *     comma-separated fields
 *     semicolon-separated fields
 *     trailing separator
 *     generic record
 *     inherited record
 *     annotated record
 *     visibility-modified record
 *
 * REQUIRED CROSS-DOMAIN CASES:
 *
 *     record ClassicalData {
 *         value: int;
 *     }
 *
 *     record TensorData<T> {
 *         value: Tensor<T>;
 *     }
 *
 *     record QuantumData {
 *         state: Qubit;
 *     }
 *
 *     record HybridData<T> {
 *         classical: T;
 *         quantum: Qubit;
 *     }
 *
 * These tests verify type integration, not target-specific behavior.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The data adapter must reject malformed syntax through the canonical
 * declaration grammar, including malformed records such as:
 *
 *     record
 *
 *     record User
 *
 *     record User {
 *
 *     record User {
 *         : int;
 *     }
 *
 *     record User {
 *         id:
 *     }
 *
 *     record User {
 *         id: int
 *         name: string
 *     }
 *
 *     record User {
 *         id: int,,
 *     }
 *
 * Exact diagnostic wording belongs to the canonical diagnostics system.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that no artificial source-level limit is introduced.
 *
 * The test suite should exercise generated records with:
 *
 *     many fields
 *     many generic parameters
 *     deeply nested types
 *     large logical declarations
 *
 * without defining a language maximum.
 *
 * Resource exhaustion testing belongs to implementation-level validation and
 * must not become a grammar semantic restriction.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no parser-level machine capacities.
 *
 * Forbidden universal assumptions include:
 *
 *     fixed CPU count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed QPU count
 *     fixed qubit count
 *     fixed memory size
 *     fixed register width
 *     fixed tensor rank
 *     fixed network size
 *     fixed device count
 *     fixed record count
 *     fixed field count.
 *
 * No such assumptions are encoded here.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Canonical lexer vocabulary is used.
 * [x] Canonical declaration record grammar is reused.
 * [x] No duplicate record syntax exists here.
 * [x] No duplicate identifier grammar exists here.
 * [x] No duplicate type grammar exists here.
 * [x] No duplicate expression grammar exists here.
 * [x] No second record AST exists here.
 * [x] No second record IR exists here.
 * [x] No serialization implementation exists here.
 * [x] No storage implementation exists here.
 * [x] No database implementation exists here.
 * [x] No network implementation exists here.
 * [x] No hardware realization exists here.
 * [x] No quantum allocation exists here.
 * [x] No routing/scheduling/QEC/ZQN/HAL logic exists here.
 * [x] No hardware-size constants exist here.
 * [x] No semantic predicates exist here.
 * [x] No embedded Rust exists here.
 * [x] No unsafe Rust is required.
 * [x] Data-domain integration has exactly one public adapter rule.
 * [x] The data dispatcher can delegate to this rule.
 * [x] Canonical declaration semantics remain unchanged.
 * [x] Source order and source spans are preserved.
 * [x] POCO-REAF remains target-independent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 *
 * This is intentionally a very small grammar.
 *
 * The concrete record language belongs to:
 *
 *     ZamaniDeclarationRecords
 *
 * This file merely exposes it under the data-domain name.
 * ============================================================================
 */

parser grammar ZamaniDataRecords;

options {
    tokenVocab = ZamaniLexer;
}

import
    ZamaniDeclarationRecords;


/*
 * ============================================================================
 * PUBLIC DATA-DOMAIN ENTRY POINT
 * ============================================================================
 *
 * No EOF is consumed here.
 *
 * The complete-program entry point remains owned by:
 *
 *     grammar/Zamani.g4
 *
 * ============================================================================
 */

dataRecordDeclaration
    : recordDeclaration
    ;