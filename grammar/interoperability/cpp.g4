/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/cpp.g4
 *
 * Grammar:
 *     CppInterop
 *
 * Status:
 *     CANONICAL C++ INTEROPERABILITY LEAF GRAMMAR
 *
 * Baseline:
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     ANTLR4
 *     safe Rust implementation
 *     no unsafe Rust required
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the Zamani SOURCE-LEVEL CONTRACT for interoperating
 * with externally implemented C++ entities.
 *
 * It does NOT define the C++ programming language.
 *
 * It does NOT parse:
 *
 *     C++ expressions
 *     C++ statements
 *     C++ class bodies
 *     C++ template implementations
 *     C++ preprocessing
 *     C++ modules as a programming language
 *     C++ concepts
 *     C++ overload resolution
 *     C++ template instantiation
 *     C++ object layout
 *     C++ name mangling
 *     C++ ABI implementation
 *     linker behavior
 *     loader behavior
 *     runtime behavior
 *     hardware behavior
 *
 * Those concerns remain downstream semantic/compiler/runtime concerns.
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
 *     canonical parser composition
 *          |
 *          v
 *     CppInterop
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+---------+----------+----------+
 *     |              |          |          |
 *     v              v          v          v
 *   types           ABI       effects   capabilities
 *     |              |          |          |
 *     +--------------+----------+----------+
 *                    |
 *                    v
 *             canonical semantic model
 *                    |
 *                    v
 *              canonical IR boundary
 *                    |
 *          +---------+----------+
 *          |         |          |
 *          v         v          v
 *      classical quantum::ir hardware/HDL
 *                    |
 *                    v
 *             target lowering
 *                    |
 *                    v
 *             ABI/linker/runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - C++ interoperability declarations;
 *     - C++ namespace references;
 *     - C++ function declarations;
 *     - C++ method declarations;
 *     - constructors;
 *     - destructors;
 *     - C++ variable references;
 *     - C++ constant references;
 *     - C++ type references;
 *     - opaque C++ type references;
 *     - enum boundary declarations;
 *     - template boundary declarations;
 *     - callback declarations;
 *     - C++ library/module references;
 *     - C++ symbol aliases;
 *     - C++-specific interoperability metadata;
 *     - C++ interoperability requirements/capabilities.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic FFI;
 *     - generic foreign functions;
 *     - ABI semantics;
 *     - generic calling-convention semantics;
 *     - universal types;
 *     - universal expressions;
 *     - universal names;
 *     - modules;
 *     - effects;
 *     - resources;
 *     - security;
 *     - compiler lowering;
 *     - linker implementation;
 *     - runtime implementation.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * General FFI:
 *
 *     grammar/interoperability/ffi.g4
 *
 * General foreign callable declarations:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * ABI contracts:
 *
 *     grammar/interoperability/abi.g4
 *
 * Calling-convention references:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Canonical names:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * Canonical types:
 *
 *     grammar/types/types.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Interoperability composition:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * This file MUST NOT redefine those systems.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * Canonical parser-facing lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar therefore uses the repository's canonical lexer vocabulary.
 *
 * It does NOT introduce:
 *
 *     CPP_FUNCTION
 *     CPP_CLASS
 *     CPP_NAMESPACE
 *     CPP_TEMPLATE
 *     CPP_ABI
 *     CPP_POINTER
 *     CPP_REFERENCE
 *     CPP_TYPE
 *
 * merely because C++ interoperability exists.
 *
 * C++ implementation names remain symbolic source names.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * C++ interoperability describes a portable external boundary.
 *
 * It MUST NOT encode universal limits such as:
 *
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_REGISTER_COUNT
 *     MAX_DEVICE_COUNT
 *     MAX_TENSOR_RANK
 *
 * It MUST NOT require:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     core0
 *     thread0
 *     register0
 *     memory_bank0
 *
 * as universal language constructs.
 *
 * ABI and target realization are downstream.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     - invoke a C++ compiler;
 *     - execute C++ code;
 *     - load a C++ library;
 *     - inspect headers;
 *     - resolve symbols;
 *     - access the filesystem;
 *     - access the network;
 *     - inspect hardware;
 *     - select a device;
 *     - execute linker commands;
 *     - execute build scripts.
 *
 * This grammar contains no embedded actions or semantic predicates.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No source-level maximum is imposed on:
 *
 *     namespaces
 *     declarations
 *     functions
 *     parameters
 *     callbacks
 *     types
 *     enum members
 *     template parameters
 *     attributes
 *     libraries
 *     requirements
 *     capabilities
 *     nested interoperability declarations
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * Practical resource limits belong to:
 *
 *     compiler resources
 *     runtime resources
 *     target resources
 *     deployment policy
 *
 * They are not grammar limits.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every C++ construct must map into the existing domain-neutral frontend
 * interoperability representation.
 *
 * Conceptual semantic categories include:
 *
 *     ForeignDeclaration
 *     ForeignFunction
 *     ForeignMethod
 *     ForeignConstructor
 *     ForeignDestructor
 *     ForeignType
 *     ForeignOpaqueType
 *     ForeignEnum
 *     ForeignCallback
 *     ForeignSymbol
 *     ForeignLibrary
 *     ForeignMetadata
 *
 * These names describe semantic categories, not permission to create a
 * parallel C++-specific IR.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * C++ interoperability MUST lower through the canonical semantic model.
 *
 * It MUST NOT create:
 *
 *     CppIR
 *     CppQuantumIR
 *     CppHardwareIR
 *
 * If a C++ boundary participates in quantum computation, its semantic effect
 * is lowered through the existing canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 */

parser grammar CppInterop;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Types,
    QualifiedNames,
    Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the entry point consumed by the interoperability composition layer.
 *
 * The complete Zamani program root remains responsible for deciding where
 * C++ interoperability declarations are legal.
 * ============================================================================
 */

cppInterop
    : cppItem*
    ;


/*
 * ============================================================================
 * C++ ITEM DISPATCH
 * ============================================================================
 */

cppItem
    : cppNamespaceDeclaration
    | cppFunctionDeclaration
    | cppMethodDeclaration
    | cppConstructorDeclaration
    | cppDestructorDeclaration
    | cppVariableDeclaration
    | cppConstantDeclaration
    | cppTypeDeclaration
    | cppOpaqueTypeDeclaration
    | cppEnumDeclaration
    | cppTemplateDeclaration
    | cppCallbackDeclaration
    | cppLibraryDeclaration
    ;


/*
 * ============================================================================
 * NAMESPACE
 * ============================================================================
 *
 * Namespace identity is symbolic.
 *
 * The grammar does not enumerate:
 *
 *     std
 *     boost
 *     vendor namespaces
 *     implementation namespaces
 *
 * A future namespace requires no grammar modification.
 * ============================================================================
 */

cppNamespaceDeclaration
    : attribute*
      EXTERN
      STRING
      cppNamespaceKeyword
      qualifiedName
      cppNamespaceBody
    ;

cppNamespaceKeyword
    : identifier
    ;

cppNamespaceBody
    : LBRACE
      cppItem*
      RBRACE
    ;


/*
 * ============================================================================
 * FREE FUNCTION
 * ============================================================================
 *
 * Example semantic form:
 *
 *     extern "C++" fn std::sqrt(value: Real) -> Real;
 *
 * The string identifies the external language/ABI boundary.
 * Its interpretation belongs to ABI semantics.
 * ============================================================================
 */

cppFunctionDeclaration
    : attribute*
      EXTERN
      STRING
      FN
      cppExternalName
      parameterList
      cppReturnClause?
      cppFunctionMetadata*
      SEMICOLON
    ;


/*
 * ============================================================================
 * MEMBER FUNCTION
 * ============================================================================
 *
 * A method is represented separately because object/member dispatch can have
 * ABI implications that differ from a free function.
 * ============================================================================
 */

cppMethodDeclaration
    : attribute*
      EXTERN
      STRING
      cppMethodKeyword
      qualifiedName
      parameterList
      cppReturnClause?
      cppMethodMetadata*
      SEMICOLON
    ;

cppMethodKeyword
    : identifier
    ;


/*
 * ============================================================================
 * CONSTRUCTOR
 * ============================================================================
 */

cppConstructorDeclaration
    : attribute*
      EXTERN
      STRING
      cppConstructorKeyword
      qualifiedName
      parameterList
      cppFunctionMetadata*
      SEMICOLON
    ;

cppConstructorKeyword
    : identifier
    ;


/*
 * ============================================================================
 * DESTRUCTOR
 * ============================================================================
 */

cppDestructorDeclaration
    : attribute*
      EXTERN
      STRING
      cppDestructorKeyword
      qualifiedName
      parameterList?
      cppFunctionMetadata*
      SEMICOLON
    ;

cppDestructorKeyword
    : identifier
    ;


/*
 * ============================================================================
 * VARIABLE
 * ============================================================================
 */

cppVariableDeclaration
    : attribute*
      EXTERN
      STRING
      cppVariableKeyword
      qualifiedName
      COLON
      typeExpression
      cppVariableMetadata*
      SEMICOLON
    ;

cppVariableKeyword
    : identifier
    ;


/*
 * ============================================================================
 * CONSTANT
 * ============================================================================
 */

cppConstantDeclaration
    : attribute*
      EXTERN
      STRING
      CONST
      qualifiedName
      COLON
      typeExpression
      cppVariableMetadata*
      SEMICOLON
    ;


/*
 * ============================================================================
 * TYPE
 * ============================================================================
 *
 * This is a boundary declaration for a type defined externally.
 *
 * The actual C++ representation is resolved semantically.
 * ============================================================================
 */

cppTypeDeclaration
    : attribute*
      EXTERN
      STRING
      TYPE
      qualifiedName
      cppTypeMetadata*
      SEMICOLON
    ;


/*
 * ============================================================================
 * OPAQUE TYPE
 * ============================================================================
 *
 * Opaque types are essential for portable FFI.
 *
 * Zamani can carry an externally managed object without embedding the
 * implementation's physical representation in the language grammar.
 * ============================================================================
 */

cppOpaqueTypeDeclaration
    : attribute*
      EXTERN
      STRING
      cppOpaqueKeyword
      TYPE
      qualifiedName
      cppTypeMetadata*
      SEMICOLON
    ;

cppOpaqueKeyword
    : identifier
    ;


/*
 * ============================================================================
 * ENUM
 * ============================================================================
 */

cppEnumDeclaration
    : attribute*
      EXTERN
      STRING
      cppEnumKeyword
      qualifiedName
      cppEnumUnderlyingType?
      cppEnumBody?
      cppTypeMetadata*
      SEMICOLON
    ;

cppEnumKeyword
    : identifier
    ;

cppEnumUnderlyingType
    : COLON
      typeExpression
    ;

cppEnumBody
    : LBRACE
      cppEnumMember
      (
          COMMA
          cppEnumMember
      )*
      COMMA?
      RBRACE
    ;

cppEnumMember
    : identifier
      cppEnumValue?
    ;

cppEnumValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * TEMPLATE BOUNDARY
 * ============================================================================
 *
 * This grammar describes a template interface.
 *
 * It does NOT parse or instantiate a C++ template implementation.
 *
 * Examples:
 *
 *     template <type T> ...
 *     template <N> ...
 *
 * The meaning of template parameters is validated semantically.
 * ============================================================================
 */

cppTemplateDeclaration
    : attribute*
      EXTERN
      STRING
      cppTemplateKeyword
      cppTemplateParameterList?
      cppTemplateEntity
      cppTemplateMetadata*
      SEMICOLON
    ;

cppTemplateKeyword
    : identifier
    ;

cppTemplateParameterList
    : LT
      cppTemplateParameter
      (
          COMMA
          cppTemplateParameter
      )*
      GT
    ;

cppTemplateParameter
    : identifier
      cppTemplateParameterConstraint?
    ;

cppTemplateParameterConstraint
    : COLON
      typeExpression
    ;

cppTemplateEntity
    : FN
      cppExternalName
      parameterList
      cppReturnClause?
    | TYPE
      qualifiedName
    ;


/*
 * ============================================================================
 * CALLBACK
 * ============================================================================
 *
 * Callback semantics, lifetime, thread-safety, reentrancy and ownership are
 * semantic/runtime concerns.
 * ============================================================================
 */

cppCallbackDeclaration
    : attribute*
      EXTERN
      STRING
      cppCallbackKeyword
      identifier
      parameterList
      cppReturnClause?
      cppCallbackMetadata*
      SEMICOLON
    ;

cppCallbackKeyword
    : identifier
    ;


/*
 * ============================================================================
 * LIBRARY / IMPLEMENTATION REFERENCE
 * ============================================================================
 *
 * A library name is a logical dependency identity.
 *
 * It is NOT a filesystem path.
 * It is NOT a permission to load a library during parsing.
 * ============================================================================
 */

cppLibraryDeclaration
    : attribute*
      EXTERN
      STRING
      cppLibraryKeyword
      cppLibraryIdentity
      cppLibraryMetadata*
      SEMICOLON
    ;

cppLibraryKeyword
    : identifier
    ;

cppLibraryIdentity
    : STRING
    | qualifiedName
    ;


/*
 * ============================================================================
 * SHARED FUNCTION PARAMETERS
 * ============================================================================
 *
 * Parameter syntax consumes the canonical type system.
 *
 * This grammar deliberately does not define:
 *
 *     C++ pointer syntax
 *     C++ reference syntax
 *     C++ type qualifiers
 *     C++ template type syntax
 *
 * Such information is represented by canonical Zamani types plus semantic
 * interoperability metadata.
 * ============================================================================
 */

parameterList
    : LPAREN
      parameterDeclaration*
      RPAREN
    ;

parameterDeclaration
    : identifier
      COLON
      typeExpression
      cppParameterMetadata*
    ;

cppParameterMetadata
    : cppMetadataClause
    ;


/*
 * ============================================================================
 * RETURN TYPE
 * ============================================================================
 */

cppReturnClause
    : THIN_ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * EXTERNAL NAME
 * ============================================================================
 *
 * A C++ source-level name can be a normal Zamani identifier or a qualified
 * external name.
 *
 * An explicit string is available for symbol names that cannot safely be
 * represented as ordinary identifiers.
 * ============================================================================
 */

cppExternalName
    : qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * FUNCTION METADATA
 * ============================================================================
 */

cppFunctionMetadata
    : cppMetadataClause
    ;

cppMethodMetadata
    : cppMetadataClause
    ;

cppVariableMetadata
    : cppMetadataClause
    ;

cppTypeMetadata
    : cppMetadataClause
    ;

cppCallbackMetadata
    : cppMetadataClause
    ;

cppTemplateMetadata
    : cppMetadataClause
    ;

cppLibraryMetadata
    : cppMetadataClause
    ;


/*
 * ============================================================================
 * C++ INTEROPERABILITY METADATA
 * ============================================================================
 *
 * Metadata is intentionally open-world.
 *
 * This prevents the grammar from becoming a closed enumeration of today's
 * C++ ABIs, compilers, platforms and vendor extensions.
 *
 * Examples of semantic keys include:
 *
 *     abi
 *     calling_convention
 *     linkage
 *     symbol
 *     visibility
 *     exceptions
 *     ownership
 *     lifetime
 *     nullable
 *     representation
 *     thread_safety
 *     noexcept
 *     variadic
 *     capability
 *     requires
 *
 * These keys are semantic metadata, not necessarily universal lexer keywords.
 * ============================================================================
 */

cppMetadataClause
    : identifier
      cppMetadataValue?
      SEMICOLON
    ;

cppMetadataValue
    : ASSIGN
      cppMetadataExpression
    ;

cppMetadataExpression
    : expression
    | qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * EXPLICIT C++ ABI METADATA
 * ============================================================================
 *
 * These aliases make the semantic contract clearer while retaining an
 * open-world representation.
 *
 * They do not enumerate ABI families.
 * ============================================================================
 */

cppAbiClause
    : cppKeyAbi
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppCallingConventionClause
    : cppKeyCallingConvention
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppLinkageClause
    : cppKeyLinkage
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppSymbolClause
    : cppKeySymbol
      ASSIGN
      STRING
      SEMICOLON
    ;

cppVisibilityClause
    : cppKeyVisibility
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppExceptionClause
    : cppKeyExceptions
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppOwnershipClause
    : cppKeyOwnership
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppLifetimeClause
    : cppKeyLifetime
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppNullabilityClause
    : cppKeyNullable
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppRepresentationClause
    : cppKeyRepresentation
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;

cppVariadicClause
    : cppKeyVariadic
      ASSIGN
      cppSymbolicValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONTEXTUAL ABI KEYS
 * ============================================================================
 *
 * These are intentionally parsed as identifiers rather than lexer keywords.
 *
 * This means future ABI vocabulary can be added semantically without changing
 * the universal lexical language.
 * ============================================================================
 */

cppKeyAbi
    : identifier
    ;

cppKeyCallingConvention
    : identifier
    ;

cppKeyLinkage
    : identifier
    ;

cppKeySymbol
    : identifier
    ;

cppKeyVisibility
    : identifier
    ;

cppKeyExceptions
    : identifier
    ;

cppKeyOwnership
    : identifier
    ;

cppKeyLifetime
    : identifier
    ;

cppKeyNullable
    : identifier
    ;

cppKeyRepresentation
    : identifier
    ;

cppKeyVariadic
    : identifier
    ;


/*
 * ============================================================================
 * SYMBOLIC VALUES
 * ============================================================================
 */

cppSymbolicValue
    : qualifiedName
    | STRING
    ;


/*
 * ============================================================================
 * INTEROPERABILITY REQUIREMENTS
 * ============================================================================
 *
 * Requirements belong semantically to the resource/capability system.
 *
 * The C++ grammar merely provides an attachment point.
 *
 * A requirement does not select a physical device.
 * ============================================================================
 */

cppRequirementClause
    : REQUIRES
      expression
      SEMICOLON
    ;

cppCapabilityClause
    : CAPABILITY
      qualifiedName
      SEMICOLON
    ;

cppResourceClause
    : RESOURCE
      expression
      SEMICOLON
;


/*
 * ============================================================================
 * C++-SPECIFIC METADATA DISPATCH
 * ============================================================================
 *
 * These rules are deliberately explicit semantic categories, but all consume
 * open-world identifiers rather than a closed C++ vocabulary.
 * ============================================================================
 */

cppBoundaryMetadata
    : cppAbiClause
    | cppCallingConventionClause
    | cppLinkageClause
    | cppSymbolClause
    | cppVisibilityClause
    | cppExceptionClause
    | cppOwnershipClause
    | cppLifetimeClause
    | cppNullabilityClause
    | cppRepresentationClause
    | cppVariadicClause
    | cppRequirementClause
    | cppCapabilityClause
    | cppResourceClause
    | attribute
    | cppMetadataClause
    ;


/*
 * ============================================================================
 * CANONICAL TYPE / EXPRESSION / NAME INTEGRATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * No local identifier rule is created.
 *
 * No local qualified-name rule is created.
 *
 * No local type-expression rule is created.
 *
 * No local expression grammar is created.
 *
 * The imported canonical grammars own these constructs.
 * ============================================================================
 */