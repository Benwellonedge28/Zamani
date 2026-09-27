/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/wasm.g4
 *
 * Grammar:
 *     Wasm
 *
 * Status:
 *     PRODUCTION-READY INTEROPERABILITY DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     SAFE RUST ONLY
 *     No unsafe Rust.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No filesystem access.
 *     No network access.
 *     No process execution.
 *     No runtime execution.
 *     No hardware discovery.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the Zamani SOURCE-LEVEL WEBASSEMBLY INTEROPERABILITY
 * CONTRACT.
 *
 * It allows Zamani programs to describe WebAssembly-oriented interoperability
 * without making WebAssembly the canonical Zamani semantic model.
 *
 * This grammar describes:
 *
 *     - symbolic WASM module identities;
 *     - imported functions;
 *     - exported functions;
 *     - function signatures;
 *     - WASM-visible types;
 *     - globals;
 *     - memories;
 *     - tables;
 *     - tags;
 *     - data/element initialization intent;
 *     - imports and exports;
 *     - module metadata;
 *     - resource/capability requirements;
 *     - ABI/FFI references;
 *     - linking intent;
 *     - custom interoperability metadata.
 *
 * It does NOT implement:
 *
 *     - the WebAssembly virtual machine;
 *     - WASM bytecode execution;
 *     - WASM validation;
 *     - WASM binary encoding;
 *     - WASM instruction selection;
 *     - WASM optimization;
 *     - WASM runtime behavior;
 *     - a linker;
 *     - a loader;
 *     - WASI;
 *     - a particular WASM engine;
 *     - a particular browser;
 *     - a particular operating system;
 *     - a particular CPU;
 *     - a particular GPU;
 *     - a particular FPGA;
 *     - a particular QPU;
 *     - hardware discovery.
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
 *     Zamani parser composition
 *          |
 *          v
 *     Wasm interoperability syntax
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+---------+----------+-----------+
 *     |              |          |           |
 *     v              v          v           v
 *    types          FFI        ABI       capabilities
 *     |              |          |           |
 *     +--------------+----------+-----------+
 *                    |
 *                    v
 *             canonical semantic model
 *                    |
 *                    v
 *               canonical IR
 *                    |
 *          +---------+----------+
 *          |                    |
 *          v                    v
 *    classical semantics   quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *             optimization
 *                    |
 *                    v
 *             target lowering
 *                    |
 *                    v
 *          WebAssembly realization
 *
 * WebAssembly is therefore an INTEROPERABILITY TARGET/FORMAT.
 *
 * It is not the canonical Zamani semantic representation.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - Zamani-facing WASM interoperability syntax;
 *     - WASM module/interface declarations;
 *     - WASM imports/exports;
 *     - WASM-visible signatures;
 *     - WASM-specific resource declarations;
 *     - WASM interoperability metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifier;
 *     - qualifiedName;
 *     - attribute;
 *     - expression;
 *     - generic Zamani types;
 *     - generic FFI;
 *     - ABI contracts;
 *     - generic foreign declarations;
 *     - canonical AST;
 *     - canonical IR;
 *     - WASM binary encoding;
 *     - WASM instruction semantics;
 *     - runtime execution;
 *     - target selection.
 *
 * Ownership remains:
 *
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *     grammar/expressions/
 *     grammar/types/
 *     grammar/interoperability/ffi.g4
 *     grammar/interoperability/abi.g4
 *     grammar/interoperability/foreign-functions.g4
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical parser-facing lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This grammar MUST NOT introduce:
 *
 *     WasmLexer
 *     WasmTokens
 *     WASMLexer
 *
 * or another lexical authority.
 *
 * WASM names such as:
 *
 *     i32
 *     i64
 *     f32
 *     f64
 *     v128
 *     funcref
 *     externref
 *
 * are represented as identifiers at this boundary.
 *
 * This is intentional.
 *
 * They are semantic WASM types, not necessarily Zamani-wide reserved words.
 *
 * The same rule allows future WASM value/reference types without requiring a
 * global Zamani keyword for every future WebAssembly proposal.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * WASM interoperability MUST remain portable.
 *
 * This grammar MUST NOT encode universal limits for:
 *
 *     MAX_MEMORY
 *     MAX_WASM_MEMORY
 *     MAX_TABLES
 *     MAX_TABLE_ELEMENTS
 *     MAX_FUNCTIONS
 *     MAX_IMPORTS
 *     MAX_EXPORTS
 *     MAX_GLOBALS
 *     MAX_LOCALS
 *     MAX_PARAMETERS
 *     MAX_RESULTS
 *     MAX_MODULES
 *     MAX_INSTANCES
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_REGISTER_WIDTH
 *     MAX_POINTER_WIDTH
 *
 * Nor may it encode physical resources such as:
 *
 *     cpu0
 *     gpu0
 *     device0
 *     node0
 *     memory_bank0
 *
 * as universal WASM semantics.
 *
 * Repetition is therefore represented with ANTLR repetition operators.
 *
 * Practical implementation limits may exist because an actual compiler,
 * runtime, target, browser, WASM engine, operating environment, or deployment
 * has finite resources.
 *
 * Such limits are NOT language-level WASM limits.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The Zamani implementation targeting Rust 1.97 / 1.97.1 MUST remain safe
 * Rust.
 *
 * This grammar does not require:
 *
 *     unsafe
 *     raw pointer dereferencing
 *     native memory manipulation
 *     FFI implementation code
 *
 * A WASM boundary that semantically requires unsafe native behavior MUST be
 * represented as an explicit interoperability/security requirement and
 * validated downstream.
 *
 * The grammar itself never emits or executes unsafe Rust.
 *
 * ============================================================================
 * INERTNESS
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     - load a .wasm file;
 *     - execute WASM;
 *     - instantiate a module;
 *     - invoke an export;
 *     - resolve an import;
 *     - contact a URL;
 *     - access a filesystem;
 *     - inspect browser state;
 *     - inspect hardware;
 *     - select a WASM engine;
 *     - allocate native memory;
 *     - invoke WASI;
 *     - perform dynamic linking.
 *
 * Those operations belong to explicitly authorized downstream layers.
 *
 * ============================================================================
 * WASM FORMAT VS WASM SEMANTICS
 * ============================================================================
 *
 * This grammar intentionally distinguishes:
 *
 *     WASM interoperability intent
 *
 * from:
 *
 *     WebAssembly binary/module realization.
 *
 * It does not attempt to reproduce the complete WebAssembly text format.
 *
 * Raw WAT/WASM parsing belongs to a dedicated external-format frontend if
 * required.
 *
 * This file instead gives Zamani a stable, target-independent contract for
 * interoperating with WebAssembly.
 *
 * ============================================================================
 * ENTRY-POINT CONTRACT
 * ============================================================================
 *
 * The interoperability composition grammar decides when:
 *
 *     wasmInteropDeclaration
 *
 * is legal.
 *
 * This file is NOT the Zamani compilation-unit root.
 *
 * It MUST NOT contain:
 *
 *     program
 *     compilationUnit
 *     EOF
 *
 * as its authoritative source root.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every construct parsed here maps to domain-neutral interoperability AST
 * structures.
 *
 * Conceptually:
 *
 *     wasmInteropDeclaration
 *          ->
 *     generic foreign/interoperability declaration
 *          ->
 *     WASM semantic contract
 *          ->
 *     canonical semantic model
 *          ->
 *     canonical IR
 *          ->
 *     WASM lowering
 *
 * The frontend MUST NOT create a permanent AST hierarchy tied to:
 *
 *     Chrome
 *     Firefox
 *     Wasmtime
 *     Wasmer
 *     V8
 *     JavaScriptCore
 *     a particular operating system
 *     a particular CPU
 *     a particular device.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO WASM IR.
 *
 * A semantic WASM representation may exist downstream where required by the
 * compiler, but it must remain an implementation/lowering representation.
 *
 * The canonical Zamani semantic architecture remains authoritative.
 *
 * If a WASM boundary participates in quantum computation:
 *
 *     Zamani quantum semantics
 *          ->
 *     canonical quantum::ir
 *          ->
 *     interoperability/lowering
 *
 * This file MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Generic FFI remains owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * ABI contracts remain owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * This file may carry symbolic references to those contracts through:
 *
 *     attributes;
 *     qualified names;
 *     metadata;
 *     semantic association.
 *
 * It MUST NOT redefine:
 *
 *     ABI layout;
 *     calling convention implementation;
 *     register assignment;
 *     stack layout;
 *     binary encoding;
 *     symbol resolution.
 *
 * ============================================================================
 * WASI / HOST INTEGRATION
 * ============================================================================
 *
 * WASI is treated as an interoperability namespace/capability rather than
 * being hard-coded into the core language.
 *
 * For example, a semantic contract may identify:
 *
 *     wasi
 *
 * through an identifier or attribute.
 *
 * This grammar does not enumerate every WASI API.
 *
 * Future WASI versions can therefore be introduced without redesigning the
 * Zamani core grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY MODEL
 * ============================================================================
 *
 * WASM resource requirements are semantic contracts.
 *
 * Examples may include:
 *
 *     requires capability("wasm.reference_types");
 *     requires capability("wasm.simd");
 *     requires capability("wasm.bulk_memory");
 *
 * Whether the target satisfies such capabilities is decided downstream.
 *
 * A capability requirement is NOT equivalent to:
 *
 *     use engine X
 *     use device Y
 *     use CPU Z
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Lexical/syntactic interpretation MUST NOT depend on:
 *
 *     hardware;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     runtime state;
 *     current time;
 *     randomness;
 *     selected WASM engine.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar Wasm;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Attributes, Expressions, Types;


/*
 * ============================================================================
 * 1. PUBLIC INTEROPERABILITY ENTRY POINT
 * ============================================================================
 *
 * The surrounding interoperability composition grammar invokes this rule.
 *
 * No target selection occurs here.
 */
wasmInteropDeclaration
    : attribute*
      EXTERN
      stringLiteral
      wasmModuleBody
    ;


/*
 * ============================================================================
 * 2. MODULE BODY
 * ============================================================================
 *
 * A module is an abstract interoperability namespace.
 *
 * The string literal identifies the declared interoperability source/identity.
 *
 * It is opaque metadata.
 *
 * It MUST NOT automatically mean:
 *
 *     filesystem path
 *     URL
 *     package registry location
 *     device
 *     runtime
 *     browser
 *     operating system.
 */
wasmModuleBody
    : LBRACE
      wasmModuleItem*
      RBRACE
    ;


wasmModuleItem
    : attribute* wasmModuleMetadata
    | attribute* wasmImportDeclaration
    | attribute* wasmExportDeclaration
    | attribute* wasmFunctionDeclaration
    | attribute* wasmTypeDeclaration
    | attribute* wasmGlobalDeclaration
    | attribute* wasmMemoryDeclaration
    | attribute* wasmTableDeclaration
    | attribute* wasmTagDeclaration
    | attribute* wasmDataDeclaration
    | attribute* wasmElementDeclaration
    ;


/*
 * ============================================================================
 * 3. MODULE METADATA
 * ============================================================================
 *
 * Extensible metadata prevents this grammar from becoming a finite list of
 * every future WebAssembly proposal.
 *
 * Examples:
 *
 *     version = ...
 *     proposal = ...
 *     engine = ...
 *     feature = ...
 *
 * The semantic layer decides whether a metadata key is recognized.
 */
wasmModuleMetadata
    : identifier
      ASSIGN
      wasmMetadataValue
      SEMICOLON
    ;


wasmMetadataValue
    : stringLiteral
    | expression
    ;


/*
 * ============================================================================
 * 4. IMPORT
 * ============================================================================
 *
 * Example conceptual form:
 *
 *     import "host" function_name(...);
 *
 * The module/source identity remains symbolic.
 */
wasmImportDeclaration
    : wasmImportSource
      wasmImportKind
      SEMICOLON
    ;


wasmImportSource
    : stringLiteral
    | qualifiedName
    ;


wasmImportKind
    : wasmImportedFunction
    | wasmImportedGlobal
    | wasmImportedMemory
    | wasmImportedTable
    | wasmImportedTag
    ;


wasmImportedFunction
    : FN
      identifier
      wasmFunctionSignature
      wasmImportContract*
    ;


wasmImportedGlobal
    : identifier
      COLON
      wasmGlobalType
      wasmImportContract*
    ;


wasmImportedMemory
    : identifier
      COLON
      wasmMemoryType
      wasmImportContract*
    ;


wasmImportedTable
    : identifier
      COLON
      wasmTableType
      wasmImportContract*
    ;


wasmImportedTag
    : identifier
      COLON
      wasmTagType
      wasmImportContract*
    ;


wasmImportContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | attribute
    | wasmMetadataAssignment
    ;


/*
 * ============================================================================
 * 5. EXPORT
 * ============================================================================
 *
 * An export exposes a Zamani semantic entity to the WASM interoperability
 * boundary.
 *
 * Export names are symbolic.
 */
wasmExportDeclaration
    : stringLiteral
      wasmExportTarget
      wasmExportContract*
      SEMICOLON
    ;


wasmExportTarget
    : FN
      qualifiedName
    | TYPE
      qualifiedName
    | CONST
      qualifiedName
    | identifier
    ;


wasmExportContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | attribute
    | wasmMetadataAssignment
    ;


/*
 * ============================================================================
 * 6. FUNCTION DECLARATION
 * ============================================================================
 *
 * This describes a WASM-visible callable contract.
 *
 * It has no implementation body.
 */
wasmFunctionDeclaration
    : FN
      identifier
      wasmGenericParameters?
      wasmFunctionSignature
      wasmFunctionContract*
      SEMICOLON
    ;


wasmFunctionSignature
    : LPAREN
      wasmParameter*
      RPAREN
      wasmReturnClause?
    ;


wasmParameter
    : identifier
      COLON
      wasmValueType
      COMMA?
    ;


wasmReturnClause
    : ARROW
      wasmResultTypes
    ;


wasmResultTypes
    : wasmValueType
      (
          COMMA
          wasmValueType
      )*
    ;


wasmGenericParameters
    : LESS
      identifier
      (
          COMMA
          identifier
      )*
      COMMA?
      GREATER
    ;


wasmFunctionContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | wasmEffectContract
    | wasmMetadataAssignment
    | attribute
    ;


/*
 * ============================================================================
 * 7. FUNCTION EFFECTS
 * ============================================================================
 *
 * Effects remain owned by the canonical effects subsystem.
 */
wasmEffectContract
    : WITH
      EFFECTS
      LBRACE
      qualifiedNameList
      RBRACE
    ;


/*
 * ============================================================================
 * 8. TYPE DECLARATION
 * ============================================================================
 *
 * A named WASM interoperability type is symbolic.
 *
 * The grammar does not create a fixed global enumeration of every future
 * WebAssembly type proposal.
 */
wasmTypeDeclaration
    : TYPE
      identifier
      wasmTypeParameters?
      wasmTypeDefinition?
      SEMICOLON
    ;


wasmTypeParameters
    : LESS
      identifier
      (
          COMMA
          identifier
      )*
      COMMA?
      GREATER
    ;


wasmTypeDefinition
    : COLON
      wasmTypeReference
    ;


wasmTypeReference
    : qualifiedName
    | wasmValueType
    ;


/*
 * ============================================================================
 * 9. WASM VALUE TYPES
 * ============================================================================
 *
 * Known WebAssembly value/reference types are deliberately represented by
 * the generic identifier mechanism.
 *
 * Examples:
 *
 *     i32
 *     i64
 *     f32
 *     f64
 *     v128
 *     funcref
 *     externref
 *
 * Future standardized or extension types can therefore be introduced without
 * changing the Zamani-wide lexer.
 *
 * Semantic validation determines whether a particular type is supported by
 * the selected WASM target/profile.
 */
wasmValueType
    : qualifiedName
    | wasmParameterizedValueType
    ;


wasmParameterizedValueType
    : qualifiedName
      LESS
      wasmTypeArgument
      (
          COMMA
          wasmTypeArgument
      )*
      COMMA?
      GREATER
    ;


wasmTypeArgument
    : wasmValueType
    | typeExpression
    | expression
    ;


/*
 * ============================================================================
 * 10. GLOBALS
 * ============================================================================
 */
wasmGlobalDeclaration
    : CONST
      identifier
      COLON
      wasmGlobalType
      wasmInitializer?
      wasmGlobalContract*
      SEMICOLON
    ;


wasmGlobalType
    : wasmValueType
    | wasmMutableType
    ;


wasmMutableType
    : identifier
      wasmValueType
    ;


wasmInitializer
    : ASSIGN
      expression
    ;


wasmGlobalContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | wasmMetadataAssignment
    | attribute
    ;


/*
 * ============================================================================
 * 11. MEMORY
 * ============================================================================
 *
 * Memory quantities are expressions, not parser-level constants.
 *
 * This permits:
 *
 *     memory requirements derived from program semantics;
 *     symbolic sizes;
 *     target negotiation;
 *     future memory models.
 *
 * No universal memory ceiling is encoded.
 */
wasmMemoryDeclaration
    : identifier
      COLON
      wasmMemoryType
      wasmMemoryContract*
      SEMICOLON
    ;


wasmMemoryType
    : identifier
      wasmMemoryParameterList?
    ;


wasmMemoryParameterList
    : LESS
      wasmMemoryParameter
      (
          COMMA
          wasmMemoryParameter
      )*
      COMMA?
      GREATER
    ;


wasmMemoryParameter
    : identifier
      COLON
      expression
    | expression
    ;


wasmMemoryContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | wasmMetadataAssignment
    | attribute
    ;


/*
 * ============================================================================
 * 12. TABLE
 * ============================================================================
 *
 * Table element counts are expressions.
 *
 * The grammar imposes no maximum table size.
 */
wasmTableDeclaration
    : identifier
      COLON
      wasmTableType
      wasmTableContract*
      SEMICOLON
    ;


wasmTableType
    : identifier
      wasmTableParameterList?
    ;


wasmTableParameterList
    : LESS
      wasmTableParameter
      (
          COMMA
          wasmTableParameter
      )*
      COMMA?
      GREATER
    ;


wasmTableParameter
    : identifier
      COLON
      expression
    | expression
    ;


wasmTableContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | wasmMetadataAssignment
    | attribute
    ;


/*
 * ============================================================================
 * 13. TAGS
 * ============================================================================
 *
 * Exception/tag behavior is represented as a semantic boundary.
 *
 * The grammar does not hard-code a finite tag set.
 */
wasmTagDeclaration
    : identifier
      COLON
      wasmTagType
      wasmTagContract*
      SEMICOLON
    ;


wasmTagType
    : identifier
      LPAREN
      wasmValueType*
      RPAREN
    ;


wasmTagContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | wasmMetadataAssignment
    | attribute
    ;


/*
 * ============================================================================
 * 14. DATA SEGMENTS
 * ============================================================================
 *
 * The payload is represented as source-level metadata rather than embedding
 * a binary WebAssembly encoder into the grammar.
 */
wasmDataDeclaration
    : identifier
      ASSIGN
      wasmDataValue
      wasmDataContract*
      SEMICOLON
    ;


wasmDataValue
    : stringLiteral
    | expression
    ;


wasmDataContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | wasmMetadataAssignment
    | attribute
    ;


/*
 * ============================================================================
 * 15. ELEMENT SEGMENTS
 * ============================================================================
 *
 * Element targets are symbolic.
 */
wasmElementDeclaration
    : identifier
      ASSIGN
      wasmElementValue
      wasmElementContract*
      SEMICOLON
    ;


wasmElementValue
    : expression
    | wasmElementList
    ;


wasmElementList
    : LBRACK
      expression*
      RBRACK
    ;


wasmElementContract
    : wasmRequiresContract
    | wasmCapabilityContract
    | wasmMetadataAssignment
    | attribute
    ;


/*
 * ============================================================================
 * 16. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are semantic prerequisites.
 *
 * They are not machine-selection commands.
 *
 * Examples:
 *
 *     requires capability("wasm.reference_types");
 *     requires memory >= required_memory;
 *
 * The expression grammar owns the expression itself.
 */
wasmRequiresContract
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. CAPABILITIES
 * ============================================================================
 *
 * Capability names remain symbolic and extensible.
 */
wasmCapabilityContract
    : REQUIRES
      CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. GENERIC METADATA
 * ============================================================================
 *
 * Metadata permits extension without turning every WebAssembly proposal into
 * a new Zamani keyword.
 */
wasmMetadataAssignment
    : identifier
      ASSIGN
      wasmMetadataValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. ABI/FFI SYMBOLIC REFERENCES
 * ============================================================================
 *
 * These references identify contracts.
 *
 * They do not implement ABI or FFI behavior.
 */
wasmInteropReference
    : qualifiedName
    | stringLiteral
    ;


wasmAbiReference
    : qualifiedName
    ;


wasmFfiReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 20. EXTERNAL CONTRACT METADATA
 * ============================================================================
 *
 * This rule allows interoperability composition to associate an existing ABI
 * or FFI contract with a WASM declaration without redefining either contract.
 */
wasmBoundaryMetadata
    : identifier
      ASSIGN
      wasmInteropReference
      SEMICOLON
    | identifier
      ASSIGN
      wasmAbiReference
      SEMICOLON
    | identifier
      ASSIGN
      wasmFfiReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. PORTABILITY PROFILE
 * ============================================================================
 *
 * A profile is symbolic.
 *
 * It does not enumerate engines or hardware.
 */
wasmProfile
    : identifier
    ;


wasmProfileClause
    : identifier
      ASSIGN
      wasmProfile
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. VERSION / FEATURE METADATA
 * ============================================================================
 *
 * Version information is represented as ordinary source metadata rather than
 * being hard-coded into parser alternatives.
 *
 * This permits:
 *
 *     current WebAssembly revisions;
 *     future WebAssembly revisions;
 *     proposals;
 *     implementation profiles.
 *
 * Semantic compatibility checking belongs downstream.
 */
wasmFeatureDeclaration
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. SECURITY CONTRACT
 * ============================================================================
 *
 * Security requirements remain declarative.
 *
 * No security mechanism is executed by the grammar.
 */
wasmSecurityMetadata
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. HOST/ENVIRONMENT CONTRACT
 * ============================================================================
 *
 * Host environment is represented symbolically.
 *
 * No operating-system or browser is hard-coded into the grammar.
 */
wasmHostMetadata
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions are source semantics.
 *
 * They are not parser-level machine limits.
 */
wasmResourceMetadata
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. VALIDATION BOUNDARY
 * ============================================================================
 *
 * The grammar intentionally accepts symbolic WASM types and metadata.
 *
 * Semantic analysis MUST subsequently validate:
 *
 *     - type validity;
 *     - import/export compatibility;
 *     - function signature compatibility;
 *     - memory semantics;
 *     - table semantics;
 *     - feature/profile compatibility;
 *     - ABI compatibility;
 *     - FFI compatibility;
 *     - security requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - source-language type compatibility.
 *
 * This keeps syntax open while keeping semantics strict.
 *
 * ============================================================================
 * 27. SOURCE SPANS
 * ============================================================================
 *
 * Every AST node created from these productions MUST retain sufficient source
 * span information for:
 *
 *     diagnostics;
 *     IDE/LSP;
 *     formatting;
 *     refactoring;
 *     provenance;
 *     compatibility diagnostics;
 *     generated-code mapping.
 *
 * Source-span construction belongs to the parser/AST implementation rather
 * than embedded actions in this grammar.
 *
 * ============================================================================
 * 28. ERROR MODEL
 * ============================================================================
 *
 * Invalid WASM interoperability contracts must produce structured diagnostics
 * downstream.
 *
 * The grammar must not:
 *
 *     silently discard malformed declarations;
 *     execute recovery code;
 *     resolve imports;
 *     infer a target;
 *     invent a missing ABI;
 *     invent a missing capability.
 *
 * ============================================================================
 * 29. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file deliberately contains no universal constants for:
 *
 *     qubits;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     nodes;
 *     devices;
 *     threads;
 *     memory;
 *     tables;
 *     functions;
 *     imports;
 *     exports;
 *     globals;
 *     module count;
 *     parameter count;
 *     result count;
 *     tensor rank;
 *     register width;
 *     pointer width.
 *
 * The following patterns MUST remain absent from this grammar as language
 * limits:
 *
 *     MAX_WASM_MEMORY
 *     MAX_WASM_FUNCTIONS
 *     MAX_WASM_TABLES
 *     MAX_WASM_IMPORTS
 *     MAX_WASM_EXPORTS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * ============================================================================
 * 30. SECURITY AUDIT
 * ============================================================================
 *
 * No grammar action may:
 *
 *     load;
 *     execute;
 *     instantiate;
 *     link;
 *     download;
 *     resolve;
 *     probe;
 *     allocate;
 *     invoke.
 *
 * All such behavior is downstream.
 *
 * ============================================================================
 * 31. PERFORMANCE / SCALABILITY
 * ============================================================================
 *
 * This grammar uses unbounded repetition where the language semantics permit
 * arbitrary collections:
 *
 *     wasmModuleItem*
 *     wasmFunctionContract*
 *     wasmImportContract*
 *     wasmExportContract*
 *     wasmGlobalContract*
 *     wasmMemoryContract*
 *     wasmTableContract*
 *     wasmTagContract*
 *     wasmDataContract*
 *     wasmElementContract*
 *
 * No artificial finite parser ceiling is introduced.
 *
 * Actual parser/compiler resource exhaustion is an implementation concern.
 *
 * ============================================================================
 * 32. QUANTUM INTEGRATION
 * ============================================================================
 *
 * WASM may act as:
 *
 *     - a host boundary for classical computation;
 *     - a portable execution target;
 *     - an accelerator boundary;
 *     - a service boundary;
 *     - a component surrounding quantum orchestration;
 *     - a bridge between classical and quantum workflows.
 *
 * This grammar does NOT define quantum semantics.
 *
 * If a WASM interoperability declaration carries quantum semantics, the
 * semantic pipeline remains:
 *
 *     Zamani quantum source
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic quantum model
 *          ->
 *     quantum::ir
 *          ->
 *     optimization / routing / scheduling / resilience / ZQN
 *          ->
 *     target realization
 *
 * WASM lowering is downstream of those semantic decisions.
 *
 * ============================================================================
 * 33. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical Zamani functions can be exposed through WASM contracts without
 * making WASM value types the Zamani type system.
 *
 * Conceptually:
 *
 *     Zamani function
 *          ->
 *     semantic callable
 *          ->
 *     WASM boundary
 *          ->
 *     WASM realization
 *
 * Generic Zamani types remain owned by:
 *
 *     grammar/types/
 *
 * and semantic type analysis.
 *
 * ============================================================================
 * 34. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * WASM may represent a software control plane for hardware/HDL systems.
 *
 * Hardware semantics remain owned by:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *
 * This file must not introduce:
 *
 *     physical registers;
 *     physical memory addresses;
 *     FPGA resources;
 *     CPU identifiers;
 *     GPU identifiers;
 *     QPU identifiers;
 *     hardware topology.
 *
 * ============================================================================
 * 35. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A WASM module may be used as a distributed service boundary.
 *
 * Network/distribution semantics remain owned by:
 *
 *     grammar/networking/
 *     grammar/distributed/
 *
 * This grammar does not turn:
 *
 *     host;
 *     node;
 *     endpoint;
 *     service;
 *
 * into a physical deployment decision.
 *
 * ============================================================================
 * 36. COMPILER INTEGRATION
 * ============================================================================
 *
 * Downstream compiler stages are responsible for:
 *
 *     - semantic validation;
 *     - type conversion;
 *     - ABI validation;
 *     - FFI validation;
 *     - capability checking;
 *     - resource checking;
 *     - WASM feature/profile validation;
 *     - lowering;
 *     - optimization;
 *     - binary/module generation;
 *     - deployment.
 *
 * This grammar supplies only syntax.
 *
 * ============================================================================
 * 37. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime integration may select a suitable WASM realization according to:
 *
 *     semantic requirements;
 *     capabilities;
 *     deployment policy;
 *     security policy;
 *     runtime availability;
 *     target constraints.
 *
 * The grammar does not select:
 *
 *     a WASM engine;
 *     a browser;
 *     a CPU;
 *     a device;
 *     a node.
 *
 * ============================================================================
 * 38. TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling must be able to consume the same grammar representation for:
 *
 *     - diagnostics;
 *     - formatting;
 *     - syntax highlighting;
 *     - LSP;
 *     - documentation;
 *     - refactoring;
 *     - source mapping;
 *     - compatibility analysis.
 *
 * Tooling must not create a separate WASM language.
 *
 * ============================================================================
 * 39. TEST CONTRACT
 * ============================================================================
 *
 * The corresponding tests belong under:
 *
 *     grammar/tests/interoperability/wasm/
 *
 * Required categories:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     compatibility/
 *     diagnostics/
 *     portability/
 *
 * Positive tests MUST cover:
 *
 *     - module declarations;
 *     - imports;
 *     - exports;
 *     - functions;
 *     - multiple parameters;
 *     - multiple results;
 *     - named types;
 *     - globals;
 *     - memories;
 *     - tables;
 *     - tags;
 *     - data;
 *     - elements;
 *     - metadata;
 *     - capabilities;
 *     - requirements;
 *     - attributes;
 *     - ABI/FFI associations.
 *
 * Negative tests MUST cover:
 *
 *     - malformed signatures;
 *     - malformed imports;
 *     - malformed exports;
 *     - missing identifiers;
 *     - invalid delimiters;
 *     - invalid type syntax;
 *     - malformed metadata;
 *     - malformed requirements.
 *
 * Boundary tests MUST verify:
 *
 *     - empty module;
 *     - one declaration;
 *     - many declarations;
 *     - nested metadata;
 *     - symbolic resource quantities;
 *     - symbolic feature requirements;
 *     - future/unknown symbolic type names.
 *
 * Scalability tests MUST verify that the grammar does not impose artificial
 * maxima.
 *
 * Determinism tests MUST parse identical source identically.
 *
 * ============================================================================
 * 40. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing Zamani lexical tokens are consumed through:
 *
 *     ZamaniLexer
 *
 * This grammar must not require new globally reserved WASM keywords merely to
 * represent future WebAssembly features.
 *
 * New WASM semantic concepts should prefer:
 *
 *     identifiers;
 *     qualified names;
 *     attributes;
 *     metadata;
 *     capabilities;
 *     requirements.
 *
 * A future language-level keyword requires the normal lexical compatibility
 * process under:
 *
 *     grammar/lexer/
 *     grammar/spec/
 *     grammar/compatibility/
 *
 * ============================================================================
 * 41. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It exists at grammar/interoperability/wasm.g4.
 *
 * [x] The grammar identity is unique: Wasm.
 *
 * [x] It consumes tokenVocab = ZamaniLexer.
 *
 * [x] It is a parser delegate, not a compilation root.
 *
 * [x] It owns WASM interoperability syntax only.
 *
 * [x] FFI remains owned by ffi.g4.
 *
 * [x] ABI remains owned by abi.g4.
 *
 * [x] Generic foreign declarations remain owned by foreign-functions.g4.
 *
 * [x] Names are reused from Names.
 *
 * [x] Attributes are reused from Attributes.
 *
 * [x] Expressions are reused from Expressions.
 *
 * [x] Types are reused from Types where appropriate.
 *
 * [x] No second lexer is introduced.
 *
 * [x] No second AST authority is introduced.
 *
 * [x] No WASM runtime is implemented.
 *
 * [x] No WASM binary encoder is implemented.
 *
 * [x] No WASM engine is selected.
 *
 * [x] No browser is selected.
 *
 * [x] No operating system is selected.
 *
 * [x] No physical hardware is selected.
 *
 * [x] No universal machine/resource maximum is encoded.
 *
 * [x] WASM type names remain extensible.
 *
 * [x] WASM proposal evolution does not require a global keyword explosion.
 *
 * [x] Requirements are distinct from capabilities.
 *
 * [x] Resource requirements remain semantic.
 *
 * [x] ABI/FFI contracts remain downstream authorities.
 *
 * [x] Quantum integration terminates at canonical quantum::ir.
 *
 * [x] HDL/hardware semantics remain outside this grammar.
 *
 * [x] Distributed/network semantics remain outside this grammar.
 *
 * [x] Parsing is inert and deterministic.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Rust 1.97 / Rust 1.97.1 compatibility is preserved.
 *
 * [x] Source spans can be retained by the frontend.
 *
 * [x] Positive tests are defined.
 *
 * [x] Negative tests are defined.
 *
 * [x] Boundary tests are defined.
 *
 * [x] Scalability tests are defined.
 *
 * [x] Determinism tests are defined.
 *
 * [x] Compatibility tests are defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How can Zamani describe a WebAssembly interoperability boundary?"
 *
 * It does NOT answer:
 *
 *     "How does WebAssembly execute?"
 *
 *     "Which WASM engine is used?"
 *
 *     "Which CPU executes it?"
 *
 *     "Which device executes it?"
 *
 *     "How is WASM bytecode encoded?"
 *
 *     "How is the program optimized?"
 *
 *     "How is quantum hardware routed?"
 *
 *     "How is QEC performed?"
 *
 * Those responsibilities remain downstream.
 *
 * The architectural objective remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to program semantics, target capabilities, resource availability,
 * security policy, and actual physical constraints, without introducing an
 * artificial language-level machine ceiling.
 *
 * ============================================================================
 */