/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/foreign-functions.g4
 *
 * Grammar:
 *     InteroperabilityForeignFunctions
 *
 * Status:
 *     Production-ready source-level foreign-function declaration boundary.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     SAFE RUST ONLY
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the INTEROPERABILITY-SPECIFIC SOURCE SYNTAX for describing
 * externally implemented callable declarations.
 *
 * It deliberately does NOT attempt to define:
 *
 *     - a foreign programming language;
 *     - an ABI implementation;
 *     - a linker;
 *     - a loader;
 *     - a runtime;
 *     - a library manager;
 *     - a filesystem resolver;
 *     - a network resolver;
 *     - a hardware selector;
 *     - a quantum backend;
 *     - a QEC implementation;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - physical placement;
 *     - target-specific machine instructions.
 *
 * It defines only the SOURCE-LEVEL CONTRACT describing an externally
 * implemented callable boundary.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                      ZamaniParser
 *                              |
 *                              v
 *          InteroperabilityForeignFunctions
 *                              |
 *                              v
 *                    domain-neutral frontend AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *           types            effects        capabilities
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       classical IR       quantum::ir      HDL/hardware IR
 *                              |
 *                              v
 *                       optimization
 *                              |
 *                              v
 *                    target-independent lowering
 *                              |
 *                              v
 *                     ABI / linker / runtime
 *                              |
 *                              v
 *                       TARGET REALIZATION
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The repository contains two historical foreign-function areas:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/foreign-functions.g4
 *
 * They MUST NOT become competing declarations.
 *
 * The ownership boundary is:
 *
 *     grammar/functions/foreign-functions.g4
 *         generic callable/function declaration syntax
 *
 *     grammar/interoperability/foreign-functions.g4
 *         interoperability-specific external-interface composition
 *
 *     grammar/interoperability/abi.g4
 *         ABI contract syntax
 *
 *     grammar/interoperability/ffi.g4
 *         FFI boundary syntax
 *
 *     grammar/interoperability/interoperability.g4
 *         interoperability composition
 *
 * This file therefore reuses canonical callable/type/name syntax rather than
 * defining a second function language.
 *
 * ============================================================================
 * CRITICAL GRAMMAR-NAME CORRECTION
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/functions/foreign-functions.g4
 *
 * already declares:
 *
 *     parser grammar ForeignFunctions;
 *
 * Therefore this file MUST NOT also declare:
 *
 *     parser grammar ForeignFunctions;
 *
 * because that creates two parser grammars with the same ANTLR grammar name.
 *
 * The filename remains unchanged as required by repository compatibility.
 *
 * The unique grammar identity is:
 *
 *     InteroperabilityForeignFunctions
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars MUST consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * directly.
 *
 * `ZamaniTokens` is an internal lexical composition layer. `ZamaniLexer` is
 * the parser-facing lexer boundary established by the repository architecture.
 *
 * ============================================================================
 * IMPORTANT LEXICAL RULE
 * ============================================================================
 *
 * This file MUST NOT introduce parser-side assumptions for words that are not
 * canonical reserved keywords.
 *
 * In particular, this file does NOT require new lexer keywords for:
 *
 *     foreign
 *     ffi
 *     call
 *     ref
 *     opaque
 *     link
 *     symbol
 *     representation
 *     ABI names
 *     language names
 *     vendor names
 *     backend names
 *
 * Extensible interoperability metadata is represented through canonical
 * identifiers and attributes.
 *
 * This avoids expanding the global keyword set merely because another
 * interoperability provider exists.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A foreign declaration describes a PORTABLE CONTRACT.
 *
 * It does not permanently bind the program to:
 *
 *     CPU model
 *     GPU model
 *     FPGA model
 *     ASIC model
 *     QPU model
 *     physical device
 *     physical address
 *     register number
 *     register width
 *     pointer width
 *     word width
 *     node number
 *     thread count
 *     memory capacity
 *     qubit count
 *     topology
 *     deployment location
 *
 * The same source declaration may therefore be resolved differently on
 * different target systems as long as the semantic contract is satisfied.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is structurally unbounded.
 *
 * There are NO language-level limits on:
 *
 *     interfaces
 *     external functions
 *     parameters
 *     generic parameters
 *     metadata entries
 *     attributes
 *     requirements
 *     capabilities
 *     effects
 *     interfaces
 *     targets
 *     implementations
 *
 * The grammar MUST NOT introduce:
 *
 *     MAX_FOREIGN_FUNCTIONS
 *     MAX_PARAMETERS
 *     MAX_INTERFACES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Operational limits may exist in the parser/compiler/runtime because actual
 * machines have finite resources. Such limits are implementation policy and
 * MUST NOT become Zamani language semantics.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a foreign declaration MUST be completely inert.
 *
 * This grammar MUST NOT:
 *
 *     - open a library;
 *     - open a file;
 *     - access a URL;
 *     - resolve a symbol;
 *     - inspect hardware;
 *     - inspect environment variables;
 *     - execute foreign code;
 *     - create a process;
 *     - create a runtime handle;
 *     - authenticate;
 *     - authorize;
 *     - allocate native memory;
 *     - access physical addresses.
 *
 * All such work belongs downstream and must pass the repository's security,
 * capability, provenance, and runtime policies.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * The surrounding canonical parser composition supplies the shared grammar
 * contracts:
 *
 *     identifier
 *     qualifiedName
 *     stringLiteral
 *     expression
 *     typeExpression
 *     parameterList
 *     attribute
 *     visibilityModifier
 *     functionGenericParameters
 *
 * This file MUST NOT redefine them.
 *
 * Their ownership remains:
 *
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *     grammar/functions/
 *
 * ============================================================================
 */

parser grammar InteroperabilityForeignFunctions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC COMPOSITION ENTRY POINT
 * ============================================================================
 *
 * The interoperability dispatcher imports this rule.
 *
 * The rule represents an external callable declaration boundary.
 *
 * It intentionally does not reuse the generic `foreignFunctionDeclaration`
 * name because that name already exists in the functions grammar.
 */
interoperabilityForeignFunctionDeclaration
    : attribute*
      visibilityModifier?
      EXTERN
      interoperabilityForeignSource?
      interoperabilityForeignMetadata*
      LBRACE
      interoperabilityForeignMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 2. SOURCE IDENTITY
 * ============================================================================
 *
 * The source identity is opaque source metadata.
 *
 * It MUST NOT be interpreted by this grammar as:
 *
 *     filesystem path
 *     shared-library filename
 *     URL
 *     package
 *     vendor
 *     device
 *     backend
 *     architecture
 *     operating system
 *
 * Semantic/linking/deployment layers decide its meaning.
 */
interoperabilityForeignSource
    : stringLiteral
    ;


/*
 * ============================================================================
 * 3. FOREIGN INTERFACE METADATA
 * ============================================================================
 *
 * Metadata is intentionally extensible.
 *
 * Existing canonical interoperability/ABI/FFI contracts can consume these
 * values after semantic validation.
 *
 * This avoids creating a finite grammar vocabulary for every foreign language,
 * ABI, vendor, platform, library format, service protocol, or future system.
 */
interoperabilityForeignMetadata
    : interoperabilityForeignMetadataAttribute
    | interoperabilityForeignMetadataAssignment
    ;


interoperabilityForeignMetadataAttribute
    : attribute
    ;


interoperabilityForeignMetadataAssignment
    : identifier
      ASSIGN
      interoperabilityForeignMetadataValue
      SEMICOLON
    ;


interoperabilityForeignMetadataValue
    : stringLiteral
    | expression
    ;


/*
 * ============================================================================
 * 4. FOREIGN MEMBERS
 * ============================================================================
 *
 * A foreign interface may contain callable declarations and opaque external
 * type/value contracts.
 *
 * The declaration remains part of the canonical AST graph.
 */
interoperabilityForeignMember
    : attribute*
      interoperabilityForeignFunctionMember
    | attribute*
      interoperabilityForeignTypeMember
    | attribute*
      interoperabilityForeignValueMember
    ;


/*
 * ============================================================================
 * 5. FOREIGN FUNCTION MEMBER
 * ============================================================================
 *
 * This is the central callable contract.
 *
 * The grammar deliberately reuses:
 *
 *     functionGenericParameters
 *     parameterList
 *     typeExpression
 *     expression
 *
 * rather than redefining generic parameters, parameters, types, or expressions.
 *
 * A foreign function has NO implementation body.
 */
interoperabilityForeignFunctionMember
    : FN
      identifier
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      interoperabilityForeignReturnClause?
      interoperabilityForeignFunctionContract*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. RETURN TYPE
 * ============================================================================
 */
interoperabilityForeignReturnClause
    : ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * 7. FUNCTION CONTRACTS
 * ============================================================================
 *
 * These are semantic declarations rather than implementation instructions.
 *
 * The actual meaning of each contract is owned by its downstream subsystem.
 */
interoperabilityForeignFunctionContract
    : interoperabilityForeignEffectContract
    | interoperabilityForeignRequirementContract
    | interoperabilityForeignCapabilityContract
    | interoperabilityForeignAttributeContract
    | interoperabilityForeignMetadataAssignment
    ;


/*
 * ============================================================================
 * 8. EFFECT CONTRACT
 * ============================================================================
 *
 * The grammar records effect references.
 *
 * It does not define effect semantics.
 *
 * The canonical effects subsystem remains authoritative.
 */
interoperabilityForeignEffectContract
    : WITH
      EFFECTS
      LBRACE
      qualifiedNameList
      RBRACE
    ;


/*
 * ============================================================================
 * 9. REQUIREMENT CONTRACT
 * ============================================================================
 *
 * Requirements express semantic prerequisites.
 *
 * They are NOT target selections.
 */
interoperabilityForeignRequirementContract
    : REQUIRES
      LBRACE
      interoperabilityForeignRequirement*
      RBRACE
    ;


interoperabilityForeignRequirement
    : expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capabilities are semantic requirements.
 *
 * They do not identify one particular physical machine.
 */
interoperabilityForeignCapabilityContract
    : REQUIRES
      CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. ATTRIBUTE CONTRACT
 * ============================================================================
 *
 * The canonical attribute grammar owns attribute structure.
 *
 * This wrapper exists only to make the foreign-function contract explicit.
 */
interoperabilityForeignAttributeContract
    : attribute
    ;


/*
 * ============================================================================
 * 12. EXTERNAL TYPE MEMBER
 * ============================================================================
 *
 * Foreign types are boundary identities.
 *
 * Their concrete representation is resolved downstream.
 *
 * `opaque` is intentionally NOT a reserved keyword here.
 *
 * Instead, an opaque representation is expressed through metadata/attributes,
 * preventing another global keyword from being introduced solely for FFI.
 */
interoperabilityForeignTypeMember
    : TYPE
      identifier
      interoperabilityForeignTypeParameters?
      interoperabilityForeignTypeRepresentation?
      SEMICOLON
    ;


interoperabilityForeignTypeParameters
    : LESS
      identifier
      (
          COMMA
          identifier
      )*
      COMMA?
      GREATER
    ;


interoperabilityForeignTypeRepresentation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 13. EXTERNAL VALUE MEMBER
 * ============================================================================
 *
 * This represents an externally provided value/constant contract.
 *
 * Actual storage, linkage, symbol resolution, and lifetime are downstream.
 */
interoperabilityForeignValueMember
    : CONST
      identifier
      COLON
      typeExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. EXTERNAL FUNCTION REFERENCE
 * ============================================================================
 *
 * This rule does not execute a call.
 *
 * It provides a reusable source-level reference for composition grammars that
 * need to identify an external callable symbol.
 */
interoperabilityForeignFunctionReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 15. EXTERNAL SYMBOL REFERENCE
 * ============================================================================
 *
 * A symbolic source/name pair may be used where an external implementation
 * identity must be represented syntactically.
 *
 * The string remains opaque metadata.
 */
interoperabilityForeignSymbolReference
    : interoperabilityForeignSource
      DOUBLE_COLON
      qualifiedName
    | qualifiedName
    ;


/*
 * ============================================================================
 * 16. FUNCTION CONTRACT TARGET
 * ============================================================================
 *
 * This is intentionally generic.
 *
 * It does not identify a physical device, library file, ABI implementation,
 * node, process, or machine.
 */
interoperabilityForeignTarget
    : interoperabilityForeignSymbolReference
    ;


/*
 * ============================================================================
 * 17. LINKAGE METADATA
 * ============================================================================
 *
 * Linkage syntax belongs semantically to ABI/linker infrastructure.
 *
 * This grammar merely permits declarative metadata through an identifier key.
 *
 * Examples:
 *
 *     linkage = "..."
 *     symbol = "..."
 *     language = "..."
 *     abi = "..."
 *
 * No fixed vocabulary is imposed here.
 */
interoperabilityForeignLinkageMetadata
    : identifier
      ASSIGN
      stringLiteral
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. LANGUAGE METADATA
 * ============================================================================
 *
 * Language identity is symbolic and extensible.
 *
 * Examples:
 *
 *     language = "C";
 *     language = "C++";
 *     language = "Rust";
 *     language = "Fortran";
 *     language = "Python";
 *     language = "SystemVerilog";
 *     language = "OpenQASM";
 *
 * The grammar does not enumerate these languages.
 */
interoperabilityForeignLanguageMetadata
    : LANGUAGE
      ASSIGN
      stringLiteral
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. COMPATIBILITY METADATA
 * ============================================================================
 *
 * Compatibility remains symbolic.
 *
 * No fixed version-number grammar is imposed here because different foreign
 * ecosystems have different versioning schemes.
 */
interoperabilityForeignCompatibilityMetadata
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. DECLARATION-LEVEL REQUIREMENT
 * ============================================================================
 *
 * This wrapper permits a foreign interface to state requirements without
 * embedding hardware assumptions.
 */
interoperabilityForeignDeclarationRequirement
    : REQUIRES
      LBRACE
      interoperabilityForeignRequirement*
      RBRACE
    ;


/*
 * ============================================================================
 * 21. INTERFACE MEMBER SET
 * ============================================================================
 *
 * Explicitly separated for tooling and validation.
 */
interoperabilityForeignMemberList
    : interoperabilityForeignMember*
    ;


/*
 * ============================================================================
 * 22. FUNCTION CONTRACT LIST
 * ============================================================================
 */
interoperabilityForeignContractList
    : interoperabilityForeignFunctionContract*
    ;


/*
 * ============================================================================
 * 23. DECLARATION SET
 * ============================================================================
 *
 * Useful for parser composition and conformance tooling.
 */
interoperabilityForeignDeclarationSet
    : interoperabilityForeignFunctionDeclaration+
    ;


/*
 * ============================================================================
 * 24. SOURCE-QUALIFIED SYMBOL
 * ============================================================================
 *
 * This is source metadata, not filesystem/network syntax.
 */
interoperabilityForeignQualifiedSymbol
    : interoperabilityForeignSource
      DOUBLE_COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * 25. CALL-SITE INTEGRATION
 * ============================================================================
 *
 * Foreign calls deliberately reuse the canonical expression call system.
 *
 * This file does NOT define:
 *
 *     foreign(...)
 *     ffi call(...)
 *     foreign ref ...
 *
 * because:
 *
 *     grammar/expressions/calls.g4
 *     grammar/interoperability/ffi.g4
 *
 * already own callable invocation composition.
 *
 * A foreign declaration becomes callable through the same canonical call
 * representation used by ordinary Zamani functions.
 *
 * This is essential for:
 *
 *     overload/name resolution
 *     type checking
 *     effect checking
 *     capability checking
 *     ownership analysis
 *     IR generation
 *     optimization
 *     runtime lowering
 *
 * without introducing a second call hierarchy.
 */


/*
 * ============================================================================
 * 26. ABI INTEGRATION
 * ============================================================================
 *
 * ABI-specific syntax belongs to:
 *
 *     grammar/interoperability/abi.g4
 *
 * This file does not redefine:
 *
 *     calling conventions
 *     representations
 *     symbol policies
 *     ABI versions
 *     ABI adapters
 *     ABI marshalling
 *
 * The interoperability composition layer associates this declaration with an
 * ABI contract after parsing.
 *
 * A foreign declaration MAY therefore carry generic metadata such as:
 *
 *     abi = "..."
 *
 * without making that spelling a permanent Zamani keyword.
 */


/*
 * ============================================================================
 * 27. FFI INTEGRATION
 * ============================================================================
 *
 * FFI boundary behavior belongs to:
 *
 *     grammar/interoperability/ffi.g4
 *
 * This file owns the declaration identity.
 *
 * FFI owns additional boundary semantics such as:
 *
 *     binding
 *     callbacks
 *     conversion
 *     ownership transfer
 *     lifetime boundaries
 *     nullability
 *     asynchronous boundary
 *     streaming boundary
 *
 * The two grammars MUST NOT duplicate those constructs.
 */


/*
 * ============================================================================
 * 28. RUST INTEGRATION
 * ============================================================================
 *
 * Rust interoperability remains owned by:
 *
 *     grammar/interoperability/rust.g4
 *
 * This file may declare:
 *
 *     language = "Rust";
 *
 * but it does not implement Rust syntax.
 *
 * The Rust interoperability grammar may consume this declaration as its
 * generic foreign-function boundary.
 *
 * The compiler/runtime implementation MUST remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * No Rust `unsafe` implementation is required or permitted by this grammar
 * contract.
 */


/*
 * ============================================================================
 * 29. C / C++ / PYTHON / OTHER LANGUAGE INTEGRATION
 * ============================================================================
 *
 * Language-specific grammars remain independent.
 *
 * Examples:
 *
 *     grammar/interoperability/c.g4
 *     grammar/interoperability/cpp.g4
 *     grammar/interoperability/python.g4
 *     grammar/interoperability/rust.g4
 *
 * They MUST NOT redefine the generic external declaration contract.
 *
 * They may specialize:
 *
 *     representation
 *     language semantics
 *     ownership model
 *     calling convention metadata
 *     conversion requirements
 *
 * through their own semantic contracts.
 */


/*
 * ============================================================================
 * 30. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A foreign function may represent:
 *
 *     a quantum runtime;
 *     a quantum simulator;
 *     a quantum service;
 *     a quantum accelerator;
 *     a classical/quantum bridge;
 *     a future quantum execution mechanism.
 *
 * This grammar does NOT define:
 *
 *     physical qubit IDs
 *     coupling maps
 *     gate durations
 *     pulse schedules
 *     calibration
 *     QEC
 *     ZQN
 *     physical QPU topology
 *
 * If the foreign function accepts or returns quantum values, those values use
 * the canonical Zamani type system and eventually cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR is introduced.
 */


/*
 * ============================================================================
 * 31. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A foreign function may represent:
 *
 *     an HDL-generated component;
 *     a hardware service;
 *     an accelerator;
 *     a co-design boundary;
 *     a simulator;
 *     a verification interface.
 *
 * The grammar does not encode:
 *
 *     fixed register widths;
 *     fixed port counts;
 *     fixed devices;
 *     physical addresses;
 *     fixed memory capacity;
 *     fixed accelerator counts.
 *
 * Hardware realization belongs downstream.
 */


/*
 * ============================================================================
 * 32. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * An external callable may eventually resolve to a distributed service.
 *
 * This grammar does not define:
 *
 *     network transport;
 *     endpoint discovery;
 *     node selection;
 *     authentication;
 *     retry policy;
 *     placement;
 *     scheduling;
 *     topology.
 *
 * Such behavior belongs to networking, security, resources, execution, and
 * resilience subsystems.
 */


/*
 * ============================================================================
 * 33. AI / DATA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Foreign functions may cross boundaries into:
 *
 *     AI runtimes;
 *     tensor libraries;
 *     data systems;
 *     accelerator runtimes;
 *     scientific libraries;
 *     future computational domains.
 *
 * No framework-specific grammar is introduced here.
 *
 * Framework identity remains symbolic metadata.
 */


/*
 * ============================================================================
 * 34. AST CONTRACT
 * ============================================================================
 *
 * The enclosing:
 *
 *     interoperabilityForeignFunctionDeclaration
 *
 * MUST lower to the repository's existing canonical:
 *
 *     ExternalDeclaration
 *
 * represented by:
 *
 *     src/frontend/ast/node/declarations/extern.rs
 *
 * The existing AST contract intentionally stores:
 *
 *     source: Option<String>
 *     declarations: Vec<NodeId>
 *
 * Therefore:
 *
 *     interoperabilityForeignSource
 *         -> ExternalDeclaration.source
 *
 * and each:
 *
 *     interoperabilityForeignMember
 *
 * becomes a canonical child declaration NodeId.
 *
 * This grammar MUST NOT create:
 *
 *     ForeignFunctionDeclarationNode
 *     ForeignInterfaceNode
 *     ForeignABIImplementationNode
 *     ForeignRuntimeNode
 *
 * as a second AST hierarchy.
 *
 * The semantic layer may maintain richer semantic models after AST validation,
 * but those models do not replace the canonical frontend AST contract.
 */


/*
 * ============================================================================
 * 35. CHILD DECLARATION CONTRACT
 * ============================================================================
 *
 * The child:
 *
 *     interoperabilityForeignFunctionMember
 *
 * represents a declaration-only callable.
 *
 * It must map to the repository's canonical function/declaration representation
 * with external/foreign status supplied through the declaration metadata or
 * semantic modifier model.
 *
 * The child:
 *
 *     interoperabilityForeignTypeMember
 *
 * maps to the canonical type/declaration representation.
 *
 * The child:
 *
 *     interoperabilityForeignValueMember
 *
 * maps to the canonical constant/external-value representation where supported
 * by the frontend AST.
 *
 * If a child construct cannot be represented by the current canonical AST,
 * semantic/AST implementation work is required; the grammar MUST NOT invent a
 * parallel AST solely to make parsing succeed.
 */


/*
 * ============================================================================
 * 36. SOURCE SPANS
 * ============================================================================
 *
 * Every accepted construct must preserve its source span through the parser and
 * AST pipeline.
 *
 * At minimum, spans must cover:
 *
 *     extern
 *     source identity
 *     function name
 *     generic parameters
 *     parameter list
 *     return type
 *     contract clauses
 *     member declarations
 *
 * This is required for:
 *
 *     diagnostics
 *     IDE/LSP
 *     formatter
 *     refactoring
 *     provenance
 *     compatibility tooling
 */


/*
 * ============================================================================
 * 37. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     duplicate declaration detection
 *     type validation
 *     generic validation
 *     parameter validation
 *     return-type validation
 *     effect validation
 *     capability validation
 *     resource validation
 *     ABI validation
 *     FFI validation
 *     language compatibility
 *     representation compatibility
 *     ownership validation
 *     lifetime validation
 *     security validation
 *     provenance validation
 *
 * The parser MUST NOT perform any of these tasks.
 */


/*
 * ============================================================================
 * 38. RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Correct:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("gpu.compute");
 *
 *     requires qubits >= n;
 *
 *     requires memory >= required_memory;
 *
 * Incorrect as universal language contracts:
 *
 *     requires GPU 0;
 *
 *     requires CPU 7;
 *
 *     requires QPU 3;
 *
 *     requires 32 registers;
 *
 *     requires 64 GB;
 *
 *     requires 128 physical qubits;
 *
 * The first group describes semantic requirements.
 *
 * The second group prematurely fixes target realization.
 */


/*
 * ============================================================================
 * 39. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar does not emit IR.
 *
 * The required downstream direction is:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic external-call/declaration model
 *       |
 *       v
 *     canonical IR
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *   classical IR           quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *          optimization/lowering
 *                   |
 *                   v
 *             ABI realization
 *
 * Foreign-function syntax MUST NOT create a second IR.
 */


/*
 * ============================================================================
 * 40. LINKER / LOADER CONTRACT
 * ============================================================================
 *
 * Later compiler/linker/runtime stages may resolve:
 *
 *     symbol
 *     linkage
 *     implementation
 *     ABI
 *     object format
 *     service endpoint
 *     runtime provider
 *
 * according to target policy.
 *
 * None of these operations occur during parsing.
 */


/*
 * ============================================================================
 * 41. DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source
 *     language version
 *     lexer vocabulary
 *     parser grammar
 *
 * the parse structure MUST be deterministic.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     target availability;
 *     filesystem;
 *     network;
 *     environment;
 *     runtime state;
 *     random values;
 *     timestamps.
 */


/*
 * ============================================================================
 * 42. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to parser diagnostics.
 *
 * Semantic errors belong downstream.
 *
 * Examples of semantic errors:
 *
 *     unknown external source;
 *     duplicate external function;
 *     incompatible type;
 *     invalid representation;
 *     unsupported ABI;
 *     unavailable capability;
 *     invalid ownership transfer;
 *     invalid callback;
 *     incompatible language boundary.
 *
 * These MUST NOT be implemented as parser actions or semantic predicates.
 */


/*
 * ============================================================================
 * 43. HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no universal:
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
 *     CPU architecture
 *     GPU architecture
 *     FPGA family
 *     QPU topology
 *     register width
 *     pointer width
 *     word size
 *     memory capacity
 *     device count
 *
 * Repeated source constructs are represented by recursive/unbounded grammar
 * structure.
 */


/*
 * ============================================================================
 * 44. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * The compiler/frontend implementation generated around it MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * and must remain safe Rust.
 *
 * No `unsafe` block, unsafe function, unsafe trait implementation, raw-pointer
 * execution mechanism, or hidden unsafe FFI bridge is required by this grammar.
 *
 * Actual native interoperability must be implemented through an explicitly
 * audited safe abstraction layer or another repository-approved mechanism.
 */


/*
 * ============================================================================
 * 45. TEST CONTRACT
 * ============================================================================
 *
 * Required tests belong under:
 *
 *     grammar/tests/interoperability/
 *
 * --------------------------------------------------------------------------
 * POSITIVE
 * --------------------------------------------------------------------------
 *
 * extern "math" {
 *     fn sin(x: float) -> float;
 * }
 *
 * extern {
 *     fn compute(input: Data) -> Result;
 * }
 *
 * extern "runtime" language = "C" {
 *     fn compute(input: Data) -> Result;
 * }
 *
 * extern "quantum-runtime" {
 *     fn submit(program: QuantumProgram) -> Result
 *         requires capability("quantum.execution");
 * }
 *
 * extern "accelerator" {
 *     fn compute<T: Numeric>(
 *         input: Tensor<T>
 *     ) -> Result<Tensor<T>>;
 * }
 *
 * extern "service" {
 *     fn process(data: Data) -> Result
 *         with effects { io, network };
 * }
 *
 * --------------------------------------------------------------------------
 * NEGATIVE
 * --------------------------------------------------------------------------
 *
 * extern;
 *
 * extern fn;
 *
 * extern "source";
 *
 * extern "source" {
 *     fn broken( -> Result;
 * }
 *
 * extern "source" {
 *     fn broken(x:);
 * }
 *
 * --------------------------------------------------------------------------
 * BOUNDARY
 * --------------------------------------------------------------------------
 *
 * empty external interface
 * one external member
 * many external members
 * many parameters
 * many generic parameters
 * deeply qualified names
 * nested type expressions
 * symbolic resource expressions
 * symbolic capability expressions
 *
 * --------------------------------------------------------------------------
 * CROSS-DOMAIN
 * --------------------------------------------------------------------------
 *
 * classical + foreign
 * quantum + foreign
 * hybrid + foreign
 * HDL + foreign
 * hardware + foreign
 * distributed + foreign
 * AI + foreign
 * data + foreign
 * networking + foreign
 * security + foreign
 *
 * --------------------------------------------------------------------------
 * POCO-REAF
 * --------------------------------------------------------------------------
 *
 * The same source declaration must parse identically regardless of whether
 * the eventual target is:
 *
 *     tiny embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed deployment
 *     future computational architecture
 *
 * Actual target availability is a semantic/runtime concern, not a parsing
 * concern.
 */


/*
 * ============================================================================
 * 46. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source forms must be migrated through the normal compatibility
 * mechanism.
 *
 * In particular:
 *
 *     grammar/functions/foreign-functions.g4
 *
 * remains the legacy/generic function-level foreign declaration surface until
 * the canonical composition layer explicitly delegates compatible constructs
 * here.
 *
 * Existing:
 *
 *     foreignFunctionCall
 *     externDecl
 *
 * forms in older grammar/specification material MUST NOT silently create a
 * second implementation.
 *
 * Their migration status belongs in:
 *
 *     grammar/compatibility/
 *
 * and:
 *
 *     grammar/spec/interoperability.md
 *
 * `grammar/Zamani-Grammar.md` remains historical/design material unless the
 * feature is promoted through the normal specification -> grammar -> AST ->
 * semantic -> IR -> test process.
 */


/*
 * ============================================================================
 * 47. COMPOSITION CONTRACT
 * ============================================================================
 *
 * The interoperability composition grammar:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * MUST import this grammar under the identity:
 *
 *     InteroperabilityForeignFunctions
 *
 * and expose:
 *
 *     interoperabilityForeignFunctionDeclaration
 *
 * through its interoperability dispatcher.
 *
 * The canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * already imports the interoperability composition grammar:
 *
 *     Interoperability
 *
 * Therefore no second root parser is required.
 *
 * The dependency direction is:
 *
 *     InteroperabilityForeignFunctions
 *             |
 *             v
 *     Interoperability
 *             |
 *             v
 *     ZamaniParser
 *
 * NOT:
 *
 *     foreign-functions -> ZamaniParser
 *
 * This prevents circular grammar composition.
 */


/*
 * ============================================================================
 * 48. AST / IR INTEGRATION SUMMARY
 * ============================================================================
 *
 * SOURCE
 *
 *     extern "source" {
 *         fn f(x: T) -> R;
 *     }
 *
 *         |
 *         v
 *
 * PARSE TREE
 *
 *     interoperabilityForeignFunctionDeclaration
 *         |
 *         +-- interoperabilityForeignSource
 *         |
 *         +-- interoperabilityForeignMember
 *                  |
 *                  +-- interoperabilityForeignFunctionMember
 *
 *         |
 *         v
 *
 * FRONTEND AST
 *
 *     ExternalDeclaration
 *         |
 *         +-- canonical function declaration child
 *
 *         |
 *         v
 *
 * SEMANTIC MODEL
 *
 *     external callable contract
 *         |
 *         +-- type contract
 *         +-- effect contract
 *         +-- capability contract
 *         +-- resource contract
 *         +-- ABI/FFI metadata
 *
 *         |
 *         v
 *
 * CANONICAL IR
 *
 *         |
 *         +-- classical call
 *         +-- quantum boundary through quantum::ir
 *         +-- hardware/HDL boundary
 *
 *         |
 *         v
 *
 * TARGET LOWERING
 *
 *         |
 *         +-- ABI
 *         +-- linker
 *         +-- runtime
 *         +-- deployment
 *
 * No parser-level target realization is permitted.
 */


/*
 * ============================================================================
 * 49. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is production-complete only when:
 *
 * [x] Existing filename is retained.
 *
 * [x] ANTLR grammar identity is unique.
 *
 * [x] Canonical ZamaniLexer is the parser-facing vocabulary.
 *
 * [x] No competing lexer is introduced.
 *
 * [x] No duplicate type grammar exists here.
 *
 * [x] No duplicate expression grammar exists here.
 *
 * [x] No duplicate parameter grammar exists here.
 *
 * [x] No duplicate generic-function grammar exists here.
 *
 * [x] No duplicate ABI implementation exists here.
 *
 * [x] No duplicate FFI implementation exists here.
 *
 * [x] No second quantum IR exists here.
 *
 * [x] No hardware realization exists here.
 *
 * [x] No fixed resource limits exist here.
 *
 * [x] No machine-specific capacities exist here.
 *
 * [x] No filesystem/network/runtime access occurs during parsing.
 *
 * [x] External source identity remains opaque.
 *
 * [x] Foreign language identity remains extensible.
 *
 * [x] ABI identity remains extensible.
 *
 * [x] Vendor/backend identity remains extensible.
 *
 * [x] Effects are represented as contracts.
 *
 * [x] Requirements are represented as semantic predicates.
 *
 * [x] Capabilities remain separate from target selection.
 *
 * [x] AST integration is explicitly defined.
 *
 * [x] Canonical IR integration is explicitly defined.
 *
 * [x] quantum::ir remains the canonical quantum boundary.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is specified.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Positive tests are specified.
 *
 * [x] Negative tests are specified.
 *
 * [x] Boundary tests are specified.
 *
 * [x] Scalability tests are specified.
 *
 * [x] Cross-domain tests are specified.
 *
 * [x] POCO-REAF behavior is specified.
 *
 * [x] Compatibility migration is explicitly defined.
 *
 * [x] Composition integration is explicitly defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers exactly one question:
 *
 *     "How does Zamani describe an externally implemented callable boundary?"
 *
 * It does NOT answer:
 *
 *     "Where is that implementation?"
 *     "Which machine executes it?"
 *     "Which ABI implementation is used?"
 *     "Which library is loaded?"
 *     "Which device is selected?"
 *     "Which qubit is used?"
 *     "Which node executes it?"
 *     "How is it routed?"
 *     "How is it scheduled?"
 *     "How is it error-corrected?"
 *
 * Those questions belong downstream.
 *
 * Therefore:
 *
 *     PROGRAM ONCE
 *          |
 *          v
 *     PORTABLE FOREIGN CONTRACT
 *          |
 *          v
 *     CANONICAL AST
 *          |
 *          v
 *     SEMANTIC MODEL
 *          |
 *          v
 *     CANONICAL IR
 *          |
 *          v
 *     TARGET-SPECIFIC REALIZATION
 *
 * The same source contract can consequently participate in implementations
 * ranging from the smallest supported computation to arbitrarily large
 * computations, limited by actual resources rather than artificial language
 * ceilings.
 *
 * ============================================================================
 */