/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/foreign.g4
 *
 * Grammar:
 *     InteroperabilityForeign
 *
 * Status:
 *     CANONICAL FOREIGN-BOUNDARY IDENTITY / METADATA GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE RUST
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns the reusable SOURCE-LEVEL DESCRIPTION of a foreign
 * interoperability boundary.
 *
 * It provides the common syntactic vocabulary needed by:
 *
 *     foreign callable declarations
 *     FFI declarations
 *     foreign type declarations
 *     ABI references
 *     interoperability dialects
 *     foreign callbacks
 *     foreign services
 *     foreign runtimes
 *     foreign data sources
 *     foreign implementations
 *     future interoperability domains
 *
 * This file intentionally does NOT own foreign functions themselves.
 *
 * Foreign callable declaration ownership remains:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * FFI boundary behavior remains:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI contracts remain:
 *
 *     grammar/interoperability/abi.g4
 *
 * Foreign type syntax remains:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * Calling-convention syntax remains:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Interoperability composition remains:
 *
 *     grammar/interoperability/interoperability.g4
 *
 *
 * The purpose of this file is therefore deliberately narrow:
 *
 *     FOREIGN IDENTITY
 *         +
 *     FOREIGN SOURCE REFERENCE
 *         +
 *     FOREIGN TARGET/IMPLEMENTATION REFERENCE
 *         +
 *     FOREIGN EXTENSIBLE METADATA
 *
 * These constructs are reusable across the interoperability subsystem without
 * forcing each foreign-domain grammar to redefine them.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                       canonical parser
 *                              |
 *                              v
 *                 interoperability composition
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        foreign.g4         ffi.g4          abi.g4
 *             |                |                |
 *             v                v                v
 *    foreign identity    FFI boundary     ABI contract
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                    foreign semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *          effects        capabilities       resources
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                         policies
 *                              |
 *                              v
 *                         provenance
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       classical IR       quantum::ir       HDL/hardware
 *                              |
 *                              v
 *                   optimization / lowering
 *                              |
 *                              v
 *                    routing / scheduling
 *                              |
 *                              v
 *                      target realization
 *
 * This grammar is declarative and inert.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     foreignSource
 *     foreignSourceReference
 *     foreignIdentity
 *     foreignReference
 *     foreignTargetReference
 *     foreignImplementationReference
 *     foreignMetadata
 *     foreignMetadataAttribute
 *     foreignMetadataAssignment
 *     foreignMetadataBlock
 *     foreignMetadataItem
 *     foreignContractMetadata
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     foreign function declarations
 *     foreign function calls
 *     FFI calls
 *     callbacks
 *     ABI declarations
 *     ABI layouts
 *     calling conventions
 *     foreign type declarations
 *     general types
 *     general expressions
 *     identifiers
 *     qualified names
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     security
 *     provenance semantics
 *     target selection
 *     linker behavior
 *     loader behavior
 *     runtime execution
 *     dynamic library loading
 *     symbol resolution
 *     machine representation
 *     physical memory
 *     hardware placement
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The following concepts MUST have exactly one grammar owner:
 *
 *     identifiers
 *         -> grammar/core/identifiers.g4
 *
 *     qualified names
 *         -> grammar/core/names.g4
 *
 *     attributes
 *         -> grammar/core/attributes.g4
 *
 *     expressions
 *         -> grammar/expressions/expressions.g4
 *
 *     foreign callable declarations
 *         -> grammar/interoperability/foreign-functions.g4
 *
 *     FFI boundary
 *         -> grammar/interoperability/ffi.g4
 *
 *     ABI
 *         -> grammar/interoperability/abi.g4
 *
 *     foreign types
 *         -> grammar/interoperability/foreign-types.g4
 *
 *     calling conventions
 *         -> grammar/interoperability/calling-conventions.g4
 *
 * This file MUST NOT recreate any of those authorities.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * EXPORTS:
 *
 *     foreignSource
 *     foreignSourceReference
 *     foreignIdentity
 *     foreignReference
 *     foreignTargetReference
 *     foreignImplementationReference
 *     foreignMetadata
 *     foreignMetadataAttribute
 *     foreignMetadataAssignment
 *     foreignMetadataBlock
 *     foreignMetadataItem
 *     foreignContractMetadata
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/interoperability/foreign-functions.g4
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/foreign-types.g4
 *     grammar/interoperability/abi.g4
 *     grammar/interoperability/interoperability.g4
 *     interoperability-specific dialect grammars
 *     AST construction
 *     semantic interoperability analysis
 *
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 *
 * SEMANTIC_OWNER:
 *
 *     interoperability semantic model
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *
 * Foreign operations requiring quantum representation MUST eventually use:
 *
 *     quantum::ir
 *
 * They MUST NOT create another foreign-quantum IR.
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/interoperability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/interoperability.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The parser-facing lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No lexer rules are defined here.
 *
 * No parser-local lexer vocabulary is created.
 *
 * No target-language-specific keywords are created.
 *
 * Existing canonical tokens used by this grammar include:
 *
 *     STRING_LITERAL
 *     ASSIGN
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *
 * Names are consumed through:
 *
 *     identifier
 *     qualifiedName
 *
 * Expressions are consumed through:
 *
 *     expression
 *
 * ============================================================================
 * KEYWORD POLICY
 * ============================================================================
 *
 * This file intentionally introduces NO new global keyword.
 *
 * In particular, it does not reserve words for:
 *
 *     C
 *     C++
 *     Rust
 *     Python
 *     Java
 *     Fortran
 *     CUDA
 *     OpenCL
 *     WebAssembly
 *     OpenQASM
 *     Verilog
 *     SystemVerilog
 *     vendor names
 *     operating systems
 *     platforms
 *     libraries
 *     device names
 *     ABI names
 *
 * A foreign identity is data.
 *
 * A foreign source is data.
 *
 * A foreign target is data.
 *
 * A foreign implementation is data.
 *
 * Future interoperability ecosystems therefore do not require modification
 * of this grammar merely because a new language, runtime, vendor, protocol,
 * architecture, accelerator, or service appears.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Foreign interoperability is OPEN-WORLD.
 *
 * The grammar MUST remain valid when new foreign systems are introduced.
 *
 * Examples of symbolic identities include:
 *
 *     c
 *     cpp
 *     rust
 *     python
 *     fortran
 *     wasm
 *     openqasm
 *     systemverilog
 *     vendor::runtime
 *     organization::service
 *     future::foreign::system
 *
 * These are examples only.
 *
 * None are reserved by this grammar.
 *
 * ============================================================================
 * FOREIGN SOURCE
 * ============================================================================
 *
 * A foreign source identifies the source ecosystem, implementation family,
 * provider, language, runtime, service, or other externally defined origin.
 *
 * Examples:
 *
 *     extern "C"
 *
 *     extern "Fortran"
 *
 *     extern "vendor.runtime"
 *
 *     extern foreign::runtime
 *
 * The grammar preserves the identity.
 *
 * Semantic analysis determines what that identity means.
 *
 * ============================================================================
 * FOREIGN IDENTITY
 * ============================================================================
 *
 * A foreign identity is deliberately broader than a programming language.
 *
 * It may represent:
 *
 *     language
 *     runtime
 *     library
 *     service
 *     protocol
 *     execution environment
 *     hardware interface
 *     accelerator interface
 *     quantum service
 *     HDL environment
 *     simulator
 *     future interoperability provider
 *
 * The grammar does not distinguish these categories through hard-coded
 * keyword lists.
 *
 * Semantic metadata supplies the category.
 *
 * ============================================================================
 * FOREIGN TARGET
 * ============================================================================
 *
 * A target reference identifies a symbolic realization target.
 *
 * It does NOT select hardware during parsing.
 *
 * Examples:
 *
 *     provider::runtime
 *     service::compute
 *     simulator::quantum
 *     accelerator::tensor
 *
 * The semantic layer determines whether the reference is:
 *
 *     valid;
 *     available;
 *     authorized;
 *     compatible;
 *     capable;
 *     resource-feasible.
 *
 * ============================================================================
 * FOREIGN IMPLEMENTATION
 * ============================================================================
 *
 * A foreign implementation reference identifies the semantic implementation
 * source of a foreign boundary.
 *
 * It is declarative metadata.
 *
 * It does NOT:
 *
 *     load a library;
 *     resolve a symbol;
 *     execute code;
 *     inspect the filesystem;
 *     access the network;
 *     inspect hardware;
 *     allocate memory;
 *     invoke a process.
 *
 * ============================================================================
 * METADATA MODEL
 * ============================================================================
 *
 * Foreign metadata is intentionally extensible.
 *
 * Metadata can describe:
 *
 *     language
 *     runtime
 *     provider
 *     target
 *     symbol
 *     library
 *     module
 *     package
 *     version
 *     compatibility
 *     adapter
 *     marshaling
 *     serialization
 *     ownership
 *     lifetime
 *     nullability
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     security
 *     provenance
 *     determinism
 *     concurrency
 *     asynchronous behavior
 *     simulation
 *     quantum participation
 *     HDL participation
 *     hardware participation
 *
 * However, this grammar does NOT make those names mandatory keywords.
 *
 * Semantic consumers interpret metadata keys.
 *
 * Unknown metadata keys MUST be preserved for forward compatibility unless a
 * higher-level policy explicitly rejects them.
 *
 * ============================================================================
 * METADATA KEY MODEL
 * ============================================================================
 *
 * Metadata keys may be:
 *
 *     identifier
 *
 * or:
 *
 *     qualifiedName
 *
 * This permits open namespaces such as:
 *
 *     abi.calling_convention
 *     ffi.symbol
 *     foreign.language
 *     vendor.extension
 *     organization.policy
 *     future.interoperability.feature
 *
 * The grammar does not maintain a central finite enum of metadata keys.
 *
 * ============================================================================
 * METADATA VALUES
 * ============================================================================
 *
 * Metadata values use the canonical expression grammar.
 *
 * This allows:
 *
 *     literals
 *     names
 *     qualified names
 *     structured expressions
 *     symbolic values
 *     compile-time expressions
 *     references
 *
 * The semantic layer determines which metadata keys require:
 *
 *     compile-time constants;
 *     symbolic references;
 *     capability expressions;
 *     resource expressions;
 *     policies;
 *     provenance;
 *     type information.
 *
 * The parser does not make those semantic decisions.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file does NOT own resource requirements.
 *
 * It merely permits resource-related metadata to cross the foreign boundary.
 *
 * Examples of semantic metadata include:
 *
 *     resources.memory
 *     resources.compute
 *     resources.network
 *     resources.quantum
 *
 * The semantic resource system owns their interpretation.
 *
 * There are NO grammar-level limits for:
 *
 *     memory
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     threads
 *     nodes
 *     devices
 *     tensor dimensions
 *     tensor rank
 *     network endpoints
 *     foreign declarations
 *     metadata entries
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities remain owned by the capability/resource semantic subsystem.
 *
 * Foreign metadata may reference capabilities such as:
 *
 *     capability("foreign.call")
 *     capability("foreign.callback")
 *     capability("foreign.remote")
 *     capability("foreign.quantum")
 *     capability("foreign.hardware")
 *
 * These are semantic examples, not grammar-enforced identifiers.
 *
 * A metadata declaration never grants a capability.
 *
 * It only describes a requirement or relationship.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A foreign boundary may participate in the canonical effect system.
 *
 * Examples:
 *
 *     foreign::call
 *     foreign::io
 *     foreign::network
 *     foreign::callback
 *     foreign::hardware
 *     foreign::quantum
 *
 * This grammar does not define those effects.
 *
 * `effects.*` metadata is only a syntactic carrier for semantic information.
 *
 * The canonical effect subsystem determines:
 *
 *     effect identity;
 *     effect composition;
 *     effect checking;
 *     effect handlers;
 *     effect capabilities.
 *
 * ============================================================================
 * CONTRACT / VALIDATION INTEGRATION
 * ============================================================================
 *
 * Foreign boundaries may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     evidence
 *     verification
 *
 * Those concepts belong to the validation/contract systems.
 *
 * This grammar does not duplicate their grammar.
 *
 * Instead, interoperability metadata may reference the resulting semantic
 * contract through namespaced metadata.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Foreign interoperability may be constrained by:
 *
 *     security policy
 *     sandbox policy
 *     resource policy
 *     execution policy
 *     deployment policy
 *     compatibility policy
 *     adaptation policy
 *
 * Policy evaluation is downstream.
 *
 * A foreign declaration does not grant permission merely by existing.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Foreign boundaries are particularly important provenance boundaries.
 *
 * Semantic analysis may attach provenance describing:
 *
 *     source identity
 *     provider
 *     implementation
 *     version
 *     transformation
 *     adapter
 *     evidence
 *     verification
 *     compatibility decision
 *     execution decision
 *
 * This grammar only preserves the source metadata required to construct that
 * semantic provenance.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A foreign boundary may participate in quantum-classical computation.
 *
 * Examples:
 *
 *     foreign::quantum
 *     provider::quantum
 *     simulator::quantum
 *     service::qpu
 *
 * The grammar does NOT define:
 *
 *     physical qubits
 *     qubit IDs
 *     gate sets
 *     coupling maps
 *     calibration
 *     routing
 *     scheduling
 *     pulse programs
 *     QEC
 *     resilience implementation
 *     ZQN
 *     HAL
 *
 * If semantic lowering determines that a foreign boundary carries quantum
 * operations, those operations MUST enter:
 *
 *     quantum::ir
 *
 * rather than a foreign-specific quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Foreign metadata may identify:
 *
 *     HDL providers
 *     synthesis systems
 *     simulation environments
 *     verification environments
 *     accelerator interfaces
 *     hardware services
 *     device interfaces
 *
 * The grammar does not define:
 *
 *     pin counts
 *     register widths
 *     bus widths
 *     memory addresses
 *     device counts
 *     FPGA capacity
 *     ASIC capacity
 *     physical topology
 *
 * Such information belongs to target/resource/capability semantics.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK BOUNDARY
 * ============================================================================
 *
 * A foreign implementation may be:
 *
 *     local
 *     remote
 *     distributed
 *     network-provided
 *     cloud-provided
 *     service-provided
 *     simulator-provided
 *
 * The grammar does not encode:
 *
 *     fixed node counts;
 *     fixed endpoint counts;
 *     fixed network sizes;
 *     physical addresses;
 *     routing tables;
 *     network topology.
 *
 * Those are semantic/runtime concerns.
 *
 * ============================================================================
 * ADAPTATION CONTRACT
 * ============================================================================
 *
 * A foreign boundary may be adapted at compile time or runtime.
 *
 * Adaptation MUST remain controlled by:
 *
 *     capabilities
 *     policies
 *     contracts
 *     resource availability
 *     compatibility
 *     provenance
 *     authorization
 *
 * This grammar does not execute adaptation.
 *
 * Metadata may describe adaptation intent, but the execution subsystem owns
 * the actual adaptation process.
 *
 * ============================================================================
 * SIMULATION CONTRACT
 * ============================================================================
 *
 * A foreign implementation may be represented by a simulator.
 *
 * Examples:
 *
 *     simulator::foreign
 *     simulator::quantum
 *     simulator::hardware
 *     simulator::distributed
 *
 * Simulation remains an execution strategy.
 *
 * It does not create a second language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text;
 *     lexer configuration;
 *     grammar;
 *     selected grammar composition.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     current time;
 *     randomness;
 *     runtime state;
 *     linker state;
 *     foreign library availability.
 *
 * Availability and compatibility are semantic analysis concerns.
 *
 * ============================================================================
 * SAFETY / INERTNESS CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no parser actions;
 *     no filesystem operations;
 *     no network operations;
 *     no process execution;
 *     no dynamic loading;
 *     no symbol resolution;
 *     no hardware discovery;
 *     no native-memory access;
 *     no pointer dereferencing;
 *     no runtime invocation.
 *
 * Generated Rust integration MUST remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar introduces no source-language machine ceiling.
 *
 * It MUST NOT define:
 *
 *     MAX_FOREIGN_SOURCES
 *     MAX_FOREIGN_TARGETS
 *     MAX_FOREIGN_IMPLEMENTATIONS
 *     MAX_FOREIGN_METADATA
 *     MAX_FOREIGN_INTERFACES
 *     MAX_FOREIGN_FUNCTIONS
 *     MAX_FOREIGN_TYPES
 *     MAX_FOREIGN_PARAMETERS
 *     MAX_FOREIGN_CALLBACKS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * or equivalent constants.
 *
 * ANTLR repetition operators represent arbitrary source cardinality.
 *
 * Practical parser/compiler limits remain implementation/resource policies,
 * not language semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar MUST NOT create a separate foreign AST universe.
 *
 * The frontend AST should preserve the common structural information:
 *
 *     source span
 *     foreign identity
 *     source reference
 *     target reference
 *     implementation reference
 *     metadata
 *
 * The semantic layer then determines whether the declaration represents:
 *
 *     language;
 *     runtime;
 *     library;
 *     service;
 *     protocol;
 *     hardware interface;
 *     quantum provider;
 *     simulator;
 *     distributed service;
 *     future interoperability domain.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic validation MUST determine:
 *
 *     whether a foreign identity exists;
 *     whether a source reference is valid;
 *     whether a target reference is compatible;
 *     whether an implementation is authorized;
 *     whether metadata is recognized;
 *     whether required capabilities exist;
 *     whether required resources are available;
 *     whether policies permit use;
 *     whether effects are declared correctly;
 *     whether contracts are satisfiable;
 *     whether provenance is sufficient;
 *     whether ABI/FFI compatibility exists.
 *
 * These are NOT parser responsibilities.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Metadata is intentionally open-world.
 *
 * Adding a new namespaced metadata key SHOULD NOT require modifying this
 * grammar.
 *
 * Removing or changing the meaning of a metadata key is a semantic/specification
 * compatibility event, not a parser grammar event.
 *
 * Deprecated metadata may remain syntactically parseable while semantic
 * validation emits the appropriate compatibility diagnostic.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 *     foreignSource
 *     foreignSourceReference
 *     foreignIdentity
 *     foreignReference
 *     foreignTargetReference
 *     foreignImplementationReference
 *     foreignMetadata
 *     foreignMetadataAttribute
 *     foreignMetadataAssignment
 *     foreignMetadataBlock
 *     foreignMetadataItem
 *     foreignContractMetadata
 *
 * These rules are public composition points.
 *
 * ============================================================================
 * PRIVATE RULES
 * ============================================================================
 *
 * No rule in this file is semantically private merely because it is named
 * internally. Composition grammars may consume any explicitly exported rule.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar InteroperabilityForeign;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Attributes,
    Expressions
;


/*
 * ============================================================================
 * 1. FOREIGN SOURCE
 * ============================================================================
 *
 * Canonical examples:
 *
 *     "C"
 *     "Fortran"
 *     "vendor.runtime"
 *
 * The string form is the most direct representation of an external source
 * identity.
 *
 * A qualified symbolic form permits extensible semantic identities without
 * forcing every provider into a lexer keyword.
 */

foreignSource
    : STRING_LITERAL
    | qualifiedName
    ;


/*
 * ============================================================================
 * 2. FOREIGN SOURCE REFERENCE
 * ============================================================================
 *
 * Reusable wrapper used by foreign-function, FFI, and interoperability
 * grammars.
 */

foreignSourceReference
    : foreignSource
    ;


/*
 * ============================================================================
 * 3. FOREIGN IDENTITY
 * ============================================================================
 *
 * A foreign identity is a symbolic identity for an external computational
 * ecosystem.
 */

foreignIdentity
    : qualifiedName
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * 4. GENERIC FOREIGN REFERENCE
 * ============================================================================
 *
 * This is intentionally symbolic.
 *
 * It does not resolve a library, process, service, hardware target, or
 * runtime.
 */

foreignReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. FOREIGN TARGET REFERENCE
 * ============================================================================
 *
 * Identifies a symbolic realization target.
 */

foreignTargetReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. FOREIGN IMPLEMENTATION REFERENCE
 * ============================================================================
 *
 * Identifies a symbolic implementation/provider.
 */

foreignImplementationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. FOREIGN METADATA
 * ============================================================================
 *
 * Metadata is deliberately open-world.
 *
 * It may be:
 *
 *     an attribute;
 *     a key/value assignment.
 *
 * No finite metadata vocabulary is embedded here.
 */

foreignMetadata
    : foreignMetadataAttribute
    | foreignMetadataAssignment
    ;


/*
 * ============================================================================
 * 8. ATTRIBUTE METADATA
 * ============================================================================
 *
 * Attribute ownership remains in grammar/core/attributes.g4.
 */

foreignMetadataAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 9. KEY/VALUE METADATA
 * ============================================================================
 *
 * Example semantic forms:
 *
 *     foreign.language = "C";
 *     foreign.symbol = "sin";
 *     foreign.provider = vendor::runtime;
 *     foreign.version = version_expression;
 *     foreign.policy = policy::name;
 *
 * The grammar does not interpret the key.
 */

foreignMetadataAssignment
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. METADATA BLOCK
 * ============================================================================
 *
 * A block is reusable by FFI, foreign functions, adapters, services, and
 * future interoperability domains.
 *
 * Example:
 *
 *     {
 *         foreign.language = "C";
 *         foreign.provider = provider::runtime;
 *     }
 *
 * The surrounding grammar owns the meaning of the block.
 */

foreignMetadataBlock
    : LBRACE
      foreignMetadataItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 11. METADATA ITEM
 * ============================================================================
 *
 * Keep the item rule deliberately small.
 *
 * Nested semantic structures belong in expressions or attributes rather than
 * being duplicated as dozens of foreign-specific grammar productions.
 */

foreignMetadataItem
    : foreignMetadata
    ;


/*
 * ============================================================================
 * 12. CONTRACT METADATA
 * ============================================================================
 *
 * This named composition point makes it explicit that foreign metadata may
 * carry information consumed by contracts, policies, capabilities, resources,
 * effects, and provenance.
 *
 * It remains syntactically identical to ordinary foreign metadata.
 *
 * The distinction is semantic, not grammatical.
 */

foreignContractMetadata
    : foreignMetadataBlock
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] It owns reusable foreign identity syntax.
 *
 * [x] It owns reusable foreign source references.
 *
 * [x] It owns reusable foreign target references.
 *
 * [x] It owns reusable foreign implementation references.
 *
 * [x] It owns reusable foreign metadata syntax.
 *
 * [x] It does not own foreign function declarations.
 *
 * [x] It does not own FFI calls.
 *
 * [x] It does not own ABI syntax.
 *
 * [x] It does not own foreign type syntax.
 *
 * [x] It does not own calling conventions.
 *
 * [x] It does not redefine identifiers.
 *
 * [x] It does not redefine qualified names.
 *
 * [x] It does not redefine expressions.
 *
 * [x] It does not redefine attributes.
 *
 * [x] It uses the canonical ZamaniLexer.
 *
 * [x] It introduces no parser-local lexer rules.
 *
 * [x] It introduces no embedded Rust.
 *
 * [x] It introduces no semantic predicates.
 *
 * [x] It performs no runtime behavior.
 *
 * [x] It performs no filesystem access.
 *
 * [x] It performs no network access.
 *
 * [x] It performs no hardware discovery.
 *
 * [x] It performs no foreign-code execution.
 *
 * [x] It introduces no target-specific ABI assumptions.
 *
 * [x] It introduces no fixed hardware capacities.
 *
 * [x] It supports open-world foreign identities.
 *
 * [x] It supports open-world metadata namespaces.
 *
 * [x] It supports arbitrary metadata cardinality through repetition.
 *
 * [x] It preserves POCO-REAF.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. FOREIGN FUNCTIONS
 * --------------------
 *
 * `grammar/interoperability/foreign-functions.g4` currently contains local
 * rules for:
 *
 *     interoperabilityForeignSource
 *     interoperabilityForeignMetadata
 *
 * Those reusable rules should be moved to this grammar.
 *
 * After integration, foreign-functions.g4 should import:
 *
 *     InteroperabilityForeign
 *
 * and use:
 *
 *     foreignSourceReference
 *     foreignMetadata
 *
 * or the repository's chosen compatibility aliases.
 *
 * The foreign-functions grammar must retain ownership of:
 *
 *     foreign callable declaration syntax.
 *
 *
 * 2. FFI
 * ------
 *
 * `grammar/interoperability/ffi.g4` remains the owner of FFI boundary
 * behavior.
 *
 * It may consume:
 *
 *     foreignSourceReference
 *     foreignIdentity
 *     foreignTargetReference
 *     foreignImplementationReference
 *     foreignMetadataBlock
 *
 * It MUST NOT recreate those rules.
 *
 *
 * 3. ABI
 * ------
 *
 * `grammar/interoperability/abi.g4` remains the ABI authority.
 *
 * ABI may consume foreign identities and metadata where needed.
 *
 * ABI MUST NOT redefine:
 *
 *     foreign source;
 *     foreign identity;
 *     generic foreign metadata.
 *
 *
 * 4. FOREIGN TYPES
 * ----------------
 *
 * `grammar/interoperability/foreign-types.g4` remains the foreign-type
 * authority.
 *
 * It may consume:
 *
 *     foreignSourceReference
 *     foreignMetadataBlock
 *
 * when a foreign type requires external-source metadata.
 *
 *
 * 5. CALLING CONVENTIONS
 * ----------------------
 *
 * `grammar/interoperability/calling-conventions.g4` remains the calling-
 * convention authority.
 *
 * This file does not enumerate calling conventions.
 *
 *
 * 6. INTEROPERABILITY COMPOSITION
 * ------------------------------
 *
 * `grammar/interoperability/interoperability.g4` is the composition root.
 *
 * It should import:
 *
 *     InteroperabilityForeign
 *
 * exactly once through the appropriate composition chain.
 *
 * It should not copy any foreign rule into its own body.
 *
 *
 * 7. EFFECTS
 * ----------
 *
 * `grammar/effects/foreign.g4` remains an EFFECT adapter.
 *
 * It must not import this grammar merely to determine whether an effect is
 * foreign.
 *
 * The relationship is semantic:
 *
 *     foreign boundary
 *             |
 *             v
 *     interoperability semantic model
 *             |
 *             v
 *     effect analysis
 *
 * This preserves the existing separation between:
 *
 *     foreign interoperability
 *
 * and:
 *
 *     foreign effects.
 *
 *
 * 8. TARGET LOWERING
 * ------------------
 *
 * No backend should consume this grammar directly as an execution command.
 *
 * The intended path is:
 *
 *     foreign syntax
 *         |
 *         v
 *     AST
 *         |
 *         v
 *     semantic foreign boundary
 *         |
 *         +--> effects
 *         +--> capabilities
 *         +--> resources
 *         +--> contracts
 *         +--> policies
 *         +--> provenance
 *         +--> ABI
 *         +--> FFI
 *         |
 *         v
 *     canonical semantic model
 *         |
 *         +--> classical IR
 *         +--> quantum::ir
 *         +--> HDL/hardware semantic lowering
 *         |
 *         v
 *     target-specific lowering
 *
 *
 * 9. QUANTUM
 * ----------
 *
 * If a foreign boundary supplies or consumes quantum computation:
 *
 *     foreign
 *       |
 *       v
 *     semantic quantum operation
 *       |
 *       v
 *     quantum::ir
 *
 * No foreign quantum IR is permitted.
 *
 *
 * 10. HDL / HARDWARE
 * ------------------
 *
 * If a foreign boundary represents HDL or hardware interaction:
 *
 *     foreign
 *       |
 *       v
 *     hardware/HDL semantic model
 *       |
 *       v
 *     target/resource/capability analysis
 *
 * No fixed hardware capacity belongs here.
 *
 *
 * 11. SECURITY
 * ------------
 *
 * Foreign metadata MUST NOT grant authorization.
 *
 * A declaration describing:
 *
 *     capability
 *     permission
 *     trust
 *     provider
 *
 * is not itself an authorization decision.
 *
 * Security/policy analysis remains authoritative.
 *
 *
 * 12. PROVENANCE
 * --------------
 *
 * Foreign identity, source, provider, implementation, version, and metadata
 * should be available to provenance analysis.
 *
 * The grammar preserves the information.
 *
 * The semantic/provenance layer determines its provenance meaning.
 *
 * ============================================================================
 * REQUIRED FOREIGN-FUNCTIONS INTEGRATION
 * ============================================================================
 *
 * The existing foreign-functions.g4 contains the following conceptual
 * structure:
 *
 *     EXTERN
 *     interoperabilityForeignSource?
 *     interoperabilityForeignMetadata*
 *
 * The one-time integration should become conceptually:
 *
 *     EXTERN
 *     foreignSourceReference?
 *     foreignMetadata*
 *
 * This avoids two grammar authorities for exactly the same concepts.
 *
 * The existing foreign callable declaration remains unchanged in ownership.
 *
 * ============================================================================
 * REQUIRED TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *     extern "C" ...
 *
 *     extern "Fortran" ...
 *
 *     extern vendor::runtime ...
 *
 *     foreign.language = "C";
 *
 *     foreign.provider = vendor::runtime;
 *
 *     foreign.target = simulator::quantum;
 *
 *     vendor::extension = expression;
 *
 * Negative tests:
 *
 *     malformed foreign source;
 *     malformed metadata assignment;
 *     missing metadata value;
 *     missing semicolon;
 *     malformed qualified metadata key;
 *     unbalanced metadata block.
 *
 * Boundary tests:
 *
 *     foreign + FFI;
 *     foreign + ABI;
 *     foreign + foreign type;
 *     foreign + calling convention;
 *     foreign + effects;
 *     foreign + capability;
 *     foreign + resource requirement;
 *     foreign + contract;
 *     foreign + policy;
 *     foreign + provenance;
 *     foreign + quantum;
 *     foreign + HDL;
 *     foreign + distributed execution.
 *
 * Scalability tests:
 *
 *     arbitrarily many metadata entries;
 *     arbitrarily many namespaced metadata keys;
 *     deeply qualified symbolic identities;
 *     large foreign-interface compositions;
 *     large generated interoperability declarations;
 *     large cross-domain programs.
 *
 * Determinism tests:
 *
 *     identical source -> identical parse structure;
 *     metadata order preserved;
 *     source order preserved;
 *     no hardware-dependent parsing;
 *     no environment-dependent parsing;
 *     no network-dependent parsing.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file describes FOREIGN INTENT.
 *
 * It does not describe FOREIGN EXECUTION.
 *
 * Therefore:
 *
 *     syntax != resolution
 *     syntax != authorization
 *     syntax != linking
 *     syntax != loading
 *     syntax != ABI lowering
 *     syntax != resource allocation
 *     syntax != target selection
 *     syntax != runtime execution
 *
 * The foreign boundary remains portable, symbolic, open-world, and
 * target-independent.
 *
 * ============================================================================
 */