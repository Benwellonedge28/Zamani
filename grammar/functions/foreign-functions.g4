/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/foreign-functions.g4
 *
 * Grammar:
 *     ForeignFunctions
 *
 * Status:
 *     Production parser grammar
 *
 * Purpose:
 *     Define the SOURCE-LEVEL syntax for externally implemented functions
 *     and foreign interfaces.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Zamani:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       +--> functions.g4
 *       |
 *       +--> foreign-functions.g4
 *       |
 *       +--> types/*
 *       |
 *       +--> expressions/*
 *       |
 *       +--> effects/*
 *       |
 *       +--> core/*
 *       |
 *       v
 *     canonical frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> classical IR
 *       |
 *       +--> quantum::ir
 *       |
 *       v
 *     lowering / optimization / routing / scheduling / hardware / runtime
 *
 * This grammar defines syntax only.
 *
 * It MUST NOT:
 *
 *     - execute foreign functions;
 *     - load libraries;
 *     - resolve symbols;
 *     - inspect hardware;
 *     - select a CPU;
 *     - select a GPU;
 *     - select an FPGA;
 *     - select an ASIC;
 *     - select a QPU;
 *     - select a physical qubit;
 *     - select a device;
 *     - select a network node;
 *     - select a topology;
 *     - allocate memory;
 *     - perform ABI lowering;
 *     - perform linking;
 *     - perform dynamic loading;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform optimization;
 *     - implement QEC;
 *     - implement ZQN;
 *     - define quantum IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Foreign declarations describe an INTERFACE, not one implementation.
 *
 * A foreign interface may ultimately be provided by:
 *
 *     - native code;
 *     - another programming language;
 *     - an operating-system service;
 *     - a runtime;
 *     - an accelerator;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum runtime;
 *     - a simulator;
 *     - a distributed service;
 *     - a future execution environment.
 *
 * The grammar therefore avoids hard-coded:
 *
 *     - vendors;
 *     - platforms;
 *     - operating systems;
 *     - library filenames;
 *     - device IDs;
 *     - ABI implementations;
 *     - hardware dimensions;
 *     - resource counts.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - foreign interface declaration syntax;
 *     - foreign function declaration syntax;
 *     - external symbol metadata syntax;
 *     - external language metadata syntax;
 *     - ABI/calling-convention metadata syntax;
 *     - linkage metadata syntax;
 *     - variadic declaration syntax;
 *     - callback/function-pointer interface metadata;
 *     - foreign representation metadata;
 *     - foreign interface attributes;
 *     - foreign requirements/capability attachment boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ordinary functions
 *       -> grammar/functions/functions.g4
 *
 *     - function types
 *       -> grammar/types/function-types.g4
 *
 *     - general types
 *       -> grammar/types/*
 *
 *     - expressions
 *       -> grammar/expressions/*
 *
 *     - effects
 *       -> grammar/effects/*
 *
 *     - capabilities
 *       -> grammar/core/*
 *
 *     - requirements
 *       -> grammar/core/*
 *
 *     - attributes
 *       -> grammar/core/attributes.g4
 *
 *     - modules/packages
 *       -> grammar/modules/*
 *
 *     - memory ownership/lifetimes
 *       -> grammar/memory/*
 *
 *     - linking
 *       -> compiler/linking
 *
 *     - ABI lowering
 *       -> compiler/backend/target layers
 *
 *     - dynamic loading
 *       -> runtime
 *
 *     - hardware discovery
 *       -> hardware HAL
 *
 *     - routing
 *       -> routing
 *
 *     - scheduling
 *       -> scheduling
 *
 *     - optimization
 *       -> optimization
 *
 *     - QEC
 *       -> quantum QEC
 *
 *     - ZQN
 *       -> quantum ZQN
 *
 *     - canonical quantum IR
 *       -> quantum::ir
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is a parser delegate.
 *
 * The complete Zamani parser must provide or compose:
 *
 *     typeExpression
 *     expression
 *     qualifiedName
 *     attribute
 *     effectClause
 *     capabilityRequirement
 *     requirementClause
 *
 * This grammar deliberately does NOT redefine those constructs.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Lexer vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Token vocabulary:
 *
 *     ZamaniTokens
 *
 * This grammar MUST NOT define lexer rules.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level maximum is imposed on:
 *
 *     - interfaces;
 *     - functions;
 *     - parameters;
 *     - generic parameters;
 *     - attributes;
 *     - requirements;
 *     - capabilities;
 *     - interface metadata;
 *     - declarations;
 *     - source size.
 *
 * There is deliberately no:
 *
 *     MAX_FOREIGN_FUNCTIONS
 *     MAX_PARAMETERS
 *     MAX_INTERFACES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_THREADS
 *     MAX_GPUS
 *
 * or equivalent limit.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on source text and the supplied token stream.
 *
 * It does not depend on:
 *
 *     - hardware;
 *     - filesystem state;
 *     - network state;
 *     - environment variables;
 *     - randomness;
 *     - wall-clock time;
 *     - backend availability.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no executable code.
 *
 * Rust implementations integrating the generated parser target:
 *
 *     Rust 1.97 / 1.97.1
 *     Edition 2021
 *
 * and must remain safe Rust.
 *
 * ============================================================================
 */

parser grammar ForeignFunctions;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. FOREIGN INTERFACE
 * ============================================================================
 *
 * Canonical examples:
 *
 *     extern "math" {
 *         fn sin(x: float) -> float;
 *     }
 *
 *     extern {
 *         fn helper(x: int) -> int;
 *     }
 *
 *     extern "runtime.math" @some_attribute {
 *         fn sin(x: float) -> float;
 *     }
 *
 * The source identity is opaque to this grammar.
 * ============================================================================
 */

foreignFunctionDeclaration
    : foreignDeclarationModifier*
      K_EXTERN
      foreignSource?
      foreignDeclarationMetadata*
      LBRACE
      foreignFunctionMember*
      RBRACE
    ;


/* ============================================================================
 * 2. FOREIGN SOURCE IDENTITY
 * ============================================================================
 *
 * The source identity is NOT interpreted here as:
 *
 *     - a path;
 *     - a URL;
 *     - a library filename;
 *     - a package;
 *     - a device;
 *     - a vendor;
 *     - a backend.
 *
 * Those interpretations belong to semantic/linking/deployment layers.
 * ============================================================================
 */

foreignSource
    : STRING_LITERAL
    ;


/* ============================================================================
 * 3. FOREIGN DECLARATION MODIFIERS
 * ============================================================================
 */

foreignDeclarationModifier
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_STATIC
    | K_CONST
    | K_INLINE
    | K_FINAL
    | K_ABSTRACT
    ;


/* ============================================================================
 * 4. FOREIGN DECLARATION METADATA
 * ============================================================================
 *
 * Metadata is intentionally extensible.
 *
 * This avoids adding hard-coded keywords such as:
 *
 *     C
 *     C++
 *     Java
 *     Python
 *     CUDA
 *     OpenCL
 *     AWS
 *     Azure
 *     GCP
 *
 * as grammar-level concepts.
 *
 * The semantic layer may interpret metadata according to a versioned schema.
 * ============================================================================
 */

foreignDeclarationMetadata
    : foreignLanguageClause
    | foreignAbiClause
    | foreignLinkageClause
    | foreignAttribute
    | foreignRequirementAttachment
    | foreignCapabilityAttachment
    ;


/* ============================================================================
 * 5. FOREIGN LANGUAGE
 * ============================================================================
 *
 * The language name is symbolic and extensible.
 *
 * Examples:
 *
 *     language = "C"
 *     language = "C++"
 *     language = "Fortran"
 *     language = "Python"
 *     language = "SystemVerilog"
 *     language = "OpenQASM"
 *
 * No exhaustive list is encoded.
 * ============================================================================
 */

foreignLanguageClause
    : K_LANGUAGE EQUALS STRING_LITERAL
    ;


/* ============================================================================
 * 6. FOREIGN ABI
 * ============================================================================
 *
 * ABI names are symbolic.
 *
 * Examples may include:
 *
 *     c
 *     cdecl
 *     stdcall
 *     system
 *     platform-defined
 *
 * This grammar does not decide whether an ABI exists or is supported.
 * ============================================================================
 */

foreignAbiClause
    : foreignAbiKeyword EQUALS STRING_LITERAL
    ;


foreignAbiKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 7. FOREIGN LINKAGE
 * ============================================================================
 *
 * Linkage metadata identifies how a declaration is externally connected.
 *
 * It does NOT perform linking.
 * ============================================================================
 */

foreignLinkageClause
    : foreignLinkageKeyword EQUALS STRING_LITERAL
    ;


foreignLinkageKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 8. FOREIGN ATTRIBUTES
 * ============================================================================
 *
 * Attribute semantics belong to grammar/core/attributes.g4.
 *
 * This rule is only an integration boundary.
 * ============================================================================
 */

foreignAttribute
    : attribute
    ;


/* ============================================================================
 * 9. FOREIGN REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe what must be available for an interface to be usable.
 *
 * They do not identify one physical machine.
 * ============================================================================
 */

foreignRequirementAttachment
    : requirementClause
    ;


/* ============================================================================
 * 10. FOREIGN CAPABILITIES
 * ============================================================================
 */

foreignCapabilityAttachment
    : capabilityRequirement
    ;


/* ============================================================================
 * 11. FOREIGN MEMBERS
 * ============================================================================
 */

foreignFunctionMember
    : foreignFunction
    ;


/* ============================================================================
 * 12. FOREIGN FUNCTION
 * ============================================================================
 *
 * Foreign functions are declarations only.
 *
 * A foreign function body is intentionally not accepted.
 *
 * Valid:
 *
 *     fn add(a: int, b: int) -> int;
 *
 * Invalid:
 *
 *     fn add(a: int, b: int) -> int {
 *         ...
 *     }
 * ============================================================================
 */

foreignFunction
    : foreignFunctionModifier*
      K_FN
      foreignFunctionName
      foreignGenericParameters?
      LPAREN
      foreignParameterList?
      RPAREN
      foreignReturnClause?
      foreignFunctionMetadata*
      SEMICOLON
    ;


/* ============================================================================
 * 13. FOREIGN FUNCTION MODIFIERS
 * ============================================================================
 */

foreignFunctionModifier
    : K_PUB
    | K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_STATIC
    | K_CONST
    | K_ASYNC
    | K_VOLATILE
    | K_INLINE
    | K_FINAL
    | K_ABSTRACT
    | K_OVERRIDE
    | K_SAFE
    ;


/* ============================================================================
 * 14. FOREIGN FUNCTION NAME
 * ============================================================================
 */

foreignFunctionName
    : IDENTIFIER
    ;


/* ============================================================================
 * 15. FOREIGN GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters remain symbolic.
 *
 * They do not represent:
 *
 *     - number of CPUs;
 *     - number of GPUs;
 *     - number of qubits;
 *     - memory capacity;
 *     - device count.
 * ============================================================================
 */

foreignGenericParameters
    : LESS_THAN
      foreignGenericParameter
      (
          COMMA
          foreignGenericParameter
      )*
      COMMA?
      GREATER_THAN
    ;


foreignGenericParameter
    : IDENTIFIER
      foreignGenericParameterBound?
    ;


foreignGenericParameterBound
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 16. FOREIGN PARAMETERS
 * ============================================================================
 */

foreignParameterList
    : foreignParameter
      (
          COMMA
          foreignParameter
      )*
      COMMA?
    ;


foreignParameter
    : foreignParameterModifier*
      foreignParameterName
      (
          COLON
          typeExpression
      )?
      foreignParameterDefault?
    ;


foreignParameterModifier
    : K_MUT
    | K_CONST
    | K_VOLATILE
    ;


foreignParameterName
    : IDENTIFIER
    ;


foreignParameterDefault
    : EQUALS
      expression
    ;


/* ============================================================================
 * 17. FOREIGN RETURN TYPE
 * ============================================================================
 */

foreignReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 18. FOREIGN FUNCTION METADATA
 * ============================================================================
 *
 * These metadata categories are deliberately open-ended.
 * ============================================================================
 */

foreignFunctionMetadata
    : foreignSymbolClause
    | foreignLanguageClause
    | foreignAbiClause
    | foreignLinkageClause
    | foreignRepresentationClause
    | foreignVariadicClause
    | foreignCallbackClause
    | foreignEffectAttachment
    | foreignRequirementAttachment
    | foreignCapabilityAttachment
    | foreignAttribute
    ;


/* ============================================================================
 * 19. FOREIGN SYMBOL
 * ============================================================================
 *
 * Separates the Zamani declaration name from the externally visible symbol.
 *
 * Example:
 *
 *     fn add_numbers(a: int, b: int)
 *         symbol = "external_add";
 *
 * This is metadata only.
 * ============================================================================
 */

foreignSymbolClause
    : foreignSymbolKeyword EQUALS STRING_LITERAL
    ;


foreignSymbolKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 20. FOREIGN REPRESENTATION
 * ============================================================================
 *
 * A source type and its external representation are distinct concepts.
 *
 * Examples:
 *
 *     representation = "opaque"
 *     representation = "pointer"
 *     representation = "struct"
 *     representation = "vector"
 *
 * The representation name is intentionally symbolic.
 * ============================================================================
 */

foreignRepresentationClause
    : foreignRepresentationKeyword EQUALS STRING_LITERAL
    ;


foreignRepresentationKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 21. VARIADIC INTERFACE
 * ============================================================================
 *
 * The grammar permits declaration of variadic foreign functions without
 * imposing an argument-count limit.
 *
 * Semantic analysis determines whether the selected interface/ABI permits it.
 * ============================================================================
 */

foreignVariadicClause
    : foreignVariadicKeyword
    ;


foreignVariadicKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 22. CALLBACK INTERFACE
 * ============================================================================
 *
 * Callback metadata remains symbolic because complete function-type semantics
 * belong to grammar/types/function-types.g4.
 *
 * The foreign layer does not create a second function-type grammar.
 * ============================================================================
 */

foreignCallbackClause
    : foreignCallbackKeyword
      EQUALS
      typeExpression
    ;


foreignCallbackKeyword
    : IDENTIFIER
    ;


/* ============================================================================
 * 23. EFFECT ATTACHMENT
 * ============================================================================
 *
 * Effects are owned by grammar/effects/*.
 * ============================================================================
 */

foreignEffectAttachment
    : effectClause
    ;


/* ============================================================================
 * 24. GENERIC FOREIGN CALL TARGET
 * ============================================================================
 *
 * A foreign declaration and a foreign call are deliberately separated.
 *
 * The declaration grammar owns the interface.
 *
 * Expression/call grammar owns invocation syntax.
 *
 * This rule exists only as a reusable name boundary for parser composition.
 * ============================================================================
 */

foreignQualifiedSymbol
    : qualifiedName
    ;


/* ============================================================================
 * 25. OPTIONAL EXPLICIT EXTERNAL SYMBOL
 * ============================================================================
 *
 * A qualified external identity may be expressed symbolically.
 *
 * Example:
 *
 *     namespace::symbol
 *
 * The grammar does not interpret the namespace as a library, vendor, device,
 * platform, or filesystem path.
 * ============================================================================
 */

foreignQualifiedName
    : qualifiedName
    ;


/* ============================================================================
 * 26. FOREIGN INTERFACE GROUP
 * ============================================================================
 *
 * Reusable wrapper for parser composition.
 * ============================================================================
 */

foreignInterface
    : K_EXTERN
      foreignSource?
      foreignDeclarationMetadata*
      LBRACE
      foreignFunctionMember*
      RBRACE
    ;


/* ============================================================================
 * 27. DECLARATION-ONLY FOREIGN FUNCTION
 * ============================================================================
 *
 * Useful when another grammar already supplies the `extern` boundary.
 * ============================================================================
 */

foreignFunctionPrototype
    : foreignFunction
    ;


/* ============================================================================
 * 28. SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser MUST NOT determine:
 *
 *     - whether a library exists;
 *     - whether a symbol exists;
 *     - whether an ABI is supported;
 *     - whether a language implementation exists;
 *     - whether a representation is compatible;
 *     - whether ownership is safe;
 *     - whether a capability exists;
 *     - whether a resource requirement can be satisfied;
 *     - whether a target supports the interface;
 *     - whether external code is trusted;
 *     - whether a dynamic loader may be used.
 *
 * These are semantic/compiler/runtime questions.
 * ============================================================================
 */


/* ============================================================================
 * 29. HARD-CODING PROHIBITIONS
 * ============================================================================
 *
 * This grammar MUST NOT grow exhaustive keyword lists for:
 *
 *     vendors
 *     cloud providers
 *     operating systems
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     quantum providers
 *     network providers
 *     library names
 *     device models
 *     ABI implementations
 *     deployment environments
 *
 * Such values are represented as symbolic metadata and interpreted downstream.
 * ============================================================================
 */


/* ============================================================================
 * 30. INTEROPERABILITY BOUNDARY
 * ============================================================================
 *
 * Conceptually:
 *
 *     Foreign source declaration
 *             |
 *             v
 *     parsed foreign interface
 *             |
 *             v
 *     semantic validation
 *             |
 *             +--> type compatibility
 *             +--> effect compatibility
 *             +--> capability requirements
 *             +--> resource requirements
 *             +--> ABI compatibility
 *             +--> linkage resolution
 *             |
 *             v
 *     canonical semantic model
 *             |
 *             +--> classical IR
 *             |
 *             +--> quantum::ir
 *             |
 *             v
 *     target-specific lowering
 *
 * Foreign grammar MUST NOT bypass the canonical semantic/IR boundary.
 * ============================================================================
 */


/* ============================================================================
 * 31. QUANTUM INTEROPERABILITY
 * ============================================================================
 *
 * A foreign interface may eventually expose quantum functionality.
 *
 * This grammar does NOT create quantum-specific foreign types.
 *
 * For example, it must NOT define:
 *
 *     QPU_PTR
 *     PHYSICAL_QUBIT
 *     IBM_QUBIT
 *     CUDA_QUBIT
 *     QIR_QUBIT
 *
 * Instead:
 *
 *     foreign declarations
 *         |
 *         v
 *     canonical Zamani type semantics
 *         |
 *         v
 *     quantum semantic analysis
 *         |
 *         v
 *     quantum::ir
 *
 * Hardware realization remains downstream.
 * ============================================================================
 */


/* ============================================================================
 * 32. MEMORY / OWNERSHIP INTEROPERABILITY
 * ============================================================================
 *
 * This grammar does not create a second ownership system.
 *
 * Ownership, borrowing, lifetime, transfer, retain/release, allocation and
 * deallocation semantics belong to the memory/type/semantic subsystems.
 *
 * Foreign declarations may carry metadata through the generic attribute and
 * requirement mechanisms.
 * ============================================================================
 */


/* ============================================================================
 * 33. SECURITY
 * ============================================================================
 *
 * Foreign declarations are trust boundaries.
 *
 * Semantic/compiler layers may use the parsed declaration to enforce:
 *
 *     - trust policy;
 *     - capability policy;
 *     - effect policy;
 *     - sandbox policy;
 *     - signing policy;
 *     - provenance;
 *     - ABI safety;
 *     - deployment policy.
 *
 * None of these policies are executed by this grammar.
 * ============================================================================
 */


/* ============================================================================
 * 34. COMPATIBILITY
 * ============================================================================
 *
 * Legacy conceptual form:
 *
 *     extern "math" {
 *         fn add(a: int, b: int) -> int;
 *     }
 *
 * remains representable.
 *
 * The grammar intentionally does not require the foreign source to be a
 * particular library or platform.
 * ============================================================================
 */


/* ============================================================================
 * 35. PRODUCTION INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 *     No physical-machine limits are encoded.
 *
 * INVARIANT 2
 *     No vendor/platform enumeration is encoded.
 *
 * INVARIANT 3
 *     No second type system is created.
 *
 * INVARIANT 4
 *     No second expression grammar is created.
 *
 * INVARIANT 5
 *     No second function-type grammar is created.
 *
 * INVARIANT 6
 *     No ABI implementation is performed.
 *
 * INVARIANT 7
 *     No linker/runtime behavior is performed.
 *
 * INVARIANT 8
 *     Foreign declarations remain semantic interfaces.
 *
 * INVARIANT 9
 *     Quantum interfaces must ultimately enter canonical quantum semantics.
 *
 * INVARIANT 10
 *     Rust integration remains compatible with Rust 1.97 / 1.97.1 and safe
 *     Rust requirements.
 *
 * ============================================================================
 */