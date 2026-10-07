/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/external-foreign-functions.g4
 *
 * Grammar:
 *     ExternalForeignFunctions
 *
 * Status:
 *     CANONICAL EXTERNAL FOREIGN-CALLABLE DECLARATION GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     SAFE RUST ONLY
 *     NO UNSAFE
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns the SOURCE-LEVEL SYNTAX of one externally implemented
 * callable declaration.
 *
 * It is intentionally a LEAF grammar.
 *
 * The surrounding external-interface grammar owns:
 *
 *     extern
 *     source identity
 *     interface/block framing
 *
 * This file owns:
 *
 *     externalForeignFunction
 *     externalForeignFunctionSignature
 *     externalForeignFunctionParameterBoundary
 *     externalForeignFunctionMetadata
 *
 * The declaration describes an externally implemented callable contract.
 *
 * It does NOT execute the callable.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     externalForeignFunction
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +------------------+
 *          |        |         |
 *          v        v         v
 *        types    effects   contracts
 *                   |         |
 *                   +----+----+
 *                        |
 *                        v
 *               interoperability
 *                 semantic model
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *        ABI          FFI/linkage   capabilities
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                canonical semantic IR
 *                        |
 *             +----------+----------+
 *             |          |          |
 *             v          v          v
 *        classical   quantum::ir   HDL/hardware
 *             |          |          |
 *             +----------+----------+
 *                        |
 *                        v
 *               target-independent
 *                   lowering
 *                        |
 *                        v
 *                target realization
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     externalForeignFunction
 *     externalForeignFunctionSignature
 *     externalForeignFunctionMetadata
 *     externalForeignFunctionMetadataValue
 *
 * It owns only the source syntax necessary to describe one external callable.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     extern-block framing
 *     library loading
 *     symbol resolution
 *     ABI definitions
 *     ABI layouts
 *     calling-convention definitions
 *     linkage definitions
 *     foreign-type definitions
 *     ordinary function definitions
 *     ordinary function bodies
 *     ordinary calls
 *     expression semantics
 *     type semantics
 *     effect semantics
 *     capability semantics
 *     resource semantics
 *     policy semantics
 *     security semantics
 *     provenance semantics
 *     target selection
 *     hardware discovery
 *     register allocation
 *     stack layout
 *     memory layout
 *     machine instruction selection
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * AUTHORITATIVE OWNERS
 * ============================================================================
 *
 * External interface framing:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * Generic FFI:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI:
 *
 *     grammar/interoperability/abi.g4
 *
 * Calling conventions:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Foreign types:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Attributes:
 *
 *     grammar/core/attributes.g4
 *
 * Parameters:
 *
 *     grammar/functions/parameters.g4
 *
 * Generic function parameters:
 *
 *     grammar/functions/generics.g4
 *     or the repository's canonical FunctionGenerics delegate
 *
 * Types:
 *
 *     grammar/types/
 *
 * Effects:
 *
 *     grammar/effects/
 *
 * Resources/capabilities:
 *
 *     grammar/resources/
 *
 * Contracts:
 *
 *     grammar/validation/
 *
 * Policies:
 *
 *     grammar/policies/
 *
 * Provenance:
 *
 *     grammar/spec/
 *     semantic provenance subsystem
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Names
 *     Attributes
 *     Parameters
 *     FunctionGenerics
 *     Types
 *
 * EXPORTS:
 *
 *     externalForeignFunction
 *     externalForeignFunctionSignature
 *     externalForeignFunctionMetadata
 *     externalForeignFunctionMetadataValue
 *
 * CONSUMED_BY:
 *
 *     grammar/interoperability/foreign-functions.g4
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/interoperability.g4
 *     interoperability semantic analysis
 *     AST construction
 *     validation
 *     compiler/lowering
 *     interoperability tooling
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     interoperability semantic analysis
 *
 * IR_OWNER:
 *
 *     existing canonical semantic model
 *
 *     No ExternalForeignFunctionIR is introduced.
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
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT define lexer rules.
 *
 * It MUST NOT introduce keywords for:
 *
 *     C
 *     C++
 *     Rust
 *     Python
 *     Fortran
 *     SystemVerilog
 *     OpenQASM
 *     vendor names
 *     operating systems
 *     CPU architectures
 *     GPU architectures
 *     ABI names
 *     calling conventions
 *     linkage kinds
 *     library names
 *     device names
 *
 * Those identities remain open-world symbolic data.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * An external callable is an INTERFACE CONTRACT.
 *
 * It does not prescribe:
 *
 *     a particular processor;
 *     a particular device;
 *     a particular library file;
 *     a particular memory capacity;
 *     a particular register width;
 *     a particular pointer width;
 *     a particular node;
 *     a particular thread count;
 *     a particular QPU;
 *     a particular physical qubit;
 *     a particular FPGA;
 *     a particular accelerator.
 *
 * The same declaration can therefore be realized differently at different
 * scales when the semantic contract is satisfiable.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * The grammar deliberately does not enumerate:
 *
 *     external languages;
 *     ABIs;
 *     symbols;
 *     calling conventions;
 *     linkers;
 *     object formats;
 *     vendors;
 *     architectures;
 *     operating systems;
 *     deployment systems.
 *
 * New interoperability technologies can therefore be introduced without
 * changing this grammar when their source identity can be represented by the
 * existing symbolic/metadata model.
 *
 * ============================================================================
 * METADATA CONTRACT
 * ============================================================================
 *
 * External metadata is intentionally open-ended.
 *
 * A metadata key is a canonical Zamani qualified name.
 *
 * Examples include semantic keys such as:
 *
 *     symbol
 *     language
 *     abi
 *     linkage
 *     calling_convention
 *     representation
 *     variadic
 *     ownership
 *     lifetime
 *     nullability
 *     encoding
 *
 * These names are NOT hard-coded into this grammar.
 *
 * Their validity and meaning belong to the interoperability semantic schema.
 *
 * This prevents the grammar from becoming a finite catalogue of every
 * present and future foreign-function technology.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser creates parse-tree structure only.
 *
 * The semantic AST should preserve at least:
 *
 *     declaration name
 *     generic parameters
 *     parameters
 *     return type
 *     attributes
 *     metadata
 *     source spans
 *
 * Semantic analysis may then associate:
 *
 *     ABI
 *     linkage
 *     calling convention
 *     effects
 *     capabilities
 *     resource requirements
 *     contracts
 *     policies
 *     provenance
 *
 * with the declaration.
 *
 * The AST MUST remain domain-neutral.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Parameter and return types MUST use the canonical Zamani type system.
 *
 * This grammar does not introduce:
 *
 *     foreignInt
 *     foreignPointer
 *     cInt
 *     nativePointer
 *     qpuPointer
 *     devicePointer
 *
 * or equivalent target-specific type universes.
 *
 * Representation is interoperability metadata, not a replacement type system.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A foreign callable may have effects such as:
 *
 *     foreign
 *     native
 *     io
 *     network
 *     mutation
 *     distributed
 *     measurement
 *     quantum
 *     simulation
 *
 * This grammar does not define those effects.
 *
 * They are attached through canonical attributes/metadata and interpreted by
 * the effects subsystem.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE CONTRACT
 * ============================================================================
 *
 * A foreign callable may require capabilities or resources.
 *
 * Examples of semantic intent include:
 *
 *     capability("foreign.call")
 *     capability("network")
 *     capability("quantum.boundary")
 *     memory >= required_memory
 *
 * This grammar does not resolve those requirements.
 *
 * Resource realization belongs downstream.
 *
 * ============================================================================
 * ABI CONTRACT
 * ============================================================================
 *
 * ABI identity MUST remain separate from the callable declaration.
 *
 * This grammar may preserve ABI metadata structurally, but it MUST NOT define:
 *
 *     parameter registers
 *     return registers
 *     stack slots
 *     stack widths
 *     alignment limits
 *     pointer widths
 *     register widths
 *     object formats
 *     symbol decoration
 *
 * Those belong to ABI semantic analysis and target lowering.
 *
 * ============================================================================
 * CALLING-CONVENTION CONTRACT
 * ============================================================================
 *
 * Calling convention is metadata.
 *
 * It does not grant capabilities.
 *
 * It does not select hardware.
 *
 * It does not define ABI layout.
 *
 * It does not define linkage.
 *
 * Semantic interoperability analysis resolves the relationship between:
 *
 *     callable
 *     calling convention
 *     ABI
 *     linkage
 *     target capability
 *
 * ============================================================================
 * LINKAGE CONTRACT
 * ============================================================================
 *
 * Linkage is represented as metadata.
 *
 * This grammar does not perform:
 *
 *     static linking
 *     dynamic linking
 *     symbol lookup
 *     loader operations
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * Parsing MUST be inert.
 *
 * This grammar performs:
 *
 *     no filesystem access
 *     no network access
 *     no environment inspection
 *     no library loading
 *     no symbol resolution
 *     no process creation
 *     no foreign-code execution
 *     no hardware discovery
 *     no memory allocation on behalf of the target
 *
 * It contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no unsafe code
 *
 * Rust code generated/consuming the parser MUST remain safe Rust and support:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses structural repetition rather than fixed cardinalities.
 *
 * It imposes no language-level maximum on:
 *
 *     functions
 *     parameters
 *     generic parameters
 *     metadata entries
 *     attributes
 *
 * It MUST NOT define:
 *
 *     MAX_FOREIGN_FUNCTIONS
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_METADATA
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Any operational parser/compiler limit is an implementation resource policy,
 * not a language semantic limit.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     lexical configuration
 *     grammar version
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     hardware availability
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     target selection
 *     runtime state
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This grammar is responsible for structural syntax errors such as:
 *
 *     missing function name
 *     malformed generic parameter list
 *     malformed parameter list
 *     malformed return type
 *     malformed metadata assignment
 *     missing semicolon
 *     malformed attribute
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples that MUST remain semantic rather than parser-level errors:
 *
 *     unknown ABI
 *     unavailable calling convention
 *     unresolved symbol
 *     unsupported foreign language
 *     incompatible representation
 *     insufficient resource
 *     unavailable capability
 *     prohibited FFI operation
 *     incompatible target
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical source form inside an existing external declaration is:
 *
 *     extern "C" {
 *         fn add(a: i32, b: i32) -> i32;
 *     }
 *
 * This file therefore consumes only the `fn ... ;` member.
 *
 * The surrounding `foreign-functions.g4` remains responsible for:
 *
 *     extern
 *     source identity
 *     enclosing braces
 *
 * Existing external declarations remain source-compatible when the member is
 * delegated to this grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Parent:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * MUST:
 *
 *     import ExternalForeignFunctions
 *
 * and delegate its foreign callable member to:
 *
 *     externalForeignFunction
 *
 * The parent MUST retain ownership of:
 *
 *     foreignFunctionDeclaration
 *     extern
 *     source identity
 *     external interface body
 *
 * Generic FFI:
 *
 *     grammar/interoperability/ffi.g4
 *
 * MAY reuse:
 *
 *     externalForeignFunctionSignature
 *
 * but MUST NOT create a second external-function signature model.
 *
 * ABI:
 *
 *     grammar/interoperability/abi.g4
 *
 * consumes semantic metadata from this declaration.
 *
 * Calling conventions:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * consumes the corresponding semantic metadata.
 *
 * Foreign types:
 *
 *     grammar/interoperability/foreign-types.g4
 *
 * remains independently authoritative.
 *
 * Interoperability dispatcher:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * MUST route external declarations through the existing foreign-function
 * declaration owner rather than directly duplicating this leaf.
 *
 * Canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * remains unchanged except for the dispatcher integration.
 *
 * Root:
 *
 *     grammar/Zamani.g4
 *
 * does not import this file directly.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * There is no:
 *
 *     ExternalForeignFunctionIR
 *     FFIIR
 *     ABIIR
 *     ForeignFunctionTargetIR
 *
 * The declaration is represented in the canonical semantic interoperability
 * model.
 *
 * If the callable participates in quantum computation, the relevant semantic
 * operation eventually crosses the existing:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backend realization may eventually perform:
 *
 *     symbol resolution
 *     ABI lowering
 *     calling-sequence construction
 *     marshalling
 *     linking
 *     dynamic loading
 *     target adaptation
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar ExternalForeignFunctions;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Attributes,
    Parameters,
    FunctionGenerics,
    Types
;


/*
 * ============================================================================
 * 1. EXTERNAL FOREIGN FUNCTION
 * ============================================================================
 *
 * This is the primary public rule.
 *
 * It is designed to be consumed inside an already established external
 * declaration/block.
 *
 * Example:
 *
 *     extern "C" {
 *         fn add(a: i32, b: i32) -> i32;
 *     }
 *
 * No `extern` token appears here because the enclosing grammar owns it.
 */
externalForeignFunction
    : attribute*
      externalForeignFunctionSignature
      externalForeignFunctionMetadata*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. SIGNATURE
 * ============================================================================
 *
 * The signature uses canonical:
 *
 *     identifier
 *     functionGenericParameters
 *     parameterList
 *     typeExpression
 *
 * rules rather than defining competing versions.
 */
externalForeignFunctionSignature
    : FN
      identifier
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      externalForeignReturnClause?
    ;


/*
 * ============================================================================
 * 3. RETURN TYPE
 * ============================================================================
 *
 * The canonical thin arrow is used.
 */
externalForeignReturnClause
    : THIN_ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * 4. OPEN-WORLD METADATA
 * ============================================================================
 *
 * Metadata remains declarative.
 *
 * Examples:
 *
 *     symbol = "native_add";
 *     language = "C";
 *     abi = "c";
 *     linkage = "external";
 *     calling_convention = "platform::native";
 *
 * The grammar deliberately does not enumerate those keys.
 */
externalForeignFunctionMetadata
    : attribute
    | qualifiedName
      ASSIGN
      externalForeignFunctionMetadataValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. METADATA VALUES
 * ============================================================================
 *
 * Metadata values are intentionally broad enough to preserve source intent.
 *
 * Semantic schemas decide which value kinds are legal for a particular key.
 */
externalForeignFunctionMetadataValue
    : expression
    ;


/*
 * ============================================================================
 * 6. REUSABLE SIGNATURE BOUNDARY
 * ============================================================================
 *
 * This adapter is intentionally signature-only.
 *
 * Consumers that need to inspect an external callable without consuming its
 * terminating semicolon may use this rule.
 */
externalForeignFunctionSignatureBoundary
    : externalForeignFunctionSignature
    ;


/*
 * ============================================================================
 * 7. REUSABLE DECLARATION BOUNDARY
 * ============================================================================
 *
 * Stable alias for tooling and interoperability delegates.
 */
externalForeignFunctionDeclaration
    : externalForeignFunction
    ;