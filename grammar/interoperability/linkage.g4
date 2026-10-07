/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/linkage.g4
 *
 * Grammar:
 *     InteroperabilityLinkage
 *
 * Status:
 *     CANONICAL SOURCE-LEVEL LINKAGE CONTRACT
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE
 *
 * Architectural objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the reusable SOURCE-LEVEL SYNTAX for describing linkage
 * intent at an interoperability boundary.
 *
 * Linkage describes how a declaration, symbol, callable, object, module,
 * interface, or other externally visible entity participates in a linking
 * relationship.
 *
 * This grammar represents LINKAGE INTENT.
 *
 * It does NOT implement linking.
 *
 * It does not resolve symbols, load objects, inspect binaries, select a
 * linker, select a platform, inspect hardware, or determine a physical
 * calling sequence.
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
 *     ZamaniParser
 *          |
 *          v
 *     linkage syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic linkage model
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *         ABI                     interoperability
 *       analysis                    analysis
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                canonical semantic model
 *                        |
 *                        v
 *                  canonical IR
 *                        |
 *                        v
 *                target-independent
 *                     lowering
 *                        |
 *                        v
 *                object/link realization
 *                        |
 *                        v
 *                    target
 *
 * Linkage therefore sits BETWEEN source-level interoperability intent and
 * downstream ABI/linker realization.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     interoperabilityLinkageReference
 *     interoperabilityLinkageClause
 *     interoperabilityLinkageDeclaration
 *     interoperabilityLinkageAttachment
 *     interoperabilityLinkageMetadata
 *
 * It owns the syntax:
 *
 *     linkage = "external";
 *
 *     linkage = platform::external;
 *
 *     linkage = vendor::linkage::model;
 *
 *     linkage = custom_linkage;
 *
 * The values remain symbolic.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     token definitions
 *     identifiers
 *     qualified names
 *     attributes
 *     ordinary declarations
 *     ordinary functions
 *     foreign-function declarations
 *     FFI declarations
 *     ABI declarations
 *     calling conventions
 *     data layouts
 *     symbol resolution
 *     object-file generation
 *     dynamic loading
 *     static linking implementation
 *     dynamic linking implementation
 *     linker implementation
 *     loader implementation
 *     library resolution
 *     package resolution
 *     target selection
 *     hardware discovery
 *     physical addresses
 *     register allocation
 *     stack layout
 *     memory layout
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Linkage is distinct from:
 *
 *     calling convention
 *     ABI
 *     data layout
 *     symbol identity
 *     visibility
 *     FFI
 *     foreign language
 *
 * The semantic relationship is:
 *
 *     linkage
 *         +
 *     symbol identity
 *         +
 *     ABI
 *         +
 *     calling convention
 *         +
 *     data-layout contract
 *         |
 *         v
 *     interoperability realization
 *
 * None of these concepts should be silently substituted for another.
 *
 * In particular:
 *
 *     linkage != calling convention
 *
 *     linkage != ABI
 *
 *     linkage != visibility
 *
 *     linkage != symbol name
 *
 *     linkage != library name
 *
 *     linkage != target architecture
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The parser-facing lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT introduce dedicated universal tokens such as:
 *
 *     LINKAGE
 *     EXTERNAL
 *     INTERNAL
 *     WEAK
 *     STRONG
 *     STATIC
 *     DYNAMIC
 *     IMPORT
 *     EXPORT
 *     COMMON
 *
 * Linkage vocabularies are open-ended semantic data.
 *
 * This prevents the language from becoming a closed enumeration of current
 * object formats, operating systems, toolchains, vendors, ABIs, or future
 * computational substrates.
 *
 * ============================================================================
 * NAME AUTHORITY
 * ============================================================================
 *
 * Name syntax belongs to:
 *
 *     grammar/core/names.g4
 *
 * This file imports:
 *
 *     Names
 *
 * and reuses:
 *
 *     identifier
 *     qualifiedName
 *
 * It MUST NOT redefine either.
 *
 * ============================================================================
 * EXPRESSION AUTHORITY
 * ============================================================================
 *
 * This file does not define the general expression language.
 *
 * Expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * Linkage references intentionally use a restricted symbolic representation
 * rather than arbitrary executable expressions.
 *
 * This distinction is important:
 *
 *     linkage = platform::external;
 *
 * identifies semantic linkage metadata.
 *
 * It does not mean:
 *
 *     evaluate an arbitrary runtime expression to choose a linker.
 *
 * ============================================================================
 * OPEN-WORLD LINKAGE MODEL
 * ============================================================================
 *
 * The grammar does not enumerate linkage models.
 *
 * Examples that are syntactically representable include:
 *
 *     linkage = "external";
 *     linkage = "internal";
 *     linkage = "weak";
 *     linkage = "strong";
 *     linkage = "static";
 *     linkage = "dynamic";
 *     linkage = "import";
 *     linkage = "export";
 *
 * and symbolic identities such as:
 *
 *     linkage = platform::external;
 *     linkage = platform::internal;
 *     linkage = vendor::linkage::extension;
 *     linkage = future::linkage::model;
 *     linkage = custom_linkage;
 *
 * These examples are DATA.
 *
 * They are deliberately NOT grammar alternatives.
 *
 * A new linkage model therefore does not require changing this file.
 *
 * Semantic analysis determines whether a referenced linkage model:
 *
 *     exists
 *     is supported
 *     is deprecated
 *     is aliased
 *     is incompatible
 *     is target-dependent
 *     is permitted
 *     requires a capability
 *     requires a particular ABI
 *     requires a particular object model
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Linkage syntax must remain portable.
 *
 * It MUST NOT encode a particular physical realization.
 *
 * The grammar therefore does not contain:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     ASIC identifiers
 *     QPU identifiers
 *     node identifiers
 *     core identifiers
 *     thread identifiers
 *     register identifiers
 *     physical addresses
 *     memory-bank identifiers
 *     physical-qubit identifiers
 *
 * A linkage declaration may therefore participate in compilation for:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future targets
 *
 * provided the downstream target can satisfy the semantic contract.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-defined ceiling on:
 *
 *     linkage declarations
 *     linkage references
 *     declaration count
 *     module count
 *     function count
 *     interface count
 *     foreign declarations
 *     ABI contracts
 *     symbols
 *     namespaces
 *     qualification depth
 *     source size
 *
 * There are no:
 *
 *     MAX_LINKAGES
 *     MAX_SYMBOLS
 *     MAX_MODULES
 *     MAX_FUNCTIONS
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *
 * Practical limits imposed by a compiler implementation are implementation
 * resource limits and MUST NOT become language semantics.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-defined ceiling.
 *
 * ============================================================================
 * SYMBOLIC VERSUS PHYSICAL LINKAGE
 * ============================================================================
 *
 * Source-level linkage is symbolic.
 *
 * For example:
 *
 *     linkage = "external";
 *
 * does NOT itself specify:
 *
 *     a shared library
 *     an object file
 *     a filesystem path
 *     a linker executable
 *     a loader
 *     an operating system
 *     an architecture
 *     an instruction set
 *     a memory address
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * LINKAGE VERSUS SYMBOL IDENTITY
 * ============================================================================
 *
 * Linkage and symbol identity are separate.
 *
 * For example:
 *
 *     linkage = "external";
 *     symbol = "compute";
 *
 * means two distinct pieces of metadata:
 *
 *     linkage
 *         -> how the entity participates in linking
 *
 *     symbol
 *         -> which external symbol is intended
 *
 * This file owns only the first.
 *
 * Symbol naming/identity remains owned by the relevant interoperability or
 * ABI grammar.
 *
 * ============================================================================
 * LINKAGE VERSUS VISIBILITY
 * ============================================================================
 *
 * Linkage and visibility are also separate.
 *
 * Linkage describes participation in a linking relationship.
 *
 * Visibility describes whether/how an entity is exposed.
 *
 * A source declaration therefore MUST NOT infer:
 *
 *     visibility
 *
 * merely from:
 *
 *     linkage
 *
 * unless the semantic ABI specification explicitly defines such a relationship
 * for a particular linkage model.
 *
 * ============================================================================
 * LINKAGE VERSUS CALLING CONVENTION
 * ============================================================================
 *
 * A calling convention controls a callable interoperability convention.
 *
 * Linkage controls linking participation.
 *
 * Therefore:
 *
 *     linkage = "external";
 *
 * does NOT imply:
 *
 *     calling_convention = "c";
 *
 * or any other convention.
 *
 * The calling-convention grammar remains:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * ============================================================================
 * LINKAGE VERSUS ABI
 * ============================================================================
 *
 * ABI is the broader compatibility boundary.
 *
 * Linkage is one dimension of that boundary.
 *
 * The ABI grammar remains:
 *
 *     grammar/interoperability/abi.g4
 *
 * This file supplies reusable linkage syntax to that ABI layer.
 *
 * ============================================================================
 * LINKAGE VERSUS DATA LAYOUT
 * ============================================================================
 *
 * Data layout is separate from linkage.
 *
 * The data-layout grammar owns:
 *
 *     representation
 *     size
 *     alignment
 *     field arrangement
 *     encoding
 *     aggregate layout intent
 *
 * Linkage does not determine these properties.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing linkage MUST depend only on:
 *
 *     source text
 *     canonical token stream
 *     grammar version
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     operating-system state
 *     filesystem state
 *     network state
 *     environment variables
 *     installed libraries
 *     linker state
 *     loader state
 *     target availability
 *     runtime state
 *     scheduler state
 *     quantum calibration
 *
 * Whether a linkage model can be realized is determined later.
 *
 * ============================================================================
 * SAFETY / INERTNESS
 * ============================================================================
 *
 * Parsing linkage MUST be completely inert.
 *
 * This grammar MUST NOT:
 *
 *     load libraries
 *     resolve symbols
 *     inspect object files
 *     invoke a linker
 *     invoke a loader
 *     open a filesystem path
 *     access a network
 *     inspect hardware
 *     execute foreign code
 *     execute generated code
 *     allocate native memory
 *     access physical addresses
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no semantic predicates
 *     no embedded code
 *     no filesystem operations
 *     no network operations
 *     no runtime operations
 *     no hardware discovery
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar produces parser contexts only.
 *
 * Frontend AST construction belongs to the existing domain-neutral AST.
 *
 * Conceptual representation:
 *
 *     linkage = symbolic-reference
 *
 * maps to generic declaration/function/interoperability metadata such as:
 *
 *     Metadata {
 *         key: "linkage",
 *         value: SymbolicReference(...)
 *     }
 *
 * or the repository's equivalent canonical metadata representation.
 *
 * This grammar MUST NOT require AST variants such as:
 *
 *     CLinkage
 *     ELFLinkage
 *     MachOLinkage
 *     PELinkage
 *     WindowsLinkage
 *     LinuxLinkage
 *     CpuLinkage
 *     GpuLinkage
 *     QpuLinkage
 *
 * The AST remains domain-neutral.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only syntactic structure.
 *
 * Semantic analysis must determine:
 *
 *     - whether the metadata key is `linkage`;
 *     - whether the reference is valid;
 *     - whether the linkage model exists;
 *     - whether the linkage model is permitted;
 *     - whether the linkage model is compatible with the enclosing ABI;
 *     - whether the declaration kind permits the linkage;
 *     - whether duplicate linkage declarations conflict;
 *     - whether required capabilities are available;
 *     - whether required resources are available;
 *     - whether security policy permits the boundary;
 *     - whether target realization is possible.
 *
 * An unknown future linkage identity is not automatically a parser error.
 *
 * It may instead be:
 *
 *     unresolved
 *     unsupported
 *     target-dependent
 *     extension-defined
 *
 * until semantic resolution.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Linkage syntax itself has no runtime effect.
 *
 * It MUST NOT automatically imply:
 *
 *     IO
 *     network
 *     native execution
 *     foreign execution
 *     filesystem access
 *     process creation
 *     dynamic loading
 *
 * If a concrete interoperability operation has such effects, those effects
 * are described through the canonical effects subsystem.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Linkage syntax does not grant capabilities.
 *
 * For example:
 *
 *     linkage = "dynamic";
 *
 * does NOT automatically grant:
 *
 *     capability("dynamic.load")
 *
 * or:
 *
 *     capability("filesystem.read")
 *
 * Capability requirements remain owned by:
 *
 *     grammar/resources/
 *     grammar/security/
 *
 * and their semantic layers.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Linkage syntax does not allocate resources.
 *
 * It MUST NOT encode:
 *
 *     processor count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     memory size
 *     register count
 *     register width
 *     device count
 *
 * Resource requirements remain separate semantic constructs.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A policy may permit, prohibit, constrain, or require a linkage model.
 *
 * Policy evaluation belongs downstream.
 *
 * For example, a security policy may prohibit dynamic external loading.
 *
 * That does not make:
 *
 *     linkage = "dynamic";
 *
 * syntactically invalid.
 *
 * It makes the declaration semantically or policy-invalid in that context.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend should preserve:
 *
 *     original linkage spelling
 *     canonical metadata key
 *     source span
 *     language version
 *     compatibility context
 *
 * Semantic resolution may additionally record:
 *
 *     canonical linkage identity
 *     aliases
 *     extension provider
 *     ABI relationship
 *     target compatibility
 *     diagnostics
 *     lowering decisions
 *
 * This grammar itself does not generate provenance.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The metadata key:
 *
 *     linkage
 *
 * is part of the source compatibility surface.
 *
 * Its meaning MUST remain stable.
 *
 * Linkage identities are open-world values and may be versioned by the
 * compatibility system.
 *
 * A future linkage extension should not require a parser modification merely
 * because a new symbolic linkage identity was introduced.
 *
 * ============================================================================
 * TARGET LOWERING CONTRACT
 * ============================================================================
 *
 * Target lowering may determine concrete implementation details such as:
 *
 *     object format
 *     symbol binding
 *     symbol visibility interaction
 *     section placement
 *     relocation model
 *     static/dynamic realization
 *     import/export realization
 *     linker arguments
 *     loader behavior
 *     thunk generation
 *     platform-specific symbol treatment
 *
 * None of these decisions belong to this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO standalone IR.
 *
 * It MUST NOT create:
 *
 *     LinkageIR
 *     AbiLinkageIR
 *     ForeignLinkageIR
 *     PlatformLinkageIR
 *     QuantumLinkageIR
 *     HardwareLinkageIR
 *
 * If canonical IR needs linkage information, linkage is metadata on the
 * existing canonical declaration/callable/external-interface representation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Linkage is domain-neutral.
 *
 * A classical/quantum boundary may require linkage metadata, but this grammar
 * does not define a quantum linkage vocabulary.
 *
 * It MUST NOT enumerate:
 *
 *     QPU linkages
 *     physical qubits
 *     gate sets
 *     coupling maps
 *     routing strategies
 *     calibration data
 *     QEC configurations
 *
 * If quantum semantics are required, the canonical path remains:
 *
 *     source
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC/resilience
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware boundaries may carry linkage metadata.
 *
 * This grammar does not define:
 *
 *     signal widths
 *     register widths
 *     bus widths
 *     physical pins
 *     device identifiers
 *     FPGA-specific primitive names
 *     ASIC-specific resources
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A distributed callable/service boundary may carry linkage metadata.
 *
 * This does not make linkage a networking construct.
 *
 * Network effects, endpoints, protocols, discovery, authentication, and
 * transport remain owned by their respective subsystems.
 *
 * No node-count limit is introduced.
 *
 * ============================================================================
 * FFI INTEGRATION
 * ============================================================================
 *
 * FFI may consume:
 *
 *     interoperabilityLinkageClause
 *
 * Foreign-function declarations remain owned by:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * and:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Those grammars MUST NOT create a competing linkage syntax.
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * ABI is the principal semantic consumer of this grammar.
 *
 * The intended composition is:
 *
 *     Abi
 *       |
 *       +--> interoperabilityLinkageDeclaration
 *       |
 *       +--> calling-convention reference
 *       |
 *       +--> data-layout contract
 *       |
 *       +--> symbol identity
 *       |
 *       +--> other ABI metadata
 *
 * ABI validation then determines whether the combined contract is coherent.
 *
 * ============================================================================
 * CALLING-CONVENTION INTEGRATION
 * ============================================================================
 *
 * The calling-convention grammar remains:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Its canonical syntax is:
 *
 *     calling_convention = "..."
 *
 * Linkage remains:
 *
 *     linkage = "..."
 *
 * These must remain separate metadata dimensions.
 *
 * ============================================================================
 * DATA-LAYOUT INTEGRATION
 * ============================================================================
 *
 * The data-layout grammar remains a separate leaf:
 *
 *     grammar/interoperability/data-layout.g4
 *
 * Linkage MUST NOT define:
 *
 *     size
 *     alignment
 *     offset
 *     padding
 *     field order
 *     encoding
 *     byte order
 *     pointer representation
 *
 * The ABI layer combines the two when required.
 *
 * ============================================================================
 * SYMBOL INTEGRATION
 * ============================================================================
 *
 * Linkage may coexist with symbol metadata:
 *
 *     linkage = "external";
 *     symbol = "compute";
 *
 * The symbol's identity is resolved elsewhere.
 *
 * This grammar does not define the `symbol` property.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Linkage that eventually results in dynamic loading, native execution, or
 * external calls may require security authorization.
 *
 * The grammar itself does not authorize any of these operations.
 *
 * The security pipeline is:
 *
 *     linkage intent
 *         ->
 *     semantic analysis
 *         ->
 *     capability analysis
 *         ->
 *     policy analysis
 *         ->
 *     target/runtime authorization
 *
 * ============================================================================
 * RESOURCE / CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * Linkage may be associated with requirements such as:
 *
 *     requires capability("foreign.link");
 *
 * or:
 *
 *     requires capability("dynamic.link");
 *
 * The requirement syntax belongs to the canonical resource/contract systems.
 *
 * This grammar does not redefine `requires`.
 *
 * ============================================================================
 * ERROR CLASSIFICATION
 * ============================================================================
 *
 * STRUCTURAL / PARSER ERRORS
 * --------------------------
 *
 * Examples:
 *
 *     linkage
 *
 *     linkage =
 *
 *     linkage = ;
 *
 *     linkage = platform::
 *
 *     linkage = ::
 *
 *     linkage = "a" "b";
 *
 *     linkage = ();
 *
 *     linkage = {};
 *
 * SEMANTIC ERRORS
 * ---------------
 *
 * Examples:
 *
 *     wrong_key = "external";
 *
 *     conflicting linkage declarations
 *
 *     unsupported linkage identity
 *
 *     linkage incompatible with declaration kind
 *
 *     linkage incompatible with ABI
 *
 * TARGET / REALIZATION ERRORS
 * ---------------------------
 *
 * Examples:
 *
 *     linkage model unavailable on target
 *
 *     required linker capability unavailable
 *
 *     required object model unavailable
 *
 *     policy prohibits requested linkage realization
 *
 * These error classes MUST remain distinguishable.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must accept the following forms at a valid metadata
 * boundary:
 *
 *     linkage = "external";
 *
 *     linkage = "internal";
 *
 *     linkage = "weak";
 *
 *     linkage = "strong";
 *
 *     linkage = "static";
 *
 *     linkage = "dynamic";
 *
 *     linkage = platform::external;
 *
 *     linkage = vendor::linkage::extension;
 *
 *     linkage = future::linkage::model;
 *
 *     linkage = custom_linkage;
 *
 *     linkage = custom::linkage;
 *
 * Deep symbolic qualification must remain supported.
 *
 * No fixed qualification depth is introduced.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must reject or semantically diagnose:
 *
 *     linkage
 *
 *     linkage =
 *
 *     linkage = ;
 *
 *     linkage = platform::
 *
 *     linkage = ::
 *
 *     linkage = "a" "b";
 *
 *     linkage = ();
 *
 *     linkage = {};
 *
 *     linkage = platform::;
 *
 *     linkage = ::platform;
 *
 * A wrong metadata key such as:
 *
 *     calling_convention = "external";
 *
 * is NOT a linkage declaration.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test linkage together with:
 *
 *     ordinary functions
 *     foreign functions
 *     FFI
 *     ABI declarations
 *     calling conventions
 *     data-layout contracts
 *     symbol metadata
 *     visibility metadata
 *     effects
 *     capabilities
 *     resource requirements
 *     security policies
 *     classical computation
 *     quantum/classical boundaries
 *     HDL/software boundaries
 *     accelerator boundaries
 *     distributed boundaries
 *     dialect extensions
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The test suite must verify that the grammar remains structurally valid for:
 *
 *     long linkage identities
 *     deeply qualified linkage identities
 *     many declarations
 *     many independent linkage-bearing entities
 *     many ABI contracts
 *     many foreign boundaries
 *     many modules
 *     large source units
 *
 * Tests must not encode artificial maxima.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The same:
 *
 *     source
 *     grammar version
 *     lexer configuration
 *
 * must produce the same parse structure regardless of:
 *
 *     target machine
 *     available libraries
 *     operating system
 *     network state
 *     hardware state
 *     linker installation
 *     runtime state
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * IDE/LSP/formatter tooling must be able to identify:
 *
 *     linkage key
 *     assignment operator
 *     linkage reference
 *     complete linkage clause
 *
 * Tooling must not require a closed enumeration of linkage identities.
 *
 * An unknown linkage identity may receive a semantic diagnostic without being
 * treated as a parser grammar failure.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Names
 *
 * EXPORTS:
 *
 *     interoperabilityLinkageReference
 *     interoperabilityLinkageClause
 *     interoperabilityLinkageDeclaration
 *     interoperabilityLinkageAttachment
 *     interoperabilityLinkageMetadata
 *
 * CONSUMED_BY:
 *
 *     grammar/interoperability/abi.g4
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/foreign-functions.g4
 *     grammar/interoperability/calling-conventions.g4 consumers
 *     interoperability semantic analysis
 *     AST construction
 *     validation
 *     compiler/lowering
 *     tooling
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     interoperability / ABI semantic analysis
 *
 * IR_OWNER:
 *
 *     existing canonical declaration/callable/external-interface IR metadata
 *
 * TEST_OWNER:
 *
 *     grammar/tests/interoperability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/compatibility/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/interoperability.md
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. ABI
 *
 *     grammar/interoperability/abi.g4
 *
 *     MUST import:
 *
 *         InteroperabilityLinkage
 *
 *     Its ABI member dispatcher should consume:
 *
 *         interoperabilityLinkageDeclaration
 *
 *     before the generic open-world ABI property fallback.
 *
 *     This gives linkage a stable semantic owner while preserving the ABI
 *     grammar as the broader ABI composition authority.
 *
 * 2. FFI
 *
 *     grammar/interoperability/ffi.g4
 *
 *     MAY consume:
 *
 *         interoperabilityLinkageDeclaration
 *
 *     where an FFI boundary explicitly permits linkage metadata.
 *
 *     It MUST NOT redefine:
 *
 *         linkage = ...
 *
 * 3. FOREIGN FUNCTIONS
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 *     MAY consume:
 *
 *         interoperabilityLinkageDeclaration
 *
 *     as part of its external declaration metadata.
 *
 * 4. CALLING CONVENTIONS
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 *     MUST remain independent.
 *
 *     It MUST NOT import linkage merely to implement calling-convention
 *     syntax.
 *
 *     The semantic model combines:
 *
 *         linkage
 *         calling convention
 *         ABI
 *
 *     downstream.
 *
 * 5. DATA LAYOUT
 *
 *     grammar/interoperability/data-layout.g4
 *
 *     remains independent.
 *
 *     ABI semantic analysis combines linkage and layout when necessary.
 *
 * 6. SYMBOLS
 *
 *     Symbol identity remains owned by its existing interoperability/ABI
 *     metadata grammar.
 *
 * 7. ROOT COMPOSITION
 *
 *     grammar/Zamani.g4
 *
 *     MUST NOT import this file directly.
 *
 *     Composition remains:
 *
 *         Zamani.g4
 *             ->
 *         ZamaniParser.g4
 *             ->
 *         Interoperability
 *             ->
 *         interoperability leaves
 *
 * ============================================================================
 * INTEGRATION ORDER
 * ============================================================================
 *
 * This file is intentionally a leaf grammar.
 *
 * Correct integration order:
 *
 *     1. InteroperabilityLinkage
 *             |
 *             v
 *     2. ABI composition
 *             |
 *             v
 *     3. FFI / foreign-function consumers
 *             |
 *             v
 *     4. semantic linkage model
 *             |
 *             v
 *     5. validation
 *             |
 *             v
 *     6. canonical semantic model
 *             |
 *             v
 *     7. canonical IR metadata
 *             |
 *             v
 *     8. target lowering
 *             |
 *             v
 *     9. linker/runtime realization
 *
 * This avoids repeatedly modifying this leaf when downstream implementations
 * evolve.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MAX_LINKAGES
 *     MAX_SYMBOLS
 *     MAX_MODULES
 *     MAX_FUNCTIONS
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *
 * It also MUST NOT encode:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     ASIC 0
 *     QPU 0
 *     node 0
 *     register 0
 *     physical address
 *     fixed pointer width
 *     fixed word size
 *     fixed object format
 *
 * Linkage identities remain symbolic.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is an ANTLR grammar and therefore contains no Rust implementation.
 *
 * Rust code integrating this grammar MUST target:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and MUST use safe Rust.
 *
 * No unsafe Rust is required by this grammar.
 *
 * The grammar must not contain embedded Rust actions or predicates.
 *
 * ============================================================================
 * ANTLR GENERATION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Canonical declaration:
 *
 *     parser grammar InteroperabilityLinkage;
 *
 * Canonical lexer:
 *
 *     ZamaniLexer
 *
 * Canonical shared name grammar:
 *
 *     Names
 *
 * The grammar source path must contain:
 *
 *     grammar/core/names.g4
 *
 * and the ANTLR build must make the `Names` grammar available to this file.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It exists at:
 *
 *         grammar/interoperability/linkage.g4
 *
 *     [ ] It declares:
 *
 *         parser grammar InteroperabilityLinkage;
 *
 *     [ ] It uses:
 *
 *         tokenVocab = ZamaniLexer;
 *
 *     [ ] It imports only the canonical name grammar required for linkage
 *         references.
 *
 *     [ ] It does not define lexer rules.
 *
 *     [ ] It does not enumerate linkage models.
 *
 *     [ ] It supports quoted symbolic linkage identities.
 *
 *     [ ] It supports qualified symbolic linkage identities.
 *
 *     [ ] It supports open-world future linkage identities.
 *
 *     [ ] It does not define ABI semantics.
 *
 *     [ ] It does not define calling-convention semantics.
 *
 *     [ ] It does not define data-layout semantics.
 *
 *     [ ] It does not define symbol-resolution behavior.
 *
 *     [ ] It does not define linker behavior.
 *
 *     [ ] It does not define loader behavior.
 *
 *     [ ] It does not inspect hardware.
 *
 *     [ ] It does not allocate resources.
 *
 *     [ ] It does not execute code.
 *
 *     [ ] It introduces no unsafe Rust.
 *
 *     [ ] It introduces no fixed hardware limits.
 *
 *     [ ] It introduces no target-specific constants.
 *
 *     [ ] It preserves source spans through the existing AST pipeline.
 *
 *     [ ] It has semantic diagnostics distinct from parser diagnostics.
 *
 *     [ ] ABI integration consumes the dedicated linkage rule before generic
 *         ABI metadata.
 *
 *     [ ] FFI integration does not duplicate linkage syntax.
 *
 *     [ ] Foreign-function integration does not duplicate linkage syntax.
 *
 *     [ ] Calling-convention syntax remains independent.
 *
 *     [ ] Data-layout syntax remains independent.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 * ============================================================================
 * GRAMMAR RULES
 * ============================================================================
 */

parser grammar InteroperabilityLinkage;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names
;


/*
 * ============================================================================
 * 1. SYMBOLIC LINKAGE REFERENCE
 * ============================================================================
 *
 * A linkage identity may be represented as:
 *
 *     "external"
 *
 * or:
 *
 *     platform::external
 *
 * or:
 *
 *     vendor::extension::linkage
 *
 * or:
 *
 *     custom_linkage
 *
 * No linkage values are enumerated.
 *
 * This rule deliberately excludes arbitrary expressions.
 *
 * Linkage is metadata, not executable computation.
 */
interoperabilityLinkageReference
    : STRING_LITERAL
    | qualifiedName
    ;


/*
 * ============================================================================
 * 2. CANONICAL LINKAGE CLAUSE
 * ============================================================================
 *
 * Canonical source form:
 *
 *     linkage = "external";
 *
 *     linkage = platform::external;
 *
 * The metadata key remains an identifier rather than a globally reserved
 * keyword.
 *
 * Semantic analysis must validate that the key is exactly:
 *
 *     linkage
 *
 * This avoids adding another global lexer keyword solely for interoperability
 * metadata.
 */
interoperabilityLinkageClause
    : identifier
      ASSIGN
      interoperabilityLinkageReference
    ;


/*
 * ============================================================================
 * 3. COMPLETE DECLARATION-LEVEL ATTACHMENT
 * ============================================================================
 *
 * This form includes its own terminator and can be consumed by declaration
 * metadata boundaries that expect complete clauses.
 */
interoperabilityLinkageDeclaration
    : interoperabilityLinkageClause
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. REUSABLE ATTACHMENT
 * ============================================================================
 *
 * This form deliberately excludes the terminator so an enclosing grammar can
 * determine declaration-boundary ownership.
 */
interoperabilityLinkageAttachment
    : interoperabilityLinkageClause
    ;


/*
 * ============================================================================
 * 5. METADATA ALIAS
 * ============================================================================
 *
 * Stable composition alias for interoperability grammars that need to attach
 * linkage metadata without depending on whether the enclosing grammar owns the
 * terminator.
 */
interoperabilityLinkageMetadata
    : interoperabilityLinkageClause
    ;