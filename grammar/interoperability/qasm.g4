/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/qasm.g4
 *
 * Grammar:
 *     Qasm
 *
 * Status:
 *     CANONICAL ZAMANI -> QASM INTEROPERABILITY BOUNDARY
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the Zamani-side interoperability contract for OpenQASM.
 *
 * IMPORTANT:
 *
 *     This file is NOT a second OpenQASM grammar.
 *
 * The complete external OpenQASM syntax remains owned by:
 *
 *     grammar/interoperability/openqasm.g4
 *
 * and its dedicated frontend implementation:
 *
 *     src/quantum/frontend/formats/openqasm/
 *
 * This file owns only the Zamani interoperability boundary:
 *
 *     Zamani source
 *          |
 *          v
 *     QASM interoperability declaration
 *          |
 *          v
 *     OpenQASM format frontend
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * It therefore prevents OpenQASM-specific syntax from becoming part of the
 * permanent Zamani language core.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                       ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                     QASM INTEROP BOUNDARY
 *                              |
 *                              v
 *                  OpenQASM format frontend
 *                              |
 *                              v
 *                    OpenQASM syntax model
 *                              |
 *                              v
 *                     semantic validation
 *                              |
 *                              v
 *                         quantum::ir
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *          optimization      routing        scheduling
 *                              |
 *                              v
 *                       resilience / QEC
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                      target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - Zamani-side QASM interoperability declaration;
 *     - QASM format identity;
 *     - QASM version requirement metadata;
 *     - source embedding/reference intent;
 *     - import/export intent;
 *     - portability metadata;
 *     - semantic compatibility metadata;
 *     - capability/resource requirements associated with the boundary;
 *     - explicit safety policy for the interoperability boundary;
 *     - integration with the existing OpenQASM frontend.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - OpenQASM lexical syntax;
 *     - OpenQASM gate syntax;
 *     - OpenQASM declaration syntax;
 *     - OpenQASM expression syntax;
 *     - OpenQASM calibration syntax;
 *     - OpenQASM timing syntax;
 *     - OpenQASM standard-library definitions;
 *     - QASM parsing implementation;
 *     - quantum AST implementation;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration execution;
 *     - hardware discovery;
 *     - device selection;
 *     - physical qubit allocation;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There is exactly one owner for each layer.
 *
 * Zamani lexical vocabulary
 *     -> grammar/lexer/
 *
 * Zamani names
 *     -> grammar/core/names.g4
 *
 * Zamani attributes
 *     -> grammar/core/attributes.g4
 *
 * Zamani expressions
 *     -> grammar/expressions/
 *
 * Zamani types
 *     -> grammar/types/
 *
 * Generic FFI
 *     -> grammar/interoperability/ffi.g4
 *
 * ABI contracts
 *     -> grammar/interoperability/abi.g4
 *
 * Foreign callable declarations
 *     -> grammar/interoperability/foreign-functions.g4
 *
 * OpenQASM source-format syntax
 *     -> grammar/interoperability/openqasm.g4
 *
 * Zamani-to-QASM interoperability boundary
 *     -> THIS FILE
 *
 * OpenQASM semantic frontend
 *     -> src/quantum/frontend/formats/openqasm/
 *
 * Canonical quantum semantic representation
 *     -> quantum::ir
 *
 * This file MUST NOT redefine any of those owners.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is intentionally a parser grammar.
 *
 * It consumes the canonical Zamani lexer:
 *
 *     ZamaniLexer
 *
 * It does not create another lexer.
 *
 * It also deliberately does not import:
 *
 *     OpenQASM
 *
 * because openqasm.g4 is a combined grammar and is therefore not a legal
 * parser-grammar delegate under ANTLR's grammar-import model.
 *
 * The actual OpenQASM grammar remains an independently generated external
 * format frontend.
 *
 * ============================================================================
 * LEXICAL DESIGN
 * ============================================================================
 *
 * The Zamani side of the boundary uses only canonical Zamani vocabulary.
 *
 * In particular, this grammar does NOT introduce:
 *
 *     QASM-specific lexer keywords
 *     gate-specific lexer tokens
 *     vendor-specific lexer tokens
 *     device-specific lexer tokens
 *
 * OpenQASM-specific lexical recognition belongs to the OpenQASM frontend.
 *
 * This is essential because OpenQASM is an interoperability format, not a
 * permanent extension of Zamani's global keyword set.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * QASM interoperability MUST remain target-independent.
 *
 * This file MUST NOT encode:
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
 * It also MUST NOT encode:
 *
 *     physical qubit numbers
 *     device numbers
 *     processor identifiers
 *     fixed topology
 *     fixed coupling maps
 *     fixed memory capacities
 *     fixed register widths
 *
 * A QASM program may contain an explicit program-level value such as:
 *
 *     1024
 *
 * without that becoming a Zamani implementation limit.
 *
 * ============================================================================
 * VERSION SEMANTICS
 * ============================================================================
 *
 * A QASM version is a FORMAT COMPATIBILITY REQUIREMENT.
 *
 * It is NOT a compiler capacity.
 *
 * For example, a semantic interoperability declaration may state that the
 * associated source is intended for:
 *
 *     OpenQASM 3.x
 *
 * The exact accepted version is determined by the OpenQASM frontend and
 * compatibility policy.
 *
 * This grammar therefore does not enumerate:
 *
 *     3.0
 *     3.1
 *     3.2
 *     ...
 *
 * as separate language constructs.
 *
 * Version resolution belongs to the interoperability frontend.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no embedded Rust;
 *     no filesystem access;
 *     no network access;
 *     no process execution;
 *     no dynamic loading;
 *     no hardware discovery;
 *     no runtime execution.
 *
 * Parsing a QASM interoperability declaration MUST be inert.
 *
 * The Rust implementation must remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * QASM SOURCE OWNERSHIP
 * ============================================================================
 *
 * The external OpenQASM source may be represented by the frontend as:
 *
 *     inline source
 *     named source unit
 *     source artifact
 *     already-parsed interoperability unit
 *
 * This grammar does not resolve files, URLs, packages, libraries, or network
 * resources.
 *
 * Resolution belongs to the compiler/toolchain under explicit capability and
 * security policy.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The following distinction is mandatory:
 *
 *     QASM syntax
 *         !=
 *     QASM semantics
 *         !=
 *     Zamani quantum semantics
 *         !=
 *     physical realization
 *
 * OpenQASM constructs must be translated into the repository's canonical
 * semantic representation.
 *
 * They must NOT create:
 *
 *     OpenQASM IR
 *     QASMGateIR
 *     QASMQuantumIR
 *
 * as competing permanent IRs.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * QASM interoperability may require capabilities such as:
 *
 *     quantum.measurement
 *     quantum.dynamic_control
 *     quantum.mid_circuit_measurement
 *     quantum.timing
 *     quantum.calibration
 *
 * The exact capability names remain semantic data.
 *
 * This grammar MUST NOT enumerate the capabilities of today's hardware.
 *
 * Resource requirements and capability validation belong downstream.
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * This file does not redefine FFI or ABI contracts.
 *
 * If a QASM interoperability boundary crosses an external ABI, the semantic
 * contract must use:
 *
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/abi.g4
 *
 * respectively.
 *
 * This avoids creating a QASM-specific ABI model.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * The QASM frontend is implemented by the existing Rust quantum frontend.
 *
 * The grammar MUST remain independent of:
 *
 *     Rust structs
 *     Rust enums
 *     Rust ownership internals
 *     Rust pointers
 *     Rust memory layout
 *     Rust unsafe blocks
 *
 * Rust is an implementation language for Zamani.
 *
 * It is not the semantic model of QASM interoperability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR IDENTITY
 * ============================================================================
 */

parser grammar Qasm;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The surrounding interoperability composition grammar decides where this
 * construct may occur.
 *
 * This rule is intentionally NOT a compilation-unit root.
 *
 * ============================================================================
 */

qasmInterop
    : qasmDeclaration
    ;


/*
 * ============================================================================
 * QASM DECLARATION
 * ============================================================================
 *
 * Conceptual forms:
 *
 *     extern language "OpenQASM" ...
 *
 * or the equivalent canonical Zamani interoperability representation supplied
 * by the surrounding interoperability grammar.
 *
 * The actual keyword/language spelling is owned by the canonical Zamani
 * lexical and interoperability contracts.
 *
 * ============================================================================
 */

qasmDeclaration
    : EXTERN
      LANGUAGE
      qasmLanguageIdentity
      qasmContractClause*
      qasmSourceClause?
      qasmImportExportClause*
      qasmRequirementClause*
      qasmCapabilityClause*
      qasmAttributeClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * LANGUAGE IDENTITY
 * ============================================================================
 *
 * A language/format identity is deliberately represented as source data
 * rather than a finite lexer enumeration.
 *
 * This permits future OpenQASM-compatible revisions and avoids adding a new
 * global keyword whenever an interoperability format evolves.
 *
 * ============================================================================
 */

qasmLanguageIdentity
    : stringLiteral
    | qualifiedName
    ;


/*
 * ============================================================================
 * CONTRACT CLAUSES
 * ============================================================================
 *
 * These clauses describe interoperability intent.
 *
 * They do not perform source resolution or compilation.
 * ============================================================================
 */

qasmContractClause
    : qasmVersionClause
    | qasmModeClause
    | qasmRepresentationClause
    ;


/*
 * ============================================================================
 * VERSION
 * ============================================================================
 *
 * Version data remains semantic metadata.
 *
 * We deliberately do not hard-code OpenQASM 3.0/3.1/etc. into this grammar.
 *
 * The semantic frontend validates the requested version against the supported
 * OpenQASM compatibility matrix.
 * ============================================================================
 */

qasmVersionClause
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * MODE
 * ============================================================================
 *
 * A mode is an interoperability policy, not a physical execution mode.
 * ============================================================================
 */

qasmModeClause
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * REPRESENTATION
 * ============================================================================
 *
 * Describes how the source artifact is represented.
 *
 * This does NOT mean physical machine representation.
 * ============================================================================
 */

qasmRepresentationClause
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * A source reference is opaque to this grammar.
 *
 * It is not interpreted as:
 *
 *     a filesystem path
 *     URL
 *     device
 *     library
 *     executable
 *
 * The downstream resolver decides how the source is obtained.
 * ============================================================================
 */

qasmSourceClause
    : identifier
      ASSIGN
      stringLiteral
    ;


/*
 * ============================================================================
 * IMPORT / EXPORT INTENT
 * ============================================================================
 *
 * These rules describe interoperability direction only.
 *
 * They do not perform filesystem, package, network, or linker operations.
 * ============================================================================
 */

qasmImportExportClause
    : qasmImportClause
    | qasmExportClause
    ;


qasmImportClause
    : identifier
      ASSIGN
      qasmSymbolReferenceList
    ;


qasmExportClause
    : identifier
      ASSIGN
      qasmSymbolReferenceList
    ;


qasmSymbolReferenceList
    : qasmSymbolReference
      (
          COMMA
          qasmSymbolReference
      )*
    ;


qasmSymbolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * A requirement describes something that must be satisfied.
 *
 * It does NOT select a physical target.
 *
 * Example semantic forms:
 *
 *     requires capability("quantum.measurement")
 *     requires qubits >= n
 *
 * The expression remains owned by the canonical expression grammar.
 * ============================================================================
 */

qasmRequirementClause
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Capability names are intentionally open-ended.
 *
 * No vendor/backend capability enumeration is encoded here.
 * ============================================================================
 */

qasmCapabilityClause
    : CAPABILITY
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attribute syntax remains owned by the canonical attribute grammar.
 *
 * This wrapper provides the QASM interoperability composition point without
 * duplicating attribute syntax.
 * ============================================================================
 */

qasmAttributeClause
    : attribute
    ;


/*
 * ============================================================================
 * SOURCE ARTIFACT REFERENCE
 * ============================================================================
 *
 * Public reusable rule for other interoperability composition grammars.
 * ============================================================================
 */

qasmSourceReference
    : stringLiteral
    ;


/*
 * ============================================================================
 * FORMAT IDENTITY
 * ============================================================================
 *
 * This rule provides a semantic boundary that can be used by compatibility
 * and tooling layers without making the parser responsible for version
 * resolution.
 * ============================================================================
 */

qasmFormatIdentity
    : qasmLanguageIdentity
    ;


/*
 * ============================================================================
 * PUBLIC CONTRACT ENTRY
 * ============================================================================
 */

qasmContract
    : qasmContractClause+
    ;


/*
 * ============================================================================
 * PUBLIC REQUIREMENT ENTRY
 * ============================================================================
 */

qasmRequirements
    : qasmRequirementClause+
    ;


/*
 * ============================================================================
 * PUBLIC CAPABILITY ENTRY
 * ============================================================================
 */

qasmCapabilities
    : qasmCapabilityClause+
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar maps to the EXISTING DOMAIN-NEUTRAL frontend AST.
 *
 * Conceptual representation:
 *
 *     QasmInteropDeclaration
 *         language
 *         version
 *         mode
 *         representation
 *         source
 *         imports
 *         exports
 *         requirements
 *         capabilities
 *         attributes
 *         source_span
 *
 * The exact Rust AST type belongs to:
 *
 *     src/frontend/ast/
 *
 * or the established interoperability AST boundary.
 *
 * This grammar must not require a new QASM-specific semantic IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 * 1. Validate the declared format identity.
 *
 * 2. Resolve the requested QASM version.
 *
 * 3. Check compatibility against the OpenQASM frontend.
 *
 * 4. Resolve the source artifact according to compiler/toolchain policy.
 *
 * 5. Parse the external OpenQASM source using:
 *
 *        grammar/interoperability/openqasm.g4
 *
 *    or the production Rust OpenQASM frontend.
 *
 * 6. Convert the resulting OpenQASM representation into Zamani's canonical
 *    quantum semantic representation.
 *
 * 7. Validate types and classical/quantum boundaries.
 *
 * 8. Validate measurement and dynamic-control semantics.
 *
 * 9. Validate resource requirements.
 *
 * 10. Validate capabilities.
 *
 * 11. Validate compatibility constraints.
 *
 * 12. Preserve source provenance and source spans.
 *
 * 13. Reject unsupported constructs explicitly.
 *
 * The parser must NOT perform these semantic operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * OPENQASM FRONTEND INTEGRATION
 * ============================================================================
 *
 * The external-format implementation remains:
 *
 *     src/quantum/frontend/formats/openqasm/
 *
 * The frontend should provide the complete OpenQASM parsing path.
 *
 * Conceptually:
 *
 *     qasmInterop
 *          |
 *          v
 *     source artifact
 *          |
 *          v
 *     OpenQasmLexer / parser
 *          |
 *          v
 *     OpenQASM AST
 *          |
 *          v
 *     semantic conversion
 *          |
 *          v
 *     Zamani quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * The external OpenQASM AST is an interoperability representation.
 *
 * It must not become the canonical Zamani quantum AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * OpenQASM interoperability MUST lower into:
 *
 *     quantum::ir
 *
 * only.
 *
 * It must not create:
 *
 *     qasm::ir
 *     openqasm::ir
 *     QasmIR
 *
 * as competing canonical quantum representations.
 *
 * The OpenQASM representation may exist temporarily at the format boundary,
 * but it must terminate at the canonical semantic boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ROUTING / SCHEDULING / QEC / ZQN
 * ============================================================================
 *
 * This grammar does not perform:
 *
 *     routing
 *     scheduling
 *     QEC
 *     resilience
 *     ZQN analysis
 *     calibration
 *     topology discovery
 *     physical placement
 *
 * After semantic conversion:
 *
 *     quantum::ir
 *          |
 *          +--> optimization
 *          |
 *          +--> routing
 *          |
 *          +--> scheduling
 *          |
 *          +--> resilience / QEC
 *          |
 *          +--> ZQN
 *          |
 *          +--> HAL
 *          |
 *          +--> target realization
 *
 * remains the authoritative pipeline.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * An OpenQASM program imported through this boundary must retain its defined
 * semantics when realized on different targets.
 *
 * A target may reject the program because it lacks:
 *
 *     required capabilities
 *     required resources
 *     required semantics
 *
 * but the compiler must not silently reinterpret the source merely because
 * the target differs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no finite language-level limits for:
 *
 *     declarations
 *     imported symbols
 *     exported symbols
 *     requirements
 *     capabilities
 *     attributes
 *     source artifacts
 *     targets
 *     qubits
 *     classical bits
 *     gates
 *     circuit depth
 *     circuit width
 *     devices
 *     nodes
 *     memory
 *     threads
 *
 * Repetition is represented with `*` or `+`.
 *
 * Any actual compiler safety/resource budget belongs to runtime configuration,
 * compiler configuration, or execution policy.
 *
 * Those limits must never become universal Zamani grammar semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains none of:
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
 * It also does not contain:
 *
 *     q0
 *     q1
 *     q2
 *
 * as a universal resource model.
 *
 * OpenQASM source may contain such identifiers where they are ordinary
 * program-level identifiers, but their existence must never establish a
 * Zamani hardware limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar
 *     language version
 *
 * It must not depend on:
 *
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     runtime state
 *     wall-clock time
 *     randomness
 *     deployment topology
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * No parsing operation may:
 *
 *     open a QASM file;
 *     download a QASM file;
 *     execute QASM;
 *     load a provider;
 *     load a library;
 *     access credentials;
 *     inspect hardware;
 *     select a device;
 *     allocate physical resources.
 *
 * Source resolution and execution require explicit downstream authorization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors:
 *
 *     malformed qasm interoperability declaration
 *
 * Semantic errors:
 *
 *     unsupported QASM format
 *     unsupported QASM version
 *     incompatible QASM construct
 *     unresolved source artifact
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *
 * Runtime/backend errors:
 *
 *     target unavailable
 *     scheduling failure
 *     routing failure
 *     calibration failure
 *     QEC/resilience failure
 *
 * These categories MUST remain distinct.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing OpenQASM support remains owned by:
 *
 *     grammar/interoperability/openqasm.g4
 *
 * Existing Rust OpenQASM frontend remains owned by:
 *
 *     src/quantum/frontend/formats/openqasm/
 *
 * This file adds the Zamani-side composition boundary without replacing or
 * renaming the existing OpenQASM grammar.
 *
 * Compatibility tooling must therefore test:
 *
 *     Zamani -> QASM interoperability
 *     QASM -> Zamani semantic conversion
 *     supported OpenQASM versions
 *     unsupported-version diagnostics
 *     source provenance
 *     semantic preservation
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     qasm language identity
 *     qasm version metadata
 *     qasm source reference
 *     qasm requirements
 *     qasm capabilities
 *     qasm attributes
 *     qasm import/export metadata
 *
 * Negative:
 *
 *     missing language identity
 *     malformed contract
 *     missing assignment
 *     empty symbol reference
 *     invalid attribute
 *
 * Boundary:
 *
 *     many requirements
 *     many capabilities
 *     many imported symbols
 *     many exported symbols
 *     long qualified names
 *     arbitrarily large source metadata
 *
 * Scalability:
 *
 *     no finite QASM program-size limit
 *     no finite qubit limit
 *     no finite gate-count limit
 *     no finite circuit-depth limit
 *     no finite device-count limit
 *
 * Determinism:
 *
 *     identical source + grammar -> identical parse structure
 *
 * Compatibility:
 *
 *     supported OpenQASM versions
 *     unsupported OpenQASM versions
 *     version migration diagnostics
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * qasm.g4 is complete when:
 *
 *     [x] It does not duplicate openqasm.g4.
 *     [x] It has a unique ANTLR grammar identity.
 *     [x] It uses the canonical Zamani lexer.
 *     [x] It owns only the Zamani-side interoperability boundary.
 *     [x] It does not create QASM-specific universal keywords.
 *     [x] It does not create a second QASM IR.
 *     [x] It preserves the quantum::ir boundary.
 *     [x] It does not select hardware.
 *     [x] It does not encode hardware capacities.
 *     [x] It does not execute external source.
 *     [x] It does not use unsafe Rust.
 *     [x] It remains compatible with Rust 1.97/1.97.1 implementation.
 *     [x] It preserves source provenance requirements.
 *     [x] It defines AST integration.
 *     [x] It defines semantic integration.
 *     [x] It defines IR integration.
 *     [x] It defines compiler/runtime integration.
 *     [x] It defines positive/negative/boundary/scalability tests.
 *
 * ============================================================================
 * END
 * ============================================================================
 */